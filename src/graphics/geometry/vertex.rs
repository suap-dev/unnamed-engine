use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode, vertex_attr_array};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
impl Vertex {
    pub fn new(position: [f32; 2], color: [f32; 3]) -> Self {
        Self { position, color }
    }

    pub fn pos(position: [f32; 2]) -> Self {
        Self::new(position, [1.0, 1.0, 1.0])
    }

    const ATTRIBUTES: &[VertexAttribute] = &vertex_attr_array![0 => Float32x2, 1 => Float32x3];
    #[must_use]
    pub const fn vertex_buffer_layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: Self::ATTRIBUTES,
        }
    }
}
