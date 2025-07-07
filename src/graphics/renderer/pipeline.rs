use std::sync::Arc;

use wgpu::*;

use crate::graphics::Vertex;

pub fn request_device(adapter: &Adapter) -> Result<(Device, Queue), RequestDeviceError> {
    pollster::block_on(adapter.request_device(&DeviceDescriptor::default()))
}

pub fn request_adapter(
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
pub fn create_surface_config(
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
pub fn create_render_pipeline(
    device: &Device,
    surface_config: &SurfaceConfiguration,
    bind_group_layout: &BindGroupLayout,
) -> RenderPipeline {
    let shader_module = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Shader #0"),
        source: ShaderSource::Wgsl(include_str!("..\\shaders\\basic.wgsl").into()),
    });

    let vertex_state = VertexState {
        module: &shader_module,
        entry_point: Some("vs_main"),
        compilation_options: PipelineCompilationOptions::default(),
        buffers: &[Vertex::vertex_buffer_layout()],
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
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[bind_group_layout],
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
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
