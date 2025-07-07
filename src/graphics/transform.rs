#[derive(Clone, Copy)]
pub struct Transform {
    pub position: [f32; 2],
    pub rotation: f32,
    pub scale: [f32; 2],
}
impl Default for Transform {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0],
            rotation: 0.0,
            scale: [1.0, 1.0],
        }
    }
}
