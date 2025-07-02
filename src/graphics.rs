use std::sync::Arc;

use wgpu::*;

#[derive(Default)]
pub struct WgpuContext {
    surface: Option<Surface<'static>>,
    device: Option<Device>,
    adapter: Option<Adapter>,
    queue: Option<Queue>,
}
impl WgpuContext {
    // TODO: currently we do it on every "resumed"; is that a good idea?
    pub fn setup(&mut self, window: &Arc<winit::window::Window>) -> anyhow::Result<()> {
        log::debug!("Setting up wgpu");

        let instance = Instance::new(&InstanceDescriptor::default());
        let surface = instance.create_surface(window.clone())?;

        let request_adapter_options = RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        };
        let adapter = pollster::block_on(instance.request_adapter(&request_adapter_options))?;

        let (device, queue) =
            pollster::block_on(adapter.request_device(&DeviceDescriptor::default()))?;

        self.surface = Some(surface);
        self.adapter = Some(adapter);
        self.device = Some(device);
        self.queue = Some(queue);

        let surface_size = window.inner_size();
        self.configure_surface(surface_size.width, surface_size.height);

        Ok(())
    }

    // TODO: we shouldn't so recklessly assume that all the struct fields are already initialised
    pub fn configure_surface(&self, width: u32, height: u32) {
        log::debug!("Configuring surface");

        // TODO: this can happen once on creation
        let surface_capabilities = self
            .surface
            .as_ref()
            .unwrap()
            .get_capabilities(self.adapter.as_ref().unwrap());

        self.surface.as_ref().unwrap().configure(
            self.device.as_ref().unwrap(),
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

    pub fn render(&self, clear_color: Color) -> anyhow::Result<()> {
        // log::debug!("Rendering");

        let output = self.surface.as_ref().unwrap().get_current_texture()?;

        // TODO: can we create encoder ONCE instead on every render?
        let mut encoder = self
            .device
            .as_ref()
            .unwrap()
            .create_command_encoder(&CommandEncoderDescriptor::default());
        encoder.begin_render_pass(&RenderPassDescriptor {
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &output
                    .texture
                    .create_view(&TextureViewDescriptor::default()),
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(clear_color),
                    store: StoreOp::default(),
                },
            })],
            ..Default::default()
        });

        self.queue
            .as_ref()
            .unwrap()
            .submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
