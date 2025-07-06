use crate::graphics::Mesh;

mod ngon;

pub fn ngon(n: u16, circumradius: f32, color: wgpu::Color) -> Mesh {
    Mesh {
        vertices: ngon::vertices(n, circumradius, color),
        indices: ngon::indices(n),
    }
}

pub fn triangle(circumradius: f32, color: wgpu::Color) -> Mesh {
    ngon(3, circumradius, color)
}

pub fn square(circumradius: f32, color: wgpu::Color) -> Mesh {
    ngon(4, circumradius, color)
}
