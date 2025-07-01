use wgpu::{
    CommandEncoderDescriptor, Device, Instance, InstanceDescriptor, Queue, RenderPassDescriptor,
    Surface, TextureViewDescriptor,
};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalPosition,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::WindowAttributes,
};

mod app;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    // EVENT LOOP
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    event_loop.run_app(&mut App::default())?;

    Ok(())
}

#[derive(Default)]
struct App {
    surface: Option<Surface<'static>>,
    device: Option<Device>,
    queue: Option<Queue>,
    cursor_position: PhysicalPosition<f64>,
}
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // WINDOW
        let window_attributes = WindowAttributes::default().with_title("unnamed-engine");
        let window = event_loop.create_window(window_attributes).unwrap();

        // SURFACE
        let instance = Instance::new(&InstanceDescriptor::default());
        let window_inner_size = window.inner_size();
        let surface = instance.create_surface(window).unwrap();

        // ADAPTER
        let request_adapter_options = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        };
        let adapter =
            pollster::block_on(instance.request_adapter(&request_adapter_options)).unwrap();

        // DEVICE & QUEUE
        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::wgt::DeviceDescriptor::default()))
                .unwrap();

        // SURFACE.CONFIGURE
        let surface_capabilities = surface.get_capabilities(&adapter);
        surface.configure(
            &device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: *surface_capabilities
                    .formats
                    .iter()
                    .find(|format| format.is_srgb())
                    .unwrap(),
                width: window_inner_size.width,
                height: window_inner_size.height,
                present_mode: surface_capabilities.present_modes[0],
                desired_maximum_frame_latency: 2,
                alpha_mode: surface_capabilities.alpha_modes[0],
                view_formats: vec![],
            },
        );

        self.surface = Some(surface);
        self.device = Some(device);
        self.queue = Some(queue);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                render(
                    self.surface.as_ref().unwrap(),
                    self.device.as_ref().unwrap(),
                    self.queue.as_ref().unwrap(),
                )
                .unwrap();
            }
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
            WindowEvent::CursorMoved { position, .. } => self.cursor_position = position,
            WindowEvent::KeyboardInput { event, .. } => {
                handle_key_event(event_loop, event);
            }
            _ => (),
        }
    }
}

fn render(surface: &Surface, device: &Device, queue: &Queue) -> anyhow::Result<()> {
    let output = surface.get_current_texture()?;
    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());
    encoder.begin_render_pass(&RenderPassDescriptor {
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &output
                .texture
                .create_view(&TextureViewDescriptor::default()),
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.5,
                    g: 0.5,
                    b: 0.5,
                    a: 0.5,
                }),
                store: wgpu::StoreOp::default(),
            },
        })],
        ..Default::default()
    });

    queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
}

fn handle_key_event(event_loop: &winit::event_loop::ActiveEventLoop, key_event: KeyEvent) {
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
        (KeyCode::KeyR, ElementState::Pressed) => request_redraw(),
        (KeyCode::KeyT, ElementState::Pressed) => toggle_control_flow(event_loop),
        (KeyCode::Escape, ElementState::Pressed) | (KeyCode::KeyQ, ElementState::Pressed) => {
            exit(event_loop)
        }
        _ => {}
    }
}

fn toggle_control_flow(event_loop: &winit::event_loop::ActiveEventLoop) {
    let previous_flow = event_loop.control_flow();
    let new_flow = match previous_flow {
        ControlFlow::Poll => ControlFlow::Wait,
        ControlFlow::Wait => ControlFlow::Poll,
        _ => ControlFlow::Wait,
    };

    event_loop.set_control_flow(new_flow);
    log::info!("Control flow changed: {previous_flow:?} -> {new_flow:?}");
}

fn exit(event_loop: &winit::event_loop::ActiveEventLoop) {
    log::info!("Exiting application");
    event_loop.exit();
}

fn request_redraw() {
    // we need to implement this.
    // the purpose is so that when the control flow is set to ControlFlow::Wait
    // the user must be able to force redraw using key R
}