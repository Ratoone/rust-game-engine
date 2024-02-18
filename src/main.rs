pub mod engine;

use engine::{components::{Position, Velocity}, systems::system_update_positions};
use ggez::{event, glam::{vec2, Vec2}, graphics::{self, Color}, GameResult};
use hecs::{PreparedQuery, World};

struct GameState {
    world: World
}

impl GameState {
    fn new() -> GameResult<GameState> {
        let mut world = World::new();
        let _ = world.spawn_batch(vec![
            (Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.0, dy: 0.0 }),
            (Position { x: 100.0, y: 100.0 }, Velocity { dx: 50.0, dy: 0.0 }),
            (Position { x: 200.0, y: 200.0 }, Velocity { dx: 0.0, dy: -1.0 }),
        ]).collect::<Vec<_>>();

        Ok(GameState{world})
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let mut motion_query = PreparedQuery::<(&mut Position, &Velocity)>::default();
        system_update_positions(&mut self.world, &mut motion_query,  &ctx.time.delta());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        for (_, pos) in self.world.query_mut::<&Position>() {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                vec2(pos.x, pos.y),
                10.0,
                2.0,
                Color::WHITE,
            )?;
            canvas.draw(&circle, Vec2::new(0.0, 0.0));
        }

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = GameState::new()?;
    event::run(ctx, event_loop, state)
}
