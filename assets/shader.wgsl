struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec3<f32>
};
struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) tex_coords: vec4<f32>
};
@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(model.position.xyz,1.0);
    out.tex_coords = model.tex_coords.xyz;
    return out;
}

@group(0) @binding(0)
var texture: texture_cube_array<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture,texture_sampler,in.tex_coords,0);
    // return textureSample(texture,texture_sampler,vec3<f32>(0.5,0.5,1.0),0);
    // return vec4(0.6,0.4,0.7,1.0);
}