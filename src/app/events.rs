use std::sync::Arc;

use winit::{
    event::{ElementState, KeyEvent},
    event_loop::{ActiveEventLoop, ControlFlow},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

pub fn handle_key_event(key_event: KeyEvent, event_loop: &ActiveEventLoop, window: &Arc<Window>) {
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
        (KeyCode::KeyR, ElementState::Pressed) => request_redraw(window),
        (KeyCode::KeyT, ElementState::Pressed) => toggle_control_flow(event_loop),
        (KeyCode::Escape, ElementState::Pressed) | (KeyCode::KeyQ, ElementState::Pressed) => {
            exit(event_loop)
        }
        _ => {}
    }
}

pub fn exit(event_loop: &ActiveEventLoop) {
    event_loop.exit();
}

fn toggle_control_flow(event_loop: &ActiveEventLoop) {
    let previous_flow = event_loop.control_flow();
    let new_flow = match previous_flow {
        ControlFlow::Poll => ControlFlow::Wait,
        ControlFlow::Wait => ControlFlow::Poll,
        _ => ControlFlow::Wait,
    };

    event_loop.set_control_flow(new_flow);
    log::info!("Control flow changed: {previous_flow:?} -> {new_flow:?}");
}

fn request_redraw(window: &Arc<Window>) {
    log::info!("Manual redraw requested");
    window.request_redraw();
}
