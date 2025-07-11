use num_traits::AsPrimitive;
use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::math::to_radians;

#[derive(Default, Debug, Clone, Copy)]
struct Vec2 {
    x: f64,
    y: f64,
}
impl Vec2 {
    pub fn new<T: AsPrimitive<f64>>(x: T, y: T) -> Self {
        Self {
            x: x.as_(),
            y: y.as_(),
        }
    }

    pub fn from_physical_position<T: AsPrimitive<f64>, U: AsPrimitive<f64>>(
        physical_position: PhysicalPosition<T>,
        surface_size: PhysicalSize<U>,
    ) {
        let (width, height) = (surface_size.width, surface_size.height);
        

    }

    pub fn rotate<T: AsPrimitive<f64>>(&mut self, rad: T) {
        let c = rad.as_().cos();
        let s = rad.as_().sin();

        (self.x, self.y) = ((self.x * c - self.y * s), self.x * s + self.y * c)
    }

    pub fn rotate_degrees<T: AsPrimitive<f64>>(&mut self, degrees: T) {
        let rad = to_radians(degrees);
        self.rotate(rad);
    }
}
impl From<Vec2> for [f32; 2] {
    fn from(val: Vec2) -> Self {
        [val.x as f32, val.y as f32]
    }
}
