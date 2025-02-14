//! Game Components Module
//!
//! This module contains all the component structs used in the game.
//! Components are pieces of data that can be attached to entities in the ECS (Entity Component System).
//! They are used to define the behavior and properties of game objects.
//!

use bevy::prelude::*;
use config::{BALL_COLOR, BALL_SIZE, BALL_SPEED, PADDLE_COLOR};

mod config {
    use bevy::color::Color;

    pub const BALL_SIZE: f32 = 5.0;
    pub const BALL_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
    pub const BALL_SPEED: f32 = 5.;

    pub const PADDLE_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
}

#[derive(Component, Default)]
#[require(Transform)]
pub struct Position {
    coords: Vec2,
}

impl Position {
    pub fn project(mut positionables: Query<(&mut Transform, &Position)>) {
        for (mut transform, position) in &mut positionables {
            transform.translation = position.coords.extend(0.)
        }
    }
}

#[derive(Component, Default)]
pub struct Velocity {
    coords: Vec2,
}

#[derive(Component)]
#[require(
    Position,
    Velocity(|| Velocity { coords: Vec2::new(-1., 1.)}),
)]
pub struct Ball;

impl Ball {
    pub fn spawn(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let shape = Circle::new(BALL_SIZE);

        let mesh = meshes.add(shape);
        let material = materials.add(BALL_COLOR);

        commands.spawn((Self, Mesh2d(mesh), MeshMaterial2d(material)));
    }

    pub fn movement(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
        if let Ok((mut position, velocity)) = ball.get_single_mut() {
            position.coords += velocity.coords * BALL_SPEED
        }
    }
}

#[derive(Component)]
#[require(Position)]
pub struct Paddle;

impl Paddle {
    pub fn spawn(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let rect = Rectangle::new(10., 50.);
        let mesh = meshes.add(rect);
        let material = materials.add(PADDLE_COLOR);

        commands.spawn((
            Self,
            Position {
                coords: Vec2 { x: 500., y: 0. },
            },
            Mesh2d(mesh),
            MeshMaterial2d(material),
        ));
    }
}

// #[derive(Component)]
// struct Brick;
