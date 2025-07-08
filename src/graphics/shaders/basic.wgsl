struct SurfaceUniform {
    size: vec2f,
}

struct TimeUniform {
    time: f32,
}

struct VertexInput {
    @location(0) position: vec2f,
    @location(1) color: vec3f,
}

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec3f,
}

struct TransformUniform {
    @location(0) position: vec2f,
    @location(1) rotation: f32,
    @location(2) scale: vec2f,
}

const tau = 6.283185307179586;
@group(0) @binding(0)
var<uniform> time_uniform: TimeUniform;

@group(0) @binding(1)
var<uniform> surface_uniform: SurfaceUniform;

@group(1) @binding(0)
var<uniform> transform_uniform: TransformUniform;


@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    let position = scale(in.position, surface_uniform.size) + transform_uniform.position;

    var out: VertexOutput;
    out.position = vec4f(position, 0, 1);
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return vec4f(in.color, 1);
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
