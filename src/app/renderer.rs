use wgpu::{
    RenderPassDescriptor,
    wgt::{CommandEncoderDescriptor, TextureViewDescriptor},
};

#[derive(Debug)]
pub(in crate::app) struct Renderer<'window> {
    surface: wgpu::Surface<'window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
}
impl<'window> Renderer<'window> {
    pub fn new(window: &'window winit::window::Window) -> anyhow::Result<Self> {
        let descriptor = wgpu::InstanceDescriptor::default();
        let instance = wgpu::Instance::new(&descriptor);

        let surface = instance.create_surface(window)?;

        let options = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        };
        let adapter = pollster::block_on(instance.request_adapter(&options))?;

        let desc = wgpu::DeviceDescriptor::default();
        let (device, queue) = pollster::block_on(adapter.request_device(&desc))?;

        // TODO: Figure out if this is not too arbitrary
        let (format, present_mode, alpha_mode) = {
            let surface_capabilities = surface.get_capabilities(&adapter);
            (
                *surface_capabilities
                    .formats
                    .iter()
                    .find(|format| format.is_srgb())
                    .unwrap(),
                surface_capabilities.present_modes[0],
                surface_capabilities.alpha_modes[0],
            )
        };

        let size = window.inner_size();
        let (width, height) = (size.width, size.height);

        surface.configure(
            &device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format,
                width,
                height,
                present_mode,
                desired_maximum_frame_latency: 2,
                alpha_mode,
                view_formats: vec![],
            },
        );

        // let surface_capabilities = surface.get_capabilities(&adapter);
        // let adapter = instance.request_adapter();

        Ok(Self {
            surface,
            device,
            queue,
        })
    }
    pub fn render(&self) -> anyhow::Result<()> {
        let output = self.surface.get_current_texture()?;

        let texture_view_desc = TextureViewDescriptor::default();
        let view = &output.texture.create_view(&texture_view_desc);

        let desc = RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
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
        };

        let command_encoder_desc = CommandEncoderDescriptor::default();
        let mut encoder = self.device.create_command_encoder(&command_encoder_desc);
        encoder.begin_render_pass(&desc);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
