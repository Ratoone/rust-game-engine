use std::time::Duration;

use hecs::{PreparedQuery, World};

use super::components::{Position, Velocity};

pub fn system_update_positions(world: &mut World, query: &mut PreparedQuery<(&mut Position, &Velocity)>, delta: &Duration) {
    for (_id, (pos, vel)) in query.query_mut(world) {
        pos.x += vel.dx * delta.as_secs_f32();
        pos.y += vel.dy * delta.as_secs_f32();
    }
}
