use bevy::prelude::*;

mod asset_loader;
mod asteroids;
mod camera;
mod collision_detection;
mod debug;
mod movement;
mod spaceship;

use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DebugPlugin)
        // default plugins
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
