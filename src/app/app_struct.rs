use std::{sync::Arc, time::Instant};

use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

use crate::{
    app::{State, events},
    graphics::{
        GraphicsContext, RenderObject, Transform, primitives,
        uniforms::{SurfaceSizeUniform, TimeUniform, UniformKind},
    },
};

const WINDOW_TITLE: &str = "unnamed-engine";

pub struct App {
    window: Option<Arc<Window>>,
    graphics_context: Option<GraphicsContext>,
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
            graphics_context: None,
            state: State {
                render_objects: vec![RenderObject::new(
                    primitives::regular_polygon(3, 0.7, wgpu::Color::BLACK),
                    Some("The Square"),
                    // TODO: send transform to gpu?
                    Transform::builder().position(-0.5, -0.5).build(), // currently does NOTHING
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
                .with_resizable(true),
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

        if self.graphics_context.is_none() {
            match GraphicsContext::setup(&window, &mut self.state) {
                Ok(graphics_context) => self.graphics_context = Some(graphics_context),
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

                if state == ElementState::Pressed {
                    let surface_size = self.window.as_ref().unwrap().inner_size();
                    self.state.add_object(RenderObject::new(
                        primitives::triangle(0.1, wgpu::Color::BLACK),
                        Some("TestTriangle"),
                        Transform::builder()
                            .physical_position(self.state.cursor_position, surface_size)
                            // .position(x, y)
                            .build(),
                    ));
                }
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

                let graphics_context = self.graphics_context.as_mut().unwrap();
                graphics_context.update_uniform(UniformKind::Time(TimeUniform::new(
                    self.state.timer.elapsed().as_secs_f32(),
                )));
                if let Err(err) = graphics_context.render(&mut self.state) {
                    log::error!("Unable to render: {err}");
                }

                if self.rendering_active {
                    window.request_redraw()
                };
            }
            WindowEvent::Resized(size) => {
                let graphics_context = self.graphics_context.as_mut().unwrap();
                if log::log_enabled!(log::Level::Debug) {
                    let current_surface_size = graphics_context.get_surface_size();
                    log::debug!(
                        "Resizing window. Current surface size: {{w: {}, h: {}}}, requested size: {{w: {}, h: {}}}",
                        current_surface_size.width,
                        current_surface_size.height,
                        size.width,
                        size.height
                    );
                }
                match graphics_context.resize_surface(size.width, size.height) {
                    Ok(_) => {
                        self.rendering_active = true;

                        graphics_context.update_uniform(UniformKind::Surface(
                            SurfaceSizeUniform::new(size.width as f32, size.height as f32),
                        ));
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
