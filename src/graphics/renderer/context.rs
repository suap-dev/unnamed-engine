use std::sync::Arc;

use wgpu::*;
use winit::dpi::PhysicalSize;

use crate::{
    app::State,
    graphics::renderer::{
        pipeline,
        uniforms::{GlobalUniforms, UniformKind},
    },
};

pub struct GraphicsContext {
    surface_config: SurfaceConfiguration,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    pipeline: RenderPipeline,
    uniforms: GlobalUniforms,
}

impl GraphicsContext {
    pub fn setup(window: &Arc<winit::window::Window>, state: &mut State) -> anyhow::Result<Self> {
        log::debug!("Setting up wgpu");

        let instance = Instance::new(&InstanceDescriptor::default());
        let surface = instance.create_surface(window.clone())?;
        let adapter = pipeline::request_adapter(instance, &surface)?;
        let (device, queue) = pipeline::request_device(&adapter)?;
        let surface_config = pipeline::create_surface_config(window, &surface, adapter);
        let uniforms = GlobalUniforms::new(&device);

        let pipeline =
            pipeline::create_render_pipeline(&device, &surface_config, uniforms.layout());

        surface.configure(&device, &surface_config);
        state.ensure_render_data(&device);

        Ok(Self {
            surface_config,
            surface,
            device,
            queue,
            pipeline,
            uniforms,
        })
    }

    #[must_use]
    pub fn get_surface_size(&self) -> PhysicalSize<u32> {
        PhysicalSize {
            width: self.surface_config.width,
            height: self.surface_config.height,
        }
    }

    pub fn resize_surface(&mut self, width: u32, height: u32) -> anyhow::Result<()> {
        log::debug!("Resizing surface");

        if width == 0 || height == 0 {
            anyhow::bail!(
                "Invalid size, dimensions have to be nonzero. {{w: {width}, h: {height}}}"
            );
        }

        self.surface_config.width = width;
        self.surface_config.height = height;

        self.surface.configure(&self.device, &self.surface_config);

        Ok(())
    }

    pub fn render(&self, state: &mut State) -> anyhow::Result<()> {
        log::debug!("Rendering");

        let output = self.surface.get_current_texture()?;

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &output
                        .texture
                        .create_view(&TextureViewDescriptor::default()),
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(state.clear_color),
                        store: StoreOp::default(),
                    },
                })],
                ..Default::default()
            });
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, self.uniforms.bind_group(), &[]);
            // TODO: instead of drawing all the objects separately, try keeping object kind/handle and then it's transform in
            // TODO: keep transforms in separate Vecs, not the entire objects; send transforms as uniforms
            // TODO: rethink ensure_render_data usage. it's quite strange I think. maybe on state-change not on every render?
            state.ensure_render_data(&self.device);
            for obj in &state.render_objects {
                render_pass.set_vertex_buffer(0, obj.vertex_buffer().slice(..));
                // TODO: consider changing IndexFormat to Uint32
                render_pass.set_index_buffer(obj.index_buffer().slice(..), IndexFormat::Uint16);
                render_pass.draw_indexed(0..obj.index_count() as u32, 0, 0..1);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn update_uniform(&mut self, uniform: UniformKind) {
        self.uniforms.update(&self.queue, uniform);
    }
}
