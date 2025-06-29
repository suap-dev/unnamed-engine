use winit::{
    event::{KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes},
};

#[derive(Debug)]
pub(crate) struct App {
    // TODO: add State
    window: Option<Window>,
}
impl App {
    pub fn run() -> anyhow::Result<()> {
        // TODO: consider adding user events (custom Event enum), then EventLoop::with_user_event().build()?
        let event_loop = EventLoop::new()?;
        let mut app = Self { window: None };
        event_loop.run_app(&mut app)?;
        Ok(())
    }
}
impl winit::application::ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // TODO: configure the Window with WindowAttributes
        let window_attributes = WindowAttributes::default().with_title("unnamed-engine");
        self.window = event_loop.create_window(window_attributes).ok();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {}
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key_code),
                        state: element_state,
                        ..
                    },
                ..
            } => {
                // TODO: create a handler for key events if we get more keys
                log::debug!("{:?} {:?}", key_code, element_state);
                if element_state.is_pressed() {
                    match key_code {
                        KeyCode::Escape | KeyCode::KeyQ => {
                            log::info!("Exitting {:?}", &self);
                            event_loop.exit();
                        }
                        KeyCode::KeyR => {
                            log::info!("Manual redraw request of window: {:?}", &self.window);
                            if let Some(window) = &self.window {
                                window.request_redraw();
                                log::info!("Redrawn window: {:?}", window);
                            } else {
                                log::error!("Couldn't redraw window: {:?}", &self.window);
                            }
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
}
