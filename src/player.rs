use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

use crate::components::Player;
use crate::constants::PLAYER_SPEED;

pub fn player_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = player.single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction == Vec3::ZERO {
        return;
    }

    direction = direction.normalize();
    transform.translation += direction * PLAYER_SPEED * time.delta_secs();
}
