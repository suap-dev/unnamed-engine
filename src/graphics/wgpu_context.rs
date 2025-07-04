use std::{num::NonZero, sync::Arc};

use wgpu::{wgc::device, *};
use winit::dpi::PhysicalSize;

use crate::graphics::uniforms::Stuff;

#[derive(Default)]
pub struct WgpuContext {
    config: Option<SurfaceConfiguration>,
    instance: Option<Instance>,
    surface: Option<Surface<'static>>,
    device: Option<Device>,
    adapter: Option<Adapter>,
    queue: Option<Queue>,
    pipeline: Option<RenderPipeline>,
    bind_group_layout: Option<BindGroupLayout>,
    uniform_buffer: Option<Buffer>,
    bind_group: Option<BindGroup>,
}

impl WgpuContext {
    pub fn setup(&mut self, window: &Arc<winit::window::Window>) -> anyhow::Result<()> {
        log::debug!("Setting up wgpu");

        self.ensure_instance();
        self.ensure_surface(window)?;
        self.ensure_adapter()?;
        self.ensure_device()?;
        self.ensure_config(window.inner_size());
        self.ensure_stuff_bind_group_layout();
        self.ensure_pipeline();

        self.configure_surface();

        Ok(())
    }

    pub fn get_surface_size(&self) -> PhysicalSize<u32> {
        let config = self.config.as_ref().unwrap();
        PhysicalSize {
            width: config.width,
            height: config.height,
        }
    }

    // TODO: can assume that all the struct fields are already initialised?
    pub fn resize_surface(&mut self, width: u32, height: u32) -> anyhow::Result<()> {
        log::debug!("Resizing surface");

        if width == 0 || height == 0 {
            anyhow::bail!(
                "Invalid size, dimensions have to be nonzero. {{w: {width}, h: {height}}}"
            );
        }

        let surface = self.surface.as_ref().unwrap();
        let device = self.device.as_ref().unwrap();

        let config = self.config.as_mut().unwrap();
        config.width = width;
        config.height = height;

        surface.configure(device, config);
        Ok(())
    }

    pub fn render(&self, clear_color: Color) -> anyhow::Result<()> {
        log::debug!("Rendering");

        let output = self.surface.as_ref().unwrap().get_current_texture()?;
        let device = self.device.as_ref().unwrap();

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());
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
            let pipeline = self.pipeline.as_ref().unwrap();
            render_pass.set_pipeline(pipeline);
            if let Some(bind_group) = &self.bind_group {
                render_pass.set_bind_group(0, bind_group, &[]);
            }
            render_pass.draw(0..3, 0..1);
        }

        let queue = self.queue.as_ref().unwrap();
        queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn update_stuff_uniform(&mut self, data: &Stuff) {
        let device = self.device.as_ref().unwrap();
        let queue = self.queue.as_ref().unwrap();
        let bind_group_layout = self.bind_group_layout.as_ref().unwrap();

        // ensure uniform buffer?
        if self.uniform_buffer.is_none() {
            self.uniform_buffer = Some(device.create_buffer(&BufferDescriptor {
                label: Some("Stuff Uniform Buffer"),
                size: 16,
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));

            let uniform_buffer = self.uniform_buffer.as_ref().unwrap();
            let stuff_entry = BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            };
            self.bind_group = Some(device.create_bind_group(&BindGroupDescriptor {
                label: Some("Stuff Uniform Bind Group"),
                layout: bind_group_layout,
                entries: &[stuff_entry],
            }));
        }

        let uniform_buffer = self.uniform_buffer.as_ref().unwrap();
        queue.write_buffer(uniform_buffer, 0, bytemuck::cast_slice(&[*data]));
    }

    // private
    fn ensure_instance(&mut self) {
        if self.instance.is_none() {
            self.instance = Some(Instance::new(&InstanceDescriptor::default()))
        }
    }

    fn ensure_surface(&mut self, window: &Arc<winit::window::Window>) -> anyhow::Result<()> {
        if self.surface.is_none() {
            let instance = self.instance.as_ref().unwrap();

            self.surface = Some(instance.create_surface(window.clone())?);
        }
        Ok(())
    }

    fn ensure_adapter(&mut self) -> anyhow::Result<()> {
        if self.adapter.is_none() {
            let instance = self.instance.as_ref().unwrap();
            let surface = self.surface.as_ref().unwrap();

            self.adapter = Some(pollster::block_on(instance.request_adapter(
                &RequestAdapterOptions {
                    power_preference: PowerPreference::default(),
                    force_fallback_adapter: false,
                    compatible_surface: Some(surface),
                },
            ))?);
        }
        Ok(())
    }

    fn ensure_device(&mut self) -> anyhow::Result<()> {
        if self.device.is_none() || self.queue.is_none() {
            let adapter = self.adapter.as_ref().unwrap();
            let (device, queue) =
                pollster::block_on(adapter.request_device(&DeviceDescriptor::default()))?;

            self.device = Some(device);
            self.queue = Some(queue);
        }
        Ok(())
    }

    fn ensure_config(&mut self, surface_size: PhysicalSize<u32>) {
        if self.config.is_none() {
            let surface = self.surface.as_ref().unwrap();
            let adapter = self.adapter.as_ref().unwrap();
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
    }

    fn ensure_stuff_bind_group_layout(&mut self) {
        let device = self.device.as_ref().unwrap();

        let bind_group_layout_0_entry_0 = BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::VERTEX,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: Some(NonZero::new(16).unwrap()), // we KNOW the size of Stuff
            },
            count: None, // only applies to arrays of elements in buffers
        };

        let bind_group_layout_0 = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout #0"),
            entries: &[bind_group_layout_0_entry_0],
        });

        self.bind_group_layout = Some(bind_group_layout_0);
    }

    fn configure_surface(&mut self) {
        let device = self.device.as_ref().unwrap();
        let config = self.config.as_ref().unwrap();
        let surface = self.surface.as_ref().unwrap();

        surface.configure(device, config);
    }

    fn ensure_pipeline(&mut self) {
        self.pipeline = Some(self.create_render_pipeline());
    }

    fn create_render_pipeline(&mut self) -> RenderPipeline {
        let device = self.device.as_ref().unwrap();
        let config = self.config.as_ref().unwrap();
        let bind_group_layout = self.bind_group_layout.as_ref().unwrap();

        let shader_module = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader #0"),
            source: ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let vertex_state = VertexState {
            module: &shader_module,
            entry_point: Some("vs_main"),
            compilation_options: PipelineCompilationOptions::default(),
            buffers: &[],
        };

        let color_target_state = ColorTargetState {
            format: config.format,
            blend: None,
            write_mask: ColorWrites::ALL,
        };

        let fragment_state = FragmentState {
            module: &shader_module,
            entry_point: Some("fs_main"),
            compilation_options: PipelineCompilationOptions::default(),
            targets: &[Some(color_target_state)],
        };

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipeline Layout #0"),
            bind_group_layouts: &[bind_group_layout],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline #0"),
            layout: Some(&pipeline_layout),
            vertex: vertex_state,
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(fragment_state),
            multiview: None,
            cache: None,
        })
    }
}
