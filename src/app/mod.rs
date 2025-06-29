use winit::{
    event::{KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes},
};

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
        match event_loop.create_window(WindowAttributes::default()) {
            Ok(window) => {
                // Store the window if needed
                self.window = Some(window);
            }
            Err(e) => {
                eprintln!("Failed to create window: {:?}", e);
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key_code),
                        state: element_state,
                        ..
                    },
                ..
            } => {
                // TODO: create a handler for this if we get more keys
                if key_code == KeyCode::Escape && element_state.is_pressed() {
                    event_loop.exit();
                }
            }
            _ => (),
        }
    }
}
