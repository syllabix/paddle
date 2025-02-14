use bevy::prelude::*;
use components::{Ball, Paddle, Position};

mod components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, Ball::spawn, Paddle::spawn).chain())
        .add_systems(Update, (Ball::movement, Position::project).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2d);
}