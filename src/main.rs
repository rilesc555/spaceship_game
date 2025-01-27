use bevy::prelude::*;

mod debug;
mod movement;
mod spaceship;

fn main() {
    App::new()
        .add_plugins(MovementPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
