use crate::graphics::Mesh;

mod ngon;

pub fn regular_polygon(vertices: u16, circumradius: f32, color: wgpu::Color) -> Mesh {
    Mesh {
        vertices: ngon::vertices(vertices, circumradius, color),
        indices: ngon::indices(vertices),
    }
}

pub fn triangle(circumradius: f32, color: wgpu::Color) -> Mesh {
    regular_polygon(3, circumradius, color)
}

pub fn square(circumradius: f32, color: wgpu::Color) -> Mesh {
    regular_polygon(4, circumradius, color)
}
