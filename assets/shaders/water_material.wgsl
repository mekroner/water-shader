#import bevy_pbr::{
    prepass_utils,
    pbr_fragment::pbr_input_from_standard_material,
    forward_io::VertexOutput,
    forward_io::FragmentOutput,
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
    pbr_functions::alpha_discard,
    mesh_view_bindings::view,
    mesh_view_bindings::globals,
}

@group(2) @binding(50) var<uniform> shallow_water: vec4<f32>;
@group(2) @binding(51) var<uniform> deep_water: vec4<f32>;
@group(2) @binding(52) var<uniform> depth: f32;
@group(2) @binding(53) var<uniform> strength: f32;
@group(2) @binding(54) var main_normal_texture: texture_2d<f32>;
@group(2) @binding(55) var main_normal_sampler: sampler;

fn depth_to_linear(d: f32) -> f32 {
    return -view.projection[3][2] / d;
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let sample_index = 0u;
    let depth_buffer = bevy_pbr::prepass_utils::prepass_depth(mesh.position, sample_index);
    let depth_view_diff = depth_to_linear(mesh.position.z) + depth - (depth_to_linear(depth_buffer));
    let beers_law = exp(-depth_view_diff * strength);
    let water_color = mix(deep_water, shallow_water, beers_law);

    let normal_scale = 2.0;
    var in = mesh;

    let speed = 0.1;
    let t_0 = sin(globals.time * speed);
    let t_1 = globals.time * speed * 0.5;

    let normal = in.world_normal;
    let tangent = in.world_tangent.xyz;
    let bitangent = cross(tangent, normal);

    var mapped_normal = textureSample(main_normal_texture, main_normal_sampler, mesh.uv * normal_scale + vec2(t_0, 0.0)).rgb;
    mapped_normal = mapped_normal * 2.0 - 1.0;
    mapped_normal = mapped_normal.x * tangent + mapped_normal.y * bitangent + mapped_normal.z * normal;
    in.world_normal = normalize(mapped_normal);

    var mapped_normal2 = textureSample(main_normal_texture, main_normal_sampler, mesh.uv * normal_scale + vec2(0.0, t_1)).rgb;
    mapped_normal2 = mapped_normal2 * 2.0 - 1.0;
    mapped_normal2 = mapped_normal2.x * tangent + mapped_normal2.y * bitangent + mapped_normal2.z * normal;
    in.world_normal = normalize(mapped_normal+ mapped_normal2);

    var pbr_input = pbr_input_from_standard_material(in, false);
    pbr_input.material.base_color = water_color;
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    return out.color;
    //return vec4(mapped_normal, 1.0);
}
