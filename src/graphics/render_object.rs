use wgpu::{Buffer, util::DeviceExt};

use crate::graphics::{Mesh, Transform};

struct RenderData {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u16,
}

pub struct RenderObject {
    // TODO: instead of keeping every info in RenderObcject divide it into components
    // - position/scale/rotation -> Transform
    // - render_data shouldn't be kept in State, and now it is
    // - we'd rather tell our graphics what TYPES of objects are to be rendered and then feed it with their transforms
    // - state only holds objet TYPE and transform, all the render data stay in graphics/wgpu_context; invisible to state and app.
    pub mesh: Mesh,
    pub name: Option<String>,
    pub transform: Transform,
    render_data: Option<RenderData>,
}
impl RenderObject {
    pub fn new(mesh: Mesh, name: Option<&str>, transform: Transform) -> Self {
        Self {
            mesh,
            name: name.map(|name| name.to_string()),
            transform,
            render_data: None,
        }
    }

    pub fn ensure_render_data(&mut self, device: &wgpu::Device) {
        if self.render_data.is_some() {
            return;
        }

        let name_suffix = match &self.name {
            Some(name) => format!(": {name}"),
            None => "".to_owned(),
        };

        self.render_data = Some(RenderData {
            vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("Vertex Buffer{name_suffix}")),
                contents: bytemuck::cast_slice(&self.mesh.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("Index Buffer{name_suffix}")),
                contents: bytemuck::cast_slice(&self.mesh.indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
            index_count: self.mesh.indices.len() as u16,
        });
    }

    #[must_use]
    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.render_data.as_ref().unwrap().vertex_buffer
    }

    #[must_use]
    pub fn index_buffer(&self) -> &wgpu::Buffer {
        &self.render_data.as_ref().unwrap().index_buffer
    }

    pub fn index_count(&self) -> u16 {
        self.render_data.as_ref().unwrap().index_count
    }
}
