use bevy::prelude::*;

use crate::components::{MainCamera, Player};
use crate::constants::{
    CAMERA_DISTANCE, CAMERA_FOV_DEGREES, GROUND_SIZE, TREE_POSITIONS,
};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        MainCamera,
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: CAMERA_FOV_DEGREES.to_radians(),
            ..default()
        }),
        Transform::from_xyz(0.0, CAMERA_DISTANCE, CAMERA_DISTANCE)
            .looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 5_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.4, 0.0)),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(GROUND_SIZE, GROUND_SIZE))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.75, 0.15),
            ..default()
        })),
        Transform::IDENTITY,
    ));

    commands.spawn((
        Player,
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.25, 0.2),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    let tree_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("tree.gltf"));

    for (x, z) in TREE_POSITIONS {
        commands.spawn((
            SceneRoot(tree_scene.clone()),
            Transform::from_xyz(x, 0.0001, z),
        ));
    }
}
