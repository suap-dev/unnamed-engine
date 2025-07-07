use std::{any::type_name, mem::size_of, num::NonZeroU64};

use bytemuck::{Pod, Zeroable};
use wgpu::*;

pub struct Uniforms {
    layout: BindGroupLayout,
    bind_group: BindGroup,
    time_buffer: Buffer,
    surface_buffer: Buffer,
}
impl Uniforms {
    pub fn new(device: &Device) -> Self {
        // TODO: we can make it safer by makind a method that will return tuples (binding, entry, buffer)
        let time_entry = entry::<TimeUniform>(0, ShaderStages::VERTEX);
        let time_buffer = create_buffer::<TimeUniform>(device);

        let surface_entry = entry::<SurfaceSizeUniform>(1, ShaderStages::VERTEX);
        let surface_buffer = create_buffer::<SurfaceSizeUniform>(device);

        let layout = create_layout(device, &[time_entry, surface_entry]);

        let bind_group = create_bind_group(device, &layout, &[&time_buffer, &surface_buffer]);

        Self {
            layout,
            bind_group,
            time_buffer,
            surface_buffer,
        }
    }

    #[must_use]
    pub fn layout(&self) -> &BindGroupLayout {
        &self.layout
    }

    #[must_use]
    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    pub fn update(&mut self, queue: &Queue, uniform: UniformKind) {
        match uniform {
            UniformKind::Surface(surface_uniform) => queue.write_buffer(
                &self.surface_buffer,
                0,
                bytemuck::bytes_of(&surface_uniform),
            ),
            UniformKind::Time(time_uniform) => {
                queue.write_buffer(&self.time_buffer, 0, bytemuck::bytes_of(&time_uniform))
            }
        }
    }
}

pub enum UniformKind {
    Surface(SurfaceSizeUniform),
    Time(TimeUniform),
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SurfaceSizeUniform {
    size: [f32; 2],
}
impl SurfaceSizeUniform {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            size: [width, height],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct TimeUniform {
    time: f32,
    _padding: [f32; 3],
}
impl TimeUniform {
    pub fn new(time: f32) -> Self {
        Self {
            time,
            _padding: [0.0; 3],
        }
    }
}

#[must_use]
fn entry<T: Pod>(binding: u32, visibility: ShaderStages) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: NonZeroU64::new(size_of::<T>() as u64),
        },
        count: None,
    }
}

#[must_use]
fn create_layout(device: &Device, entries: &[BindGroupLayoutEntry]) -> BindGroupLayout {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Uniform Bind Group Layout"),
        entries,
    })
}

#[must_use]
fn create_buffer<T: Pod>(device: &Device) -> Buffer {
    device.create_buffer(&BufferDescriptor {
        label: Some(&format!("{} Buffer", type_name::<T>())),
        size: size_of::<T>() as u64,
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

#[must_use]
fn create_bind_group(device: &Device, layout: &BindGroupLayout, buffers: &[&Buffer]) -> BindGroup {
    let entries: Vec<BindGroupEntry> = buffers
        .iter()
        .enumerate()
        .map(|(binding, buffer)| BindGroupEntry {
            binding: binding as u32,
            resource: buffer.as_entire_binding(),
        })
        .collect();

    device.create_bind_group(&BindGroupDescriptor {
        label: Some("Uniform Bind Group"),
        layout,
        entries: &entries,
    })
}
