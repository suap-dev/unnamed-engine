use std::{sync::Arc, time::Instant};

use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalPosition, PhysicalSize},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

use crate::{
    graphics::{WgpuContext, uniforms},
    user_events,
};

const WINDOW_TITLE: &str = "unnamed-engine";

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    wgpu_context: WgpuContext,
    cursor_position: PhysicalPosition<f64>,
    clear_color: wgpu::Color,
    rendering_active: bool,
    timer: Option<Instant>,
}
impl App {
    pub fn run() -> anyhow::Result<()> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut Self::default())?;

        Ok(())
    }
}
fn create_window(event_loop: &winit::event_loop::ActiveEventLoop) -> anyhow::Result<Arc<Window>> {
    Ok(Arc::new(
        event_loop.create_window(
            WindowAttributes::default()
                .with_title(WINDOW_TITLE)
                .with_inner_size(PhysicalSize {
                    width: 1280,
                    height: 720,
                })
                .with_resizable(false),
        )?,
    ))
}
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        log::debug!("Application resumed");
        if self.timer.is_none() {
            self.timer = Some(Instant::now());
        }

        let window = match create_window(event_loop) {
            Ok(window) => window,
            Err(err) => {
                log::error!("Unable to create window: {err}");
                return;
            }
        };

        if let Err(err) = self.wgpu_context.setup(&window) {
            log::error!("Unable to set up graphics: {err}");
            return;
        };

        window.request_redraw();
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let window = self.window.as_ref().unwrap();
        match event {
            WindowEvent::CloseRequested => user_events::exit(event_loop),
            WindowEvent::CursorMoved { position, .. } => self.cursor_position = position,
            WindowEvent::MouseInput { state, button, .. } => {
                log::debug!(
                    "MouseInput: {:?} {:?} at {:?}",
                    button,
                    state,
                    self.cursor_position
                );
            }
            WindowEvent::MouseWheel { delta, phase, .. } => {
                log::debug!(
                    "MouseWheel: {:?} {:?} at {:?}",
                    delta,
                    phase,
                    self.cursor_position
                );
            }
            WindowEvent::KeyboardInput {
                event: key_event, ..
            } => {
                user_events::handle_key_event(key_event, event_loop, window);
            }
            WindowEvent::RedrawRequested => {
                // TODO: delete; funzies
                {
                    let (width, height) = {
                        let surface_size = window.inner_size();
                        (surface_size.width as f64, surface_size.height as f64)
                    };
                    let (x, y) = { (self.cursor_position.x, self.cursor_position.y) };

                    let red = x / width;
                    let green = (1.0 - (x / width + y / height)).clamp(0.0, 1.0);
                    let blue = y / height;

                    self.clear_color.r = red;
                    self.clear_color.g = green;
                    self.clear_color.b = blue;
                }

                self.wgpu_context
                    .update_stuff_uniform(&uniforms::Stuff::new(
                        window.inner_size(),
                        self.timer.as_ref().unwrap().elapsed().as_secs_f32(),
                    ));
                if let Err(err) = self.wgpu_context.render(self.clear_color) {
                    log::error!("Unable to render: {err}");
                }
                if self.rendering_active {
                    window.request_redraw()
                };
            }
            WindowEvent::Resized(size) => {
                if log::log_enabled!(log::Level::Debug) {
                    let current_surface_size = self.wgpu_context.get_surface_size();
                    log::debug!(
                        "Resizing window. Current surface size: {{w: {}, h: {}}}, requested size: {{w: {}, h: {}}}",
                        current_surface_size.width,
                        current_surface_size.height,
                        size.width,
                        size.height
                    );
                }
                match self.wgpu_context.resize_surface(size.width, size.height) {
                    Ok(_) => {
                        self.rendering_active = true;
                        log::debug!("Window resized");
                    }
                    Err(err) => {
                        // TODO: be aware that rendering stops when we minimise the window
                        self.rendering_active = false;
                        log::warn!("{err}");
                    }
                };
            }
            _ => {}
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        log::debug!("Application suspended");
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        log::debug!("Application exiting");
    }
}
