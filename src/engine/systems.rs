use std::time::Duration;

use ggez::input::keyboard;
use hecs::{PreparedQuery, World};

use super::components::{Acceleration, Friction, MovementInput, Position, Velocity};

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

pub fn system_player_move(
    world: &mut World,
    query: &mut PreparedQuery<(&mut Acceleration, &MovementInput)>,
    input: ggez::input::keyboard::KeyInput,
    pressed: bool,
) {
    for (_id, (acc, movement)) in query.query_mut(world) {
        match input.keycode {
            Some(keyboard::KeyCode::A) => acc.dx = -movement.acceleration * (pressed as i32 as f32),
            Some(keyboard::KeyCode::D) => acc.dx = movement.acceleration * (pressed as i32 as f32),
            Some(keyboard::KeyCode::S) => acc.dy = movement.acceleration * (pressed as i32 as f32),
            Some(keyboard::KeyCode::W) => acc.dy = -movement.acceleration * (pressed as i32 as f32),
            _ => continue,
        };
    }
}
