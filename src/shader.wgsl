// Vertex shader

struct Camera {
    view_proj: mat4x4<f32>,
}
@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}
struct InstanceInput {
    @location(5) pos: vec3<f32>,
    @location(6) color: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: u32,
    @location(1) v_pos: vec3<f32>,
    @location(2) v_norm: vec3<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(model.position + instance.pos, 1.0);
    out.color = instance.color;
    out.v_pos = model.position + instance.pos;
    out.v_norm = model.normal;
    return out;
}

// Fragment shader

struct PosInfo {
    light: vec3<f32>,
    eye: vec3<f32>,
}

@group(1) @binding(0) var t_color: texture_2d<f32>;
@group(2) @binding(0) var<uniform> pos: PosInfo;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var y = in.color >> 16u;
    var x = in.color & 0xFFFFu;

    // Look mum, no sampler!
    var color = textureLoad(t_color, vec2(x, y), 0);

    var diffuse_map = color * 1.0;
    var specular_map = vec4(1.0);
    var ambient_comp = 0.3 * color;

    var light_dir = normalize(pos.light - in.v_pos);
    var eye_dir = normalize(pos.eye - in.v_pos);
    var diffuse = max(dot(light_dir, in.v_norm), 0.0);

    var half_way = normalize(light_dir + eye_dir);
    var specular = pow(max(dot(half_way, in.v_norm), 0.0), 100.0);

    return ambient_comp + diffuse * diffuse_map + specular * specular_map;
}
