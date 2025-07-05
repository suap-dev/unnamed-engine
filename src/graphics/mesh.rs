use std::f64::consts::TAU;

use crate::graphics::vertrex::Vertex;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}
