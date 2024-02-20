pub mod engine;

use std::path;

use engine::{components::{Position, Scale, Velocity}, systems::system_update_positions};
use ggez::{event, glam::vec2, graphics::{self, Image}, input::keyboard, Context, GameResult};
use hecs::{Entity, PreparedQuery, World};

struct GameState {
    world: World,
    player: Entity
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let mut world = World::new();
        let _ = world.spawn_batch(vec![
            (Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.0, dy: 0.0 }),
            (Position { x: 200.0, y: 200.0 }, Velocity { dx: 0.0, dy: -1.0 }),
        ]).collect::<Vec<_>>();

        let player = world.spawn((
            Position { x: 100.0, y: 100.0 }, 
            Velocity { dx: 0.0, dy: 0.0 }, 
            Image::from_path(ctx, "/potion.png")?,
            Scale { x: 0.1, y: 0.1 }
        ));

        Ok(GameState{world, player})
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

        for (_, (pos, image, scale)) in self.world.query_mut::<(&Position, &Image, &Scale)>() {
            canvas.draw(image, graphics::DrawParam::new().dest(vec2(pos.x, pos.y)).scale(vec2(scale.x, scale.y)));
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
        let movement = match input.keycode {
            Some(keyboard::KeyCode::A) => vec2(-1.0, 0.0),
            Some(keyboard::KeyCode::D) => vec2(1.0, 0.0),
            Some(keyboard::KeyCode::S) => vec2(0.0, 1.0),
            Some(keyboard::KeyCode::W) => vec2(0.0, -1.0),
            _ => vec2(0.0, 0.0)
        };

        *self.world.get::<&mut Velocity>(self.player).unwrap() = Velocity {dx: 50.0 * movement.x, dy: 50.0 * movement.y};

        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: keyboard::KeyInput) -> Result<(), ggez::GameError> {
        if vec![Some(keyboard::KeyCode::W), Some(keyboard::KeyCode::A), Some(keyboard::KeyCode::S),Some(keyboard::KeyCode::D)].iter().any(|&key| key==input.keycode) {
            *self.world.get::<&mut Velocity>(self.player).unwrap() = Velocity {dx: 0.0, dy: 0.0 };
        }

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
