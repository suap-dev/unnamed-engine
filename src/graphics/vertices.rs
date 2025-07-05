use std::f64::consts::TAU;

use wgpu::{
    BufferAddress, Color, VertexAttribute, VertexBufferLayout, VertexStepMode, vertex_attr_array,
};

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
    // pub fn pos(position: [f32; 2]) -> Self {
    //     Self::new(position, [1.0, 1.0, 1.0])
    // }

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

pub fn n_gon_vertices(n: u16, circumradius: f32, color: Color) -> Vec<Vertex> {
    let mut vertices = Vec::new();

    for vertex_nr in 0..n {
        vertices.push(Vertex::new(
            n_gon_vertex_pos(vertex_nr, n, circumradius),
            [color.r as f32, color.g as f32, color.b as f32],
        ));
    }

    vertices
}

pub fn n_gon_indices(n: u16) -> Vec<u16> {
    let mut indices = Vec::new();

    for i in 1..n - 1 {
        indices.push(0);
        indices.push(i);
        indices.push(i + 1);
    }

    indices
}

fn n_gon_vertex_pos(vertex_nr: u16, n: u16, circumradius: f32) -> [f32; 2] {
    let v0 = [0.0, -circumradius as f64];
    rotate_2d(v0, vertex_nr as f64 * TAU / (n as f64))
}

fn rotate_2d(v: [f64; 2], angle: f64) -> [f32; 2] {
    let c = angle.cos();
    let s = angle.sin();

    [(v[0] * c - v[1] * s) as f32, (v[0] * s + v[1] * c) as f32]
}
