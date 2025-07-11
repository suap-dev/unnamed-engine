mod vec2;

use std::f64::consts::TAU;

use num_traits::AsPrimitive;

// TODO: find and use native rotate method
pub fn rotated_2d<T: AsPrimitive<f64>, U: AsPrimitive<f64>>(v: [T; 2], angle: U) -> [f64; 2] {
    let (x, y) = (v[0].as_(), v[1].as_());
    let c = angle.as_().cos();
    let s = angle.as_().sin();

    [(x * c - y * s), x * s + y * c]
}

pub fn to_radians<T: AsPrimitive<f64>>(degrees: T) -> f64 {
    degrees.as_() / 360.0 * TAU
}

pub fn to_degrees<T: AsPrimitive<f64>>(rad: T) -> f64 {
    rad.as_() / TAU * 360.0
}
