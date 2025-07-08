use num_traits::AsPrimitive;

use crate::math::to_radians;

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
impl Transform {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> TransformBuilder {
        TransformBuilder::default()
    }
}

pub struct TransformBuilder {
    pub position: [f64; 2],
    pub rotation: f64,
    pub scale: [f64; 2],
}
impl Default for TransformBuilder {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0],
            rotation: 0.0,
            scale: [1.0, 1.0],
        }
    }
}
impl TransformBuilder {
    pub fn position<T: AsPrimitive<f64>>(mut self, x: T, y: T) -> Self {
        self.position = [x.as_(), y.as_()];
        self
    }

    pub fn physical_position<T: AsPrimitive<f64>>(
        mut self,
        physical_position: winit::dpi::PhysicalPosition<T>,
    ) -> Self {
        self.position = [physical_position.x.as_(), physical_position.y.as_()];
        self
    }

    pub fn rotation<T: AsPrimitive<f64>>(mut self, rad: T) -> Self {
        self.rotation = rad.as_();
        self
    }

    pub fn rotation_degrees<T: AsPrimitive<f64>>(mut self, degrees: T) -> Self {
        self.rotation = to_radians(degrees.as_());
        self
    }

    pub fn scale<T: AsPrimitive<f64>>(mut self, scale: [T; 2]) -> Self {
        self.scale = [scale[0].as_(), scale[1].as_()];
        self
    }

    pub fn build(self) -> Transform {
        Transform {
            position: [self.position[0] as f32, self.position[1] as f32],
            rotation: self.rotation as f32,
            scale: [self.scale[0] as f32, self.scale[1] as f32],
        }
    }
}
