mod renderer;
mod state;

use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes},
};

use crate::app::{renderer::Renderer, state::State};

#[derive(Debug)]
pub(crate) struct App {
    // TODO: add State
    window: Option<Window>,
    renderer: Option<Renderer<'static>>,
    state: State,
}
impl App {
    pub fn run() -> anyhow::Result<()> {
        // TODO: consider adding user events (custom Event enum), then EventLoop::with_user_event().build()?
        let event_loop = EventLoop::new()?;
        let mut app = Self {
            window: None,
            renderer: None,
            state: State::default(),
        };
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        event_loop.run_app(&mut app)?;
        Ok(())
    }

    // TODO: Separate module for key handler and it's methods?
    fn handle_key_event(
        &self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        key_event: KeyEvent,
    ) {
        let KeyEvent {
            physical_key: PhysicalKey::Code(key_code),
            state,
            ..
        } = key_event
        else {
            return;
        };

        log::debug!("KeyboardInput: {key_code:?} {state:?}");

        match (key_code, state) {
            (KeyCode::KeyT, ElementState::Pressed) => self.toggle_control_flow(event_loop),
            (KeyCode::Escape, ElementState::Pressed) | (KeyCode::KeyQ, ElementState::Pressed) => {
                self.exit(event_loop)
            }
            (KeyCode::KeyR, ElementState::Pressed) => self.request_redraw(),
            _ => {}
        }
    }

    fn toggle_control_flow(&self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let previous_flow = event_loop.control_flow();
        let new_flow = match previous_flow {
            ControlFlow::Poll => ControlFlow::Wait,
            ControlFlow::Wait => ControlFlow::Poll,
            _ => ControlFlow::Wait,
        };

        event_loop.set_control_flow(new_flow);
        log::info!("Control flow changed: {previous_flow:?} -> {new_flow:?}");
    }

    fn exit(&self, event_loop: &winit::event_loop::ActiveEventLoop) {
        log::info!("Exiting application");
        event_loop.exit();
    }

    fn request_redraw(&self) {
        log::info!("Manual redraw request");
        match &self.window {
            Some(window) => {
                window.request_redraw();
            }
            None => {
                log::error!("Cannot redraw: no window available");
            }
        }
    }
}
impl winit::application::ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // TODO: configure the Window with WindowAttributes
        let window_attributes = WindowAttributes::default().with_title("unnamed-engine");
        match event_loop.create_window(window_attributes) {
            Ok(window) => {
                log::info!("Window created successfully");
                self.window = Some(window);
                match Renderer::new(self.window.as_ref().unwrap()) {
                    Ok(renderer) => {
                        self.renderer = Some(renderer);
                        if let Err(err) = self.renderer.as_ref().unwrap().render() {
                            log::error!("Failed to render: {err}")
                        };
                    }
                    Err(err) => {
                        log::error!("Renderer not initialised: {err}");
                    }
                }
            }
            Err(err) => {
                log::error!("Failed to create window: {err}");
            }
        };
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => match self.renderer.as_ref() {
                Some(renderer) => {
                    if let Err(err) = renderer.render() {
                        log::error!("Cannot render: {err}");
                    };
                }
                None => log::warn!("Cannot render: no renderer available"),
            },
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
            WindowEvent::CursorMoved { position, .. } => self.state.cursor_position = position,
            WindowEvent::KeyboardInput { event, .. } => {
                self.handle_key_event(event_loop, event);
            }
            _ => (),
        }
    }
}
