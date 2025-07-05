use std::sync::Arc;

use wgpu::*;
use winit::dpi::PhysicalSize;

use crate::graphics::{buffers, pipeline::*, primitives, uniforms::Stuff};

pub struct WgpuContext {
    surface_config: SurfaceConfiguration,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    pipeline: RenderPipeline,
    bind_group: BindGroup,
    // TODO: clean up the buffers initialisations; what should and what shouldn't be in wgpu context?
    uniform_buffer: Buffer,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
}

const N_GON_VERTICES: u16 = 8;
const N_GON_CIRCUMRADIUS: f32 = 0.66;
const N_GON_COLOR: Color = Color::RED;

impl WgpuContext {
    pub fn setup(window: &Arc<winit::window::Window>) -> anyhow::Result<Self> {
        log::debug!("Setting up wgpu");

        let instance = Instance::new(&InstanceDescriptor::default());
        let surface = instance.create_surface(window.clone())?;
        let adapter = request_adapter(instance, &surface)?;
        let (device, queue) = request_device(&adapter)?;
        let surface_config = create_surface_config(window, &surface, adapter);
        let uniform_buffer = create_uniform_buffer(&device);
        let stuff_bind_group_layout = create_bind_group_layout(&device);
        let stuff_bind_group =
            create_bind_group(&device, &uniform_buffer, &stuff_bind_group_layout);

        let pipeline = create_render_pipeline(&device, &surface_config, &stuff_bind_group_layout);

        surface.configure(&device, &surface_config);

        // TODO: remove hardcodded test-buffers
        let ngon = primitives::ngon(N_GON_VERTICES, N_GON_CIRCUMRADIUS, N_GON_COLOR);
        let vertex_buffer = buffers::create_vertex_buffer(&device, &ngon.vertices);
        let index_buffer = buffers::create_index_buffer(&device, &ngon.indices);

        let index_count = ngon.indices.len() as u32;

        Ok(Self {
            surface_config,
            surface,
            device,
            queue,
            pipeline,
            bind_group: stuff_bind_group,
            uniform_buffer,
            vertex_buffer,
            index_buffer,
            index_count, // TODO: check if index_count field isn't redundant in WgpuContext struct
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

    pub fn render(&self, clear_color: Color) -> anyhow::Result<()> {
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
                        load: LoadOp::Clear(clear_color),
                        store: StoreOp::default(),
                    },
                })],
                ..Default::default()
            });
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.index_count, 0, 0..1);
            // render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn update_stuff_uniform(&mut self, data: &Stuff) {
        self.queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[*data]));
    }
}
