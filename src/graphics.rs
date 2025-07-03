use std::sync::Arc;

use wgpu::{wgc::device, *};

#[derive(Default)]
pub struct WgpuContext {
    config: Option<SurfaceConfiguration>,
    instance: Option<Instance>,
    surface: Option<Surface<'static>>,
    device: Option<Device>,
    adapter: Option<Adapter>,
    queue: Option<Queue>,
}
impl WgpuContext {
    pub fn setup(&mut self, window: &Arc<winit::window::Window>) -> anyhow::Result<()> {
        log::debug!("Setting up wgpu");

        let instance = {
            if self.instance.is_none() {
                self.instance = Some(Instance::new(&InstanceDescriptor::default()))
            }
            self.instance.as_ref().unwrap()
        };

        let surface = {
            if self.surface.is_none() {
                self.surface = Some(instance.create_surface(window.clone())?);
            }
            self.surface.as_ref().unwrap()
        };

        let adapter = {
            if self.adapter.is_none() {
                self.adapter = Some(pollster::block_on(instance.request_adapter(
                    &RequestAdapterOptions {
                        power_preference: PowerPreference::default(),
                        force_fallback_adapter: false,
                        compatible_surface: Some(surface),
                    },
                ))?);
            }
            self.adapter.as_ref().unwrap()
        };

        let device = {
            if self.device.is_none() || self.queue.is_none() {
                let (device, queue) = pollster::block_on(
                    self.adapter
                        .as_ref()
                        .unwrap()
                        .request_device(&DeviceDescriptor::default()),
                )?;
                self.device = Some(device);
                self.queue = Some(queue);
            }
            self.device.as_ref().unwrap()
        };

        let config = {
            if self.config.is_none() {
                let surface_size = window.inner_size();
                let surface_capabilities = surface.get_capabilities(adapter);
                self.config = Some(SurfaceConfiguration {
                    usage: TextureUsages::RENDER_ATTACHMENT,
                    format: *surface_capabilities
                        .formats
                        .iter()
                        .find(|format| format.is_srgb())
                        .unwrap(),
                    width: surface_size.width,
                    height: surface_size.height,
                    present_mode: surface_capabilities.present_modes[0],
                    desired_maximum_frame_latency: 2,
                    alpha_mode: surface_capabilities.alpha_modes[0],
                    view_formats: vec![],
                });
            }
            self.config.as_ref().unwrap()
        };

        surface.configure(device, config);

        Ok(())
    }

    // TODO: can assume that all the struct fields are already initialised?
    pub fn resize_surface(&mut self, width: u32, height: u32) {
        log::debug!("Resizing surface");

        // exit early if minimised
        if width == 0 || height == 0 {
            return;
        }

        let surface = self.surface.as_ref().unwrap();
        let device = self.device.as_ref().unwrap();

        let config = self.config.as_mut().unwrap();
        config.width = width;
        config.height = height;

        surface.configure(device, config);
    }

    pub fn render(&self, clear_color: Color) -> anyhow::Result<()> {
        log::debug!("Rendering");

        let output = self.surface.as_ref().unwrap().get_current_texture()?;
        let device = self.device.as_ref().unwrap();

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());
        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
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
        }

        let queue = self.queue.as_ref().unwrap();
        queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
