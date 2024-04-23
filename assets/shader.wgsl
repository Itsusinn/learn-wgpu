struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(1) @binding(0)
var<uniform> camera: CameraUniform;


struct VertexInput {
    @location(0) position: vec2<f32>,
};
struct InstanceInput {
    @location(5) pos: vec2<f32>,
    @location(6) tex_id: u32,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;

    // out.clip_position = camera.view_proj * vec4<f32>(model.position + instance.pos, 0.0, 1.0);
    let tmp = camera.view_proj * vec4<f32>(model.position + instance.pos, 0.0, 1.0);
    // out.clip_position = vec4<f32>(tmp.x, tmp.y, (tmp.z+1.0)/2, tmp.w);
    out.clip_position = tmp;
    out.tex_coord = model.position;
    return out;
}
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) tex_id: u32,
};
@group(0) @binding(0)
var texture_array: texture_2d_array<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // https://www.w3.org/TR/WGSL/#texturesample
    // return vec4<f32>(0.3, 0.7, 0.7, 0.5);
    return textureSample(
        texture_array,
        texture_sampler,
        in.tex_coord,
        in.tex_id
    );
}