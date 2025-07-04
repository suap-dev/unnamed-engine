use std::{num::NonZero, sync::Arc};

use wgpu::*;
use winit::dpi::PhysicalSize;

use crate::graphics::uniforms::Stuff;

pub struct WgpuContext {
    surface_config: SurfaceConfiguration,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    pipeline: RenderPipeline,
    bind_group: BindGroup,
    uniform_buffer: Buffer,
}

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
        let stuff_bind_group = create_bind_group(&device, &uniform_buffer);

        let pipeline = create_render_pipeline(&device, &surface_config, &stuff_bind_group_layout);

        surface.configure(&device, &surface_config);

        Ok(Self {
            surface_config,
            surface,
            device,
            queue,
            pipeline,
            bind_group: stuff_bind_group,
            uniform_buffer,
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
            render_pass.draw(0..3, 0..1);
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

fn create_bind_group(
    device: &Device,
    // bind_group_layout: &BindGroupLayout,
    uniform_buffer: &Buffer,
) -> BindGroup {
    let bind_group_layout = create_bind_group_layout(device);

    let stuff_entry = BindGroupEntry {
        binding: 0,
        resource: uniform_buffer.as_entire_binding(),
    };

    device.create_bind_group(&BindGroupDescriptor {
        label: Some("Stuff Uniform Bind Group"),
        layout: &bind_group_layout,
        entries: &[stuff_entry],
    })
}

fn request_device(adapter: &Adapter) -> Result<(Device, Queue), RequestDeviceError> {
    pollster::block_on(adapter.request_device(&DeviceDescriptor::default()))
}

fn request_adapter(
    instance: Instance,
    surface: &Surface<'_>,
) -> Result<Adapter, RequestAdapterError> {
    pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: Some(surface),
    }))
}

#[must_use]
fn create_uniform_buffer(device: &Device) -> Buffer {
    device.create_buffer(&BufferDescriptor {
        label: Some("Stuff Uniform Buffer"),
        size: 16,
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

#[must_use]
fn create_surface_config(
    window: &Arc<winit::window::Window>,
    surface: &Surface<'_>,
    adapter: Adapter,
) -> SurfaceConfiguration {
    let surface_size = window.inner_size();
    let surface_capabilities = surface.get_capabilities(&adapter);
    SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: *surface_capabilities
            .formats
            .iter()
            .find(|format| format.is_srgb())
            .unwrap(),
        width: surface_size.width,
        height: surface_size.height,
        present_mode: PresentMode::AutoVsync,
        desired_maximum_frame_latency: 2,
        alpha_mode: CompositeAlphaMode::Auto,
        view_formats: vec![],
    }
}

#[must_use]
fn create_render_pipeline(
    device: &Device,
    surface_config: &SurfaceConfiguration,
    bind_group_layout: &BindGroupLayout,
) -> RenderPipeline {
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
        format: surface_config.format,
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
        // bind_group_layouts: &[],
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

#[must_use]
fn create_bind_group_layout(device: &Device) -> BindGroupLayout {
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

    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Uniform Bind Group Layout #0"),
        entries: &[bind_group_layout_0_entry_0],
    })
}
