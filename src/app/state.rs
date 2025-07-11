use std::time::Instant;

use winit::dpi::PhysicalPosition;

use crate::graphics::RenderObject;

pub struct State {
    pub render_objects: Vec<RenderObject>,
    pub cursor_position: PhysicalPosition<f64>,
    pub clear_color: wgpu::Color,
    pub timer: Instant,
}
impl State {
    pub fn ensure_render_data(&mut self, device: &wgpu::Device) {
        for obj in &mut self.render_objects {
            obj.ensure_render_data(device);
        }
    }

    pub fn add_object(&mut self, object: RenderObject) {
        self.render_objects.push(object);
    }
}
