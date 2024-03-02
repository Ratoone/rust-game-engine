pub mod engine;

use std::path;

use engine::{
    collisions::{
        system_find_collisions, system_handle_collisions, Collider, Collisions, QuadTreeNode,
    },
    components::Scale,
    input::{system_player_move, MovementInput},
    motion::{
        system_friction, system_update_positions, system_update_velocity, Acceleration, Friction,
        Position, Velocity,
    },
};
use ggez::{
    event,
    glam::vec2,
    graphics::{self, Color, Image, Rect},
    input::keyboard,
    mint::Point2,
    Context, GameResult,
};
use hecs::{Entity, PreparedQuery, World};

struct GameState {
    world: World,
    root: Option<QuadTreeNode<Entity>>,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let mut world = World::new();
        let _ = world
            .spawn_batch(vec![(
                Position { x: 250.0, y: 100.0 },
                Collider {
                    mask: 0,
                    layer: 1,
                    width: 20.0,
                    height: 20.0,
                    behaviour: Some(engine::collisions::ColliderBehaviour::Block),
                },
            )])
            .collect::<Vec<_>>();

        world.spawn((
            Position { x: 000.0, y: 000.0 },
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
            Collider {
                mask: 1,
                layer: 0,
                width: 50.0,
                height: 50.0,
                behaviour: None,
            },
        ));

        Ok(GameState { world, root: None })
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let mut motion_query = PreparedQuery::<(&mut Position, &Velocity)>::default();
        let mut acc_query = PreparedQuery::<(&mut Velocity, &Acceleration)>::default();
        let mut friction_query = PreparedQuery::<(&mut Velocity, &Friction)>::default();
        let mut collision_query = PreparedQuery::<(&Position, &Collider)>::default();
        let mut handle_collision_query =
            PreparedQuery::<(&mut Position, &mut Collisions)>::default();
        let delta = &ctx.time.delta();
        system_friction(&mut self.world, &mut friction_query, delta);
        system_update_velocity(&mut self.world, &mut acc_query, delta);
        system_update_positions(&mut self.world, &mut motion_query, delta);
        self.root = Some(system_find_collisions(
            &mut self.world,
            &mut collision_query,
        ));
        system_handle_collisions(&mut self.world, &mut handle_collision_query, delta);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        if self.root.is_some() {
            self.root.as_ref().unwrap().display(&mut canvas, ctx);
        }
        for (_, (pos, image, scale)) in self.world.query_mut::<(&Position, &Image, &Scale)>() {
            canvas.draw(
                image,
                graphics::DrawParam::new()
                    .dest(vec2(pos.x, pos.y))
                    .scale(vec2(scale.x, scale.y)),
            );
        }

        for (_, (pos, collider)) in self.world.query_mut::<(&Position, &Collider)>() {
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(1.0),
                Rect::new(0.0, 0.0, collider.width, collider.height),
                Color::WHITE,
            )?;
            canvas.draw(&rect, graphics::DrawParam::new().dest(vec2(pos.x, pos.y)));
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
