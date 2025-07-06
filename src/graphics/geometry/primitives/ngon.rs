use std::f64::consts::TAU;

use crate::graphics::{Vertex, math};

pub fn vertices(n: u16, circumradius: f32, color: wgpu::Color) -> Vec<Vertex> {
    let mut vertices = Vec::new();

    for vertex_nr in 0..n {
        vertices.push(Vertex::new(
            ngon_vertex_pos(vertex_nr, n, circumradius),
            [color.r as f32, color.g as f32, color.b as f32],
        ));
    }

    vertices
}

pub fn indices(n: u16) -> Vec<u16> {
    let mut indices = Vec::new();

    for i in 1..n - 1 {
        indices.push(0);
        indices.push(i);
        indices.push(i + 1);
    }

    indices
}

fn ngon_vertex_pos(vertex_nr: u16, n: u16, circumradius: f32) -> [f32; 2] {
    let v0 = [0.0, circumradius as f64];
    math::rotate_2d(v0, vertex_nr as f64 * TAU / (n as f64))
}
