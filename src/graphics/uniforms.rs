use winit::dpi::PhysicalSize;

#[repr(C)]
#[derive(Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Stuff {
    surface_size: [f32; 2],
    rotations_per_second: f32,
    time: f32,
}
impl Stuff {
    #[must_use]
    pub fn new(surface_size: PhysicalSize<u32>, rotations_per_second: f32, time_secs: f32) -> Self {
        let surface_size = [surface_size.width as f32, surface_size.height as f32];

        Self {
            surface_size,
            rotations_per_second,
            time: time_secs,
        }
    }
}
