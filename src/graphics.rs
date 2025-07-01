use std::sync::Arc;

use wgpu::{
    Adapter, Color, CommandEncoderDescriptor, Device, Instance, InstanceDescriptor, LoadOp,
    Operations, PowerPreference, Queue, RenderPassColorAttachment, RenderPassDescriptor,
    RequestAdapterOptions, StoreOp, Surface, SurfaceConfiguration, TextureUsages,
    TextureViewDescriptor,
};

pub fn setup(
    window: &Arc<winit::window::Window>,
) -> anyhow::Result<(Surface<'static>, Device, Queue)> {
    log::debug!("Setting up wgpu");

    let instance = Instance::new(&InstanceDescriptor::default());
    let surface = instance.create_surface(window.clone()).unwrap();

    let request_adapter_options = RequestAdapterOptions {
        power_preference: PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
    };
    let adapter = pollster::block_on(instance.request_adapter(&request_adapter_options)).unwrap();

    let (device, queue) =
        pollster::block_on(adapter.request_device(&wgpu::wgt::DeviceDescriptor::default()))
            .unwrap();

    let surface_size = window.inner_size();

    configure_surface(
        &surface,
        &device,
        &adapter,
        surface_size.width,
        surface_size.height,
    );

    Ok((surface, device, queue))
}

pub fn configure_surface(
    surface: &Surface,
    device: &Device,
    adapter: &Adapter,
    width: u32,
    height: u32,
) {
    log::debug!("Configuring surface");

    let surface_capabilities = surface.get_capabilities(adapter);

    log::debug!(
        "Available present modes: {:?}",
        surface_capabilities.present_modes
    );

    surface.configure(
        device,
        &SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: *surface_capabilities
                .formats
                .iter()
                .find(|format| format.is_srgb())
                .unwrap(),
            width,
            height,
            present_mode: surface_capabilities.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        },
    );
}

pub fn render(surface: &Surface, device: &Device, queue: &Queue) -> anyhow::Result<()> {
    log::debug!("Rendering");

    let output = surface.get_current_texture()?;
    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());
    encoder.begin_render_pass(&RenderPassDescriptor {
        color_attachments: &[Some(RenderPassColorAttachment {
            view: &output
                .texture
                .create_view(&TextureViewDescriptor::default()),
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color {
                    r: 0.5,
                    g: 0.5,
                    b: 0.5,
                    a: 0.5,
                }),
                store: StoreOp::default(),
            },
        })],
        ..Default::default()
    });

    queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
}
