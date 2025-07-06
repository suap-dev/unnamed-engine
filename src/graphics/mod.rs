pub mod primitives;
pub mod render_object;

mod math;
mod mesh;
mod renderer;
mod vertex;

pub use mesh::Mesh;
pub use renderer::context::GraphicsContext;
pub use vertex::Vertex;
