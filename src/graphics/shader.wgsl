struct Stuff {
    surface_size: vec2f,
    time: f32,
    rotations_per_second: f32,
}

struct VertexInput {
    @location(0) position: vec2f,
    @location(1) color: vec3f,
}

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec3f,
}

const tau = 6.283185307179586;

@group(0) @binding(0)
var<uniform> stuff: Stuff;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    let position = scale(rotate_2d(in.position, tau * stuff.rotations_per_second * stuff.time), stuff.surface_size);

    var out: VertexOutput;
    out.position = vec4f(position, 0, 1);
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return vec4f(in.color, 1); // use in.color, not in
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
