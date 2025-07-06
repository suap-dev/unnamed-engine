use crate::graphics::Mesh;

pub struct RenderObject {
    pub mesh: Mesh,

    pub rotation: f32,
    pub scale: f32,
    pub position: [f32; 2],

    // GPU resources
    // TODO: we would like to consolidate all objects into one buffer later
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u16,
}
