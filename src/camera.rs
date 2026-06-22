//* Libraries imports
use bevy::prelude::*;

use crate::components::{MainCamera, Player};
use crate::constants::{CAMERA_DISTANCE, CAMERA_PITCH_DEGREES};

pub fn camera_follow(
    player: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };
    let Ok(mut camera_transform) = camera.single_mut() else {
        return;
    };

    let pitch_radians = CAMERA_PITCH_DEGREES.to_radians();
    let horizontal_distance = CAMERA_DISTANCE * pitch_radians.cos();
    let height = CAMERA_DISTANCE * pitch_radians.sin();

    let target = player_transform.translation + Vec3::Y * 0.5;
    let offset = Vec3::new(0.0, height, horizontal_distance);

    camera_transform.translation = target + offset;
    camera_transform.look_at(target, Vec3::Y);
}
