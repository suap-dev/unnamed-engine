struct Stuff {
    surface_size: vec2f,
    time: f32,
}

@group(0) @binding(0)
var<uniform> stuff: Stuff;

const tau = 6.283185307179586;
const tau_3 = tau / 3;
const v0 = vec2f(0.0, 0.5);

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4f {
    let vert = scale(
        rotate_2d(
            v0,
            (f32(vertex_index) * tau_3) + stuff.time / 10),
        stuff.surface_size
    );

    return vec4f(vert, 0, 1);
}

fn rotate_2d(v: vec2f, angle: f32) -> vec2f {
    let c = cos(angle);
    let s = sin(angle);

    return vec2f(v.x * c - v.y * s, v.x * s + v.y * c);
}

fn scale(v: vec2f, surface_size: vec2f) -> vec2f {
    let smaller_dimension = min(surface_size.x, surface_size.y);
    return vec2f(v.x / surface_size.x, v.y / surface_size.y) * smaller_dimension;
}

@fragment
fn fs_main() -> @location(0) vec4f {
    return vec4(0, 0, 0, 1);
}
