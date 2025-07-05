use std::f64::consts::TAU;

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
    pub fn pos(x: f32, y: f32) -> Self {
        Self::new([x, y], [1.0, 1.0, 1.0])
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

// pub fn get_vertices() -> Vec<Vertex> {
//     vec![
//         Vertex::pos(-0.5, -0.5),
//         Vertex::pos(-0.5, 0.5),
//         Vertex::pos(0.5, 0.5),
//         Vertex::pos(0.5, -0.5),
//     ]
// }

pub fn get_vertices() -> Vec<Vertex> {
    vec![
        Vertex::new([-0.5, -0.5], [1.0, 0.0, 0.0]),
        Vertex::new([-0.5, 0.5], [0.0, 1.0, 0.0]),
        Vertex::new([0.5, 0.5], [0.0, 0.0, 1.0]),
        Vertex::new([0.5, -0.5], [0.0, 0.0, 0.0]),
    ]
}

pub fn get_indices() -> Vec<u16> {
    vec![0, 1, 2, 2, 3, 0]
}

// fn n_gon_vertex(vertex_nr: u32, n: u32, radius: f32 /*, color: [f32; 3] */) {
//     let v0 = [0.0, -radius as f64];
//     n as f64 * TAU / (n as f64)
// }

// fn rotate_2d(v: [f64; 2], angle: f64) -> [f32; 2] {
//     let c = angle.cos();
//     let s = angle.sin();

//     [(v[0] * c - v[1] * s) as f32, (v[0] * s + v[1] * c) as f32]
// }
