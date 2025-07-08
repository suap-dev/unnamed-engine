use bytemuck::{Pod, Zeroable};
use num_traits::AsPrimitive;

use crate::math::to_radians;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Transform {
    pub position: [f32; 2],
    pub rotation: f32,
    pub scale: [f32; 2],
    _padding: [f32; 3],
}
impl Default for Transform {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0],
            rotation: 0.0,
            scale: [1.0, 1.0],
            _padding: [0.0; 3],
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

    pub fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Transform Uniform Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: std::num::NonZeroU64::new(size_of::<Transform>() as u64),
                },
                count: None,
            }],
        })
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

    pub fn physical_position<T: AsPrimitive<f64>, U: AsPrimitive<f64>>(
        mut self,
        physical_position: winit::dpi::PhysicalPosition<T>,
        physical_size: winit::dpi::PhysicalSize<U>,
    ) -> Self {
        let half_width = physical_size.width.as_() * 0.5;
        let half_height = physical_size.height.as_() * 0.5;

        let mut x = physical_position.x.as_();
        let mut y = physical_position.y.as_();

        x = (x - half_width) / half_width;
        y = -(y - half_height) / half_height;

        self.position = [x, y];
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
            _padding: [0.0; 3],
        }
    }
}
