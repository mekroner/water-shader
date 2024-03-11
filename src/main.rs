use std::f32::consts::PI;

use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::{
        render_resource::{AsBindGroup, ShaderRef},
        texture::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    },
};
mod fly_cam;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, WaterMaterial>> {
                prepass_enabled: false,
                ..default()
            },
            fly_cam::FlyCamPlugin,
        ))
        .insert_resource(ClearColor(Color::rgb_u8(112, 142, 250)))
        .add_systems(Startup, (spawn_light, spawn_water))
        .run()
}

fn spawn_light(mut cmd: Commands) {
    cmd.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 10_000_000.0,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 4.0, 0.0),
        ..default()
    });
}

fn spawn_water(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, WaterMaterial>>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let sampler_desc = ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..default()
    };

    let settings = move |s: &mut ImageLoaderSettings| {
        s.sampler = ImageSampler::Descriptor(sampler_desc.clone());
    };

    cmd.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(Plane3d::default().mesh().size(20., 20.)).with_generated_tangents().unwrap()),
        material: materials.add(ExtendedMaterial {
            base: StandardMaterial { 
                perceptual_roughness: 0.22,
                // metallic: 1.0,
                // normal_map_texture: Some(asset_server.load_with_settings("textures/normals/water.png", settings.clone())),
                // specular_transmission: 1.0,
                ..default() },
            extension: WaterMaterial {
                shallow_water: Color::rgb_u8(210, 162, 255),
                deep_water: Color::rgb_u8(103, 64, 128),
                depth: 0.1,
                strength: 0.5,
                main_normal_texture: asset_server
                    .load_with_settings("textures/normals/water20.png", settings),
            },
        }),
        ..default()
    });

    cmd.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
        material: std_materials.add(Color::SILVER),
        transform: Transform::from_xyz(0.0, -1.0, 0.0),
        ..default()
    });

    cmd.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
        material: std_materials.add(Color::SILVER),
        transform: Transform::from_xyz(0.0, -1.0, 0.0)
            .with_rotation(Quat::from_rotation_x(PI / 16.0)),
        ..default()
    });

    cmd.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: std_materials.add(Color::SILVER),
        transform: Transform::from_scale(Vec3::new(1.0, 3.0, 1.0)),
        ..default()
    });
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct WaterMaterial {
    #[uniform(50)]
    shallow_water: Color,
    #[uniform(51)]
    deep_water: Color,
    #[uniform(52)]
    depth: f32,
    #[uniform(53)]
    strength: f32,
    #[texture(54)]
    #[sampler(55)]
    main_normal_texture: Handle<Image>,
}

impl MaterialExtension for WaterMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/water_material.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        ShaderRef::Default
    }
}
