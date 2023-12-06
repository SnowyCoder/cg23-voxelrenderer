// Vertex shader

struct Camera {
    view_proj: mat4x4<f32>,
}
@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec3<f32>,
}
struct InstanceInput {
    @location(5) pos: vec3<f32>,
    @location(6) color: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: u32,
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(model.position + instance.pos, 1.0);
    out.color = instance.color;
    return out;
}

// Fragment shader

@group(1) @binding(0) var t_color : texture_2d<f32>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var y = in.color >> 16u;
    var x = in.color & 0xFFFFu;

    // Look mum, no sampler!
    return textureLoad(t_color, vec2(x, y), 0);
}