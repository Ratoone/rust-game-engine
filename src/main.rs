pub mod engine;

use std::path;

use engine::{
    components::{Acceleration, Friction, MovementInput, Position, Scale, Velocity},
    systems::{
        system_friction, system_player_move, system_update_positions, system_update_velocity,
    },
};
use ggez::{
    event,
    glam::vec2,
    graphics::{self, Image},
    input::keyboard,
    Context, GameResult,
};
use hecs::{PreparedQuery, World};

struct GameState {
    world: World,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let mut world = World::new();
        let _ = world
            .spawn_batch(vec![
                (Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.0, dy: 0.0 }),
                (
                    Position { x: 200.0, y: 200.0 },
                    Velocity { dx: 0.0, dy: -1.0 },
                ),
            ])
            .collect::<Vec<_>>();

        world.spawn((
            Position { x: 100.0, y: 100.0 },
            Velocity::default(),
            Acceleration {
                dx: 0.0,
                dy: 0.0,
                max_speed: 50.0,
            },
            Image::from_path(ctx, "/potion.png")?,
            Scale { x: 0.1, y: 0.1 },
            MovementInput {
                acceleration: 200.0,
            },
            Friction { acceleration: 50.0 },
        ));

        Ok(GameState { world })
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let mut motion_query = PreparedQuery::<(&mut Position, &Velocity)>::default();
        let mut acc_query = PreparedQuery::<(&mut Velocity, &Acceleration)>::default();
        let mut friction_query = PreparedQuery::<(&mut Velocity, &Friction)>::default();
        let delta = &ctx.time.delta();
        system_friction(&mut self.world, &mut friction_query, delta);
        system_update_velocity(&mut self.world, &mut acc_query, delta);
        system_update_positions(&mut self.world, &mut motion_query, delta);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        for (_, (pos, image, scale)) in self.world.query_mut::<(&Position, &Image, &Scale)>() {
            canvas.draw(
                image,
                graphics::DrawParam::new()
                    .dest(vec2(pos.x, pos.y))
                    .scale(vec2(scale.x, scale.y)),
            );
        }

        canvas.finish(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        let mut motion_query = PreparedQuery::<(&mut Acceleration, &MovementInput)>::default();
        system_player_move(&mut self.world, &mut motion_query, input, true);
        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        input: keyboard::KeyInput,
    ) -> Result<(), ggez::GameError> {
        let mut motion_query = PreparedQuery::<(&mut Acceleration, &MovementInput)>::default();
        system_player_move(&mut self.world, &mut motion_query, input, false);

        Ok(())
    }
}

pub fn main() -> GameResult {
    let assets_dir = path::PathBuf::from("./assets");
    let cb = ggez::ContextBuilder::new("super_simple", "ggez").add_resource_path(assets_dir);
    let (mut ctx, event_loop) = cb.build()?;
    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
