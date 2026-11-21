// WESL (Bevy 0.20+). The output epilogue in `fragment` mirrors bevy_sprite_render's
// own mesh2d.wesl, so spine meshes land in the same color space as every other mesh.
import bevy_sprite_render::mesh2d::{
    functions as mesh_functions,
    view_bindings::view,
};

@if(TONEMAP_IN_SHADER)
import bevy_core_pipeline::tonemapping;
@if(SRGB_OUTPUT)
import bevy_render::color_operations::linear_to_srgb;
@if(OKLAB_OUTPUT)
import bevy_render::color_operations::linear_rgb_to_oklab;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(4) color: vec4<f32>,
    @location(10) dark_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(4) color: vec4<f32>,
    @location(10) dark_color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.uv = vertex.uv;
    var model = mesh_functions::get_world_from_local(vertex.instance_index);
    out.world_position = mesh_functions::mesh2d_position_local_to_world(
        model,
        vec4<f32>(vertex.position, 1.0)
    );
    out.position = mesh_functions::mesh2d_position_world_to_clip(out.world_position);
    out.world_normal = mesh_functions::mesh2d_normal_local_to_world(vertex.normal, vertex.instance_index);
    out.color = vertex.color;
    out.dark_color = vertex.dark_color;
    return out;
}

@group(constants::MATERIAL_BIND_GROUP) @binding(0)
var texture: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(1)
var texture_sampler: sampler;

@fragment
fn fragment(
    input: VertexOutput,
) -> @location(0) vec4<f32> {
    let tex_color = textureSample(texture, texture_sampler, input.uv);
    var color = vec4(
        ((tex_color.a - 1.0) * input.dark_color.a + 1.0 - tex_color.rgb) * input.dark_color.rgb + tex_color.rgb * input.color.rgb,
        tex_color.a * input.color.a,
    );
@if(TONEMAP_IN_SHADER)
    color = tonemapping::tone_mapping(color, view.color_grading);
@if(SRGB_OUTPUT)
    color = vec4(linear_to_srgb(color.rgb), color.a);
@if(OKLAB_OUTPUT)
    color = vec4(linear_rgb_to_oklab(color.rgb), color.a);
    return color;
}
