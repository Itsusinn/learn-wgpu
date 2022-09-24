struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_index: i32,
    @location(2) tex_coords: vec3<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec3<f32>,
    @location(1) tex_index: i32,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    let position = camera.view_proj * vec4<f32>(in.position,1.0);
    out.clip_position = position;
    out.tex_coords = in.tex_coords;
    out.tex_index = in.tex_index;
    return out;
}

@group(0) @binding(0)
var texture: texture_cube_array<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var out: vec4<f32>;
    out = textureSample(texture,texture_sampler,in.tex_coords,in.tex_index);
    return out;
}