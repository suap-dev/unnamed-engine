use wgpu::*;
use winit::dpi::PhysicalSize;

#[repr(C)]
#[derive(Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AppDataUniform {
    surface_size: [f32; 2],
    time: f32,
    _pad: f32,
}
impl AppDataUniform {
    #[must_use]
    pub fn new(surface_size: PhysicalSize<u32>, time_secs: f32) -> Self {
        let surface_size = [surface_size.width as f32, surface_size.height as f32];

        Self {
            surface_size,
            time: time_secs,
            _pad: 0.0,
        }
    }
}

pub struct Uniforms {
    layout: BindGroupLayout,
    bind_group: BindGroup,
    buffer: Buffer,
}
impl Uniforms {
    pub fn new(device: &Device) -> Self {
        let layout = create_layout(device);
        let buffer = create_buffer(device);
        let bind_group = create_bind_group(device, &layout, &buffer);
        Self {
            layout,
            buffer,
            bind_group,
        }
    }
    pub fn update(&mut self, queue: &Queue, data: &AppDataUniform) {
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(data));
    }

    pub fn layout(&self) -> &BindGroupLayout {
        &self.layout
    }

    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }
}

#[must_use]
fn create_layout(device: &Device) -> BindGroupLayout {
    let entry = BindGroupLayoutEntry {
        binding: 0,
        visibility: ShaderStages::VERTEX,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None, // TODO: if we KNOW the size of Stuff, we can configure it
        },
        count: None, // only applies to arrays of elements in buffers
    };

    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Uniform Bind Group Layout #0"),
        entries: &[entry],
    })
}

#[must_use]
pub fn create_buffer(device: &Device) -> Buffer {
    device.create_buffer(&BufferDescriptor {
        label: Some("Uniform Buffer"),
        size: std::mem::size_of::<AppDataUniform>() as u64,
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

#[must_use]
fn create_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    buffer: &wgpu::Buffer,
) -> BindGroup {
    device.create_bind_group(&BindGroupDescriptor {
        label: Some("Uniform Bind Group"),
        layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
    })
}
