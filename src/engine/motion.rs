use std::time::Duration;

use hecs::{PreparedQuery, World};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Acceleration {
    pub dx: f32,
    pub dy: f32,
    pub max_speed: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Friction {
    pub acceleration: f32,
}

pub fn distance(pos1: &Position, pos2: &Position) -> f32 {
    ((pos1.x - pos2.x).powi(2) + (pos1.y - pos2.y).powi(2)).sqrt()
}

pub fn system_update_positions(
    world: &mut World,
    query: &mut PreparedQuery<(&mut Position, &Velocity)>,
    delta: &Duration,
) {
    for (_id, (pos, vel)) in query.query_mut(world) {
        pos.x += vel.dx * delta.as_secs_f32();
        pos.y += vel.dy * delta.as_secs_f32();
    }
}

pub fn system_update_velocity(
    world: &mut World,
    query: &mut PreparedQuery<(&mut Velocity, &Acceleration)>,
    delta: &Duration,
) {
    for (_id, (vel, acc)) in query.query_mut(world) {
        vel.dx = (vel.dx + acc.dx * delta.as_secs_f32())
            .min(acc.max_speed)
            .max(-acc.max_speed);
        vel.dy = (vel.dy + acc.dy * delta.as_secs_f32())
            .min(acc.max_speed)
            .max(-acc.max_speed);
    }
}

pub fn system_friction(
    world: &mut World,
    query: &mut PreparedQuery<(&mut Velocity, &Friction)>,
    delta: &Duration,
) {
    for (_id, (vel, friction)) in query.query_mut(world) {
        vel.dx = if vel.dx == 0.0 {
            0.0
        } else {
            vel.dx.signum()
                * ((vel.dx.abs() - friction.acceleration * delta.as_secs_f32()).max(0.0))
        };
        vel.dy = if vel.dy == 0.0 {
            0.0
        } else {
            vel.dy.signum()
                * ((vel.dy.abs() - friction.acceleration * delta.as_secs_f32()).max(0.0))
        };
    }
}
