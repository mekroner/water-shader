#import bevy_pbr::{
    prepass_utils,
    forward_io::VertexOutput,
    mesh_view_bindings::view,
}

@group(2) @binding(0) var<uniform> shallow_water: vec4<f32>;
@group(2) @binding(1) var<uniform> deep_water: vec4<f32>;
@group(2) @binding(2) var<uniform> depth: f32;
@group(2) @binding(3) var<uniform> strength: f32;

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
    let out = mix(deep_water, shallow_water, beers_law);

    return out;
}
