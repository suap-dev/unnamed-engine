// TODO: find and use native rotate method
pub fn rotate_2d(v: [f64; 2], angle: f64) -> [f32; 2] {
    let c = angle.cos();
    let s = angle.sin();

    [(v[0] * c - v[1] * s) as f32, (v[0] * s + v[1] * c) as f32]
}
