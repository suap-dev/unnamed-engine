use std::f64::consts::TAU;

use crate::graphics::{Mesh, Vertex};

pub fn ngon(n: u16, circumradius: f32, color: wgpu::Color) -> Mesh {
    Mesh {
        vertices: ngon_vertices(n, circumradius, color),
        indices: ngon_indices(n),
    }
}
fn ngon_vertices(n: u16, circumradius: f32, color: wgpu::Color) -> Vec<Vertex> {
    let mut vertices = Vec::new();

    for vertex_nr in 0..n {
        vertices.push(Vertex::new(
            ngon_vertex_pos(vertex_nr, n, circumradius),
            [color.r as f32, color.g as f32, color.b as f32],
        ));
    }

    vertices
}

fn ngon_indices(n: u16) -> Vec<u16> {
    let mut indices = Vec::new();

    for i in 1..n - 1 {
        indices.push(0);
        indices.push(i);
        indices.push(i + 1);
    }

    indices
}

fn ngon_vertex_pos(vertex_nr: u16, n: u16, circumradius: f32) -> [f32; 2] {
    let v0 = [0.0, -circumradius as f64];
    rotate_2d(v0, vertex_nr as f64 * TAU / (n as f64))
}

fn rotate_2d(v: [f64; 2], angle: f64) -> [f32; 2] {
    let c = angle.cos();
    let s = angle.sin();

    [(v[0] * c - v[1] * s) as f32, (v[0] * s + v[1] * c) as f32]
}
