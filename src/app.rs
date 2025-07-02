use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalPosition,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

use crate::{graphics, user_events};

const WINDOW_TITLE: &str = "unnamed-engine";

#[derive(Default)]
pub struct App {
    surface: Option<wgpu::Surface<'static>>,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    window: Option<Arc<Window>>,
    cursor_position: PhysicalPosition<f64>,
    clear_color: wgpu::Color,
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
    Ok(Arc::new(event_loop.create_window(
        WindowAttributes::default().with_title(WINDOW_TITLE),
    )?))
}
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = create_window(event_loop).unwrap();
        let (surface, device, queue) = graphics::setup(&window).unwrap();

        window.request_redraw();

        self.window = Some(window);
        self.surface = Some(surface);
        self.device = Some(device);
        self.queue = Some(queue);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
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
                user_events::handle_key_event(key_event, event_loop, self.window.as_ref().unwrap());
            }
            WindowEvent::RedrawRequested => {
                // TODO: delete; funzies
                {
                    let (width, height) = {
                        let surface_size = self.window.as_ref().unwrap().inner_size();
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

                graphics::render(
                    self.surface.as_ref().unwrap(),
                    self.device.as_ref().unwrap(),
                    self.queue.as_ref().unwrap(),
                    self.clear_color,
                )
                .unwrap();
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
