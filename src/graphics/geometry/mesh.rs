use crate::graphics::geometry::vertex::Vertex;

#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}
