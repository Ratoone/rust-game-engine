use ggez::input::keyboard;
use hecs::{PreparedQuery, World};

use super::motion::Acceleration;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovementInput {
    pub acceleration: f32,
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
