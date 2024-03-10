use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
mod fly_cam;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<WaterMaterial> {
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
            range: 1000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    });
}

fn spawn_water(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WaterMaterial>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
) {
    cmd.spawn(MaterialMeshBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
        material: materials.add(WaterMaterial {
            shallow_water: Color::rgb_u8(210, 162, 255),
            deep_water: Color::rgb_u8(103, 64, 128),
            depth: 0.1,
            strength: 0.5,
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
        ..default()
    });
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct WaterMaterial {
    #[uniform(0)]
    shallow_water: Color,
    #[uniform(1)]
    deep_water: Color,
    #[uniform(2)]
    depth: f32,
    #[uniform(3)]
    strength: f32,
}

impl Material for WaterMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/water_material.wgsl".into()
    }
}
