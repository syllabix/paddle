use bevy::{
    ecs::{
        query::{With, Without},
        system::Query,
    },
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
};

use crate::components::{Ball, Position, Shape, Velocity};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn detect_collision(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let center = ball.center();
    let closest = wall.closest_point(center);
    let offset = center - closest;

    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0.0 {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0.0 {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}

pub fn handle_collisions(
    mut ball: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
    game_objects: Query<(&Position, &Shape), Without<Ball>>,
) {
    if let Ok((mut ball_velocity, ball_position, ball_shape)) = ball.get_single_mut() {
        for (position, shape) in &game_objects {
            if let Some(collision) = detect_collision(
                BoundingCircle::new(ball_position.coords, ball_shape.size.x),
                Aabb2d::new(position.coords, shape.size / 2.0),
            ) {
                match collision {
                    Collision::Left => {
                        ball_velocity.direction.x *= -1.;
                    }
                    Collision::Right => {
                        ball_velocity.direction.x *= -1.;
                    }
                    Collision::Top => {
                        ball_velocity.direction.y *= -1.;
                    }
                    Collision::Bottom => {
                        ball_velocity.direction.y *= -1.;
                    }
                }
            }
        }
    }
}
