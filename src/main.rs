//* Libraries imports
use bevy::prelude::{App, DefaultPlugins, Startup, Update};

//* Local imports
mod camera;
mod components;
mod constants;
mod fps;
mod player;
mod setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(fps::FpsPlugin)
        .add_systems(Startup, setup::setup)
        .add_systems(Update, (player::player_movement, camera::camera_follow))
        .run();
}
