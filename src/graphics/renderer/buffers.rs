use wgpu::{
    Buffer, BufferUsages, Device,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::graphics::Vertex;

pub fn create_vertex_buffer(device: &Device, vertices: &[Vertex]) -> Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Vertex Buffer #0"),
        contents: bytemuck::cast_slice(vertices),
        usage: BufferUsages::VERTEX,
    })
}

pub fn create_index_buffer(device: &Device, indices: &[u16]) -> Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Index Buffer #0"),
        contents: bytemuck::cast_slice(indices),
        usage: BufferUsages::INDEX,
    })
}
