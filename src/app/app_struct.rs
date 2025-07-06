use std::{sync::Arc, time::Instant};

use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalPosition, PhysicalSize},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

use crate::{
    app::{State, events},
    graphics::{GraphicsContext, RenderObject, primitives},
};

const WINDOW_TITLE: &str = "unnamed-engine";

pub struct App {
    window: Option<Arc<Window>>,
    wgpu_context: Option<GraphicsContext>,
    state: State,
    rendering_active: bool,
}
impl App {
    pub fn run() -> anyhow::Result<()> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut Self::new())?;

        Ok(())
    }

    fn new() -> Self {
        Self {
            window: None,
            wgpu_context: None,
            state: State {
                render_objects: vec![RenderObject::new(
                    &primitives::regular_polygon(4, 0.7, wgpu::Color::BLACK),
                    Some("The Square"),
                )],
                cursor_position: PhysicalPosition::default(),
                clear_color: wgpu::Color {
                    g: 0.25,
                    r: 0.25,
                    b: 0.25,
                    a: 1.0,
                },
                timer: Instant::now(),
            },
            rendering_active: false,
        }
    }
}
fn create_window(event_loop: &ActiveEventLoop) -> anyhow::Result<Arc<Window>> {
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

        let window = match create_window(event_loop) {
            Ok(window) => window,
            Err(err) => {
                log::error!("Unable to create window: {err}");
                return;
            }
        };

        if self.wgpu_context.is_none() {
            match GraphicsContext::setup(&window, &mut self.state) {
                Ok(wgpu_context) => self.wgpu_context = Some(wgpu_context),
                Err(err) => log::error!("Unable to set up graphics: {err}"),
            }
        }

        window.request_redraw();
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => events::exit(event_loop),
            WindowEvent::CursorMoved { position, .. } => self.state.cursor_position = position,
            WindowEvent::MouseInput { state, button, .. } => {
                log::debug!(
                    "MouseInput: {:?} {:?} at {:?}",
                    button,
                    state,
                    self.state.cursor_position
                );
            }
            WindowEvent::MouseWheel { delta, phase, .. } => {
                log::debug!(
                    "MouseWheel: {:?} {:?} at {:?}",
                    delta,
                    phase,
                    self.state.cursor_position
                );
            }
            WindowEvent::KeyboardInput {
                event: key_event, ..
            } => {
                let window = self.window.as_ref().unwrap();
                events::handle_key_event(key_event, event_loop, window);
            }
            WindowEvent::RedrawRequested => {
                let window = self.window.as_ref().unwrap();
                let surface_size = window.inner_size();

                let wgpu_context = self.wgpu_context.as_mut().unwrap();
                wgpu_context.update_uniforms(surface_size, self.state.timer.elapsed());
                if let Err(err) = wgpu_context.render(&self.state) {
                    log::error!("Unable to render: {err}");
                }

                if self.rendering_active {
                    window.request_redraw()
                };
            }
            WindowEvent::Resized(size) => {
                let wgpu_context = self.wgpu_context.as_mut().unwrap();
                if log::log_enabled!(log::Level::Debug) {
                    let current_surface_size = wgpu_context.get_surface_size();
                    log::debug!(
                        "Resizing window. Current surface size: {{w: {}, h: {}}}, requested size: {{w: {}, h: {}}}",
                        current_surface_size.width,
                        current_surface_size.height,
                        size.width,
                        size.height
                    );
                }
                match wgpu_context.resize_surface(size.width, size.height) {
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
