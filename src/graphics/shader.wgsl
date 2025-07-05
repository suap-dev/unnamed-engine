struct Stuff {
    surface_size: vec2f,
    time: f32,
}

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec3f,
}

@group(0) @binding(0)
var<uniform> stuff: Stuff;

const tau = 6.283185307179586;
const tau_3 = tau / 3;
const v0 = vec2f(0.0, 0.5);

const colors = array(vec3f(0, 0.33, 0.33), vec3f(0.33, 0, 0.33), vec3f(0.33, 0.33, 0));

fn rotate_2d(v: vec2f, angle: f32) -> vec2f {
    let c = cos(angle);
    let s = sin(angle);

    return vec2f(v.x * c - v.y * s, v.x * s + v.y * c);
}

fn scale(v: vec2f, surface_size: vec2f) -> vec2f {
    let smaller_dimension = min(surface_size.x, surface_size.y);
    return vec2f(v.x / surface_size.x, v.y / surface_size.y) * smaller_dimension;
}


@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    let vert = scale(rotate_2d(v0,(f32(vertex_index) * tau_3) + stuff.time / 10), stuff.surface_size);

    var out: VertexOutput;
    out.position = vec4f(vert, 0, 1);
    out.color = colors[vertex_index];

    return out;
}
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return vec4f(in.color, 1);
}
