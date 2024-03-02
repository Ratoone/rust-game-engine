use std::{collections::HashMap, time::Duration};

use ggez::{
    glam::vec2,
    graphics::{self, Canvas, Color, Rect},
    Context,
};
use hecs::{Entity, PreparedQuery, World};

use super::motion::{Position, Velocity};

#[derive(Copy, Clone, Debug)]
pub enum ColliderBehaviour {
    Block,
    Physics,
    Event,
}

#[derive(Copy, Clone, Debug)]
pub struct Collider {
    pub mask: i16,
    pub layer: i16,
    pub width: f32,
    pub height: f32,
    pub behaviour: Option<ColliderBehaviour>,
}

#[derive(Debug)]
struct Collision {
    entity: Entity,
    collider: Collider,
}

#[derive(Debug)]
pub struct Collisions(Vec<Collision>);

pub struct QuadTreeNode<T: Copy> {
    top_left: Option<Box<QuadTreeNode<T>>>,
    top_right: Option<Box<QuadTreeNode<T>>>,
    bottom_left: Option<Box<QuadTreeNode<T>>>,
    bottom_right: Option<Box<QuadTreeNode<T>>>,
    values: Vec<T>,
    center: Position,
    width: f32,
    height: f32,
}

impl<T: Copy> QuadTreeNode<T> {
    fn new(center: Position, width: f32, height: f32) -> QuadTreeNode<T> {
        QuadTreeNode::<T> {
            top_left: None,
            top_right: None,
            bottom_left: None,
            bottom_right: None,
            values: Vec::new(),
            center: center,
            width: width,
            height: height,
        }
    }

    fn add(
        &mut self,
        value: T,
        value_position: &Position,
        value_width: f32,
        value_height: f32,
    ) -> Vec<T> {
        assert!(value_position.x >= self.center.x - self.width / 2.0);
        assert!(value_position.x + value_width <= self.center.x + self.width / 2.0);
        assert!(value_position.y >= self.center.y - self.height / 2.0);
        assert!(value_position.y + value_height <= self.center.y + self.height / 2.0);
        let mut potential_cols = Vec::new();
        if value_position.x + value_width < self.center.x {
            if value_position.y + value_height < self.center.y {
                if self.top_left.is_none() {
                    self.top_left = Some(Box::new(QuadTreeNode::new(
                        Position {
                            x: self.center.x - self.width / 4.0,
                            y: self.center.y - self.height / 4.0,
                        },
                        self.width / 2.0,
                        self.height / 2.0,
                    )));
                }
                potential_cols = self.top_left.as_mut().unwrap().add(
                    value,
                    value_position,
                    value_width,
                    value_height,
                );
            }
            if value_position.y > self.center.y {
                if self.bottom_left.is_none() {
                    self.bottom_left = Some(Box::new(QuadTreeNode::new(
                        Position {
                            x: self.center.x - self.width / 4.0,
                            y: self.center.y + self.height / 4.0,
                        },
                        self.width / 2.0,
                        self.height / 2.0,
                    )));
                }
                potential_cols = self.bottom_left.as_mut().unwrap().add(
                    value,
                    value_position,
                    value_width,
                    value_height,
                );
            }
        }
        if value_position.x > self.center.x {
            if value_position.y + value_height < self.center.y {
                if self.top_right.is_none() {
                    self.top_right = Some(Box::new(QuadTreeNode::new(
                        Position {
                            x: self.center.x + self.width / 4.0,
                            y: self.center.y - self.height / 4.0,
                        },
                        self.width / 2.0,
                        self.height / 2.0,
                    )));
                }
                potential_cols = self.top_right.as_mut().unwrap().add(
                    value,
                    value_position,
                    value_width,
                    value_height,
                );
            }
            if value_position.y > self.center.y {
                if self.bottom_right.is_none() {
                    self.bottom_right = Some(Box::new(QuadTreeNode::new(
                        Position {
                            x: self.center.x + self.width / 4.0,
                            y: self.center.y + self.height / 4.0,
                        },
                        self.width / 2.0,
                        self.height / 2.0,
                    )));
                }
                potential_cols = self.bottom_right.as_mut().unwrap().add(
                    value,
                    value_position,
                    value_width,
                    value_height,
                );
            }
        }

        if !potential_cols.is_empty() {
            potential_cols.extend(self.values.iter());
            return potential_cols;
        }

        self.values.push(value);
        self.get_all_values_within()
    }

    fn get_all_values_within(&self) -> Vec<T> {
        let mut values = Vec::new();
        values.extend(self.values.iter());
        if self.top_left.is_some() {
            values.extend(
                self.top_left
                    .as_ref()
                    .unwrap()
                    .get_all_values_within()
                    .iter(),
            );
        }
        if self.top_right.is_some() {
            values.extend(
                self.top_right
                    .as_ref()
                    .unwrap()
                    .get_all_values_within()
                    .iter(),
            );
        }
        if self.bottom_left.is_some() {
            values.extend(
                self.bottom_left
                    .as_ref()
                    .unwrap()
                    .get_all_values_within()
                    .iter(),
            );
        }
        if self.bottom_right.is_some() {
            values.extend(
                self.bottom_right
                    .as_ref()
                    .unwrap()
                    .get_all_values_within()
                    .iter(),
            );
        }

        values
    }

    pub fn display(&self, canvas: &mut Canvas, ctx: &mut Context) {
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            Rect::new(
                -self.width / 2.0,
                -self.height / 2.0,
                self.width,
                self.height,
            ),
            Color::WHITE,
        )
        .unwrap();
        canvas.draw(
            &rect,
            graphics::DrawParam::new().dest(vec2(self.center.x, self.center.y)),
        );

        if self.top_left.is_some() {
            self.top_left.as_ref().unwrap().display(canvas, ctx);
        }

        if self.top_right.is_some() {
            self.top_right.as_ref().unwrap().display(canvas, ctx);
        }

        if self.bottom_left.is_some() {
            self.bottom_left.as_ref().unwrap().display(canvas, ctx);
        }

        if self.bottom_right.is_some() {
            self.bottom_right.as_ref().unwrap().display(canvas, ctx);
        }
    }
}

fn collides(
    pos: &Position,
    collision: &Collider,
    other_pos: &Position,
    other_collision: &Collider,
) -> bool {
    if collision.mask & other_collision.layer == 0 {
        return false;
    }

    let dx = (pos.x + collision.width / 2.0 - other_pos.x - other_collision.width / 2.0).abs();
    let dy = (pos.y + collision.height / 2.0 - other_pos.y - other_collision.height / 2.0).abs();
    dx < (collision.width + other_collision.width) / 2.0
        && dy < (collision.height + other_collision.height) / 2.0
}

pub fn system_find_collisions(
    world: &mut World,
    query: &mut PreparedQuery<(&Position, &Collider)>,
) -> QuadTreeNode<Entity> {
    let mut root = QuadTreeNode::<_>::new(Position { x: 0.0, y: 0.0 }, 800.0, 800.0);
    let mut all_collisions = HashMap::new();
    for (id, (pos, collision)) in query.query(world).iter() {
        let collision_ids = root.add(id, pos, collision.width, collision.height);
        for entity in collision_ids {
            if id.id() == entity.id() {
                continue;
            }
            let other_pos = world.get::<&Position>(entity).unwrap();
            let other_col = world.get::<&Collider>(entity).unwrap();
            if collides(pos, collision, &other_pos, &other_col) {
                if !all_collisions.contains_key(&id) {
                    all_collisions.insert(id, Vec::new());
                }

                all_collisions.get_mut(&id).unwrap().push(Collision {
                    entity,
                    collider: *other_col,
                })
            }

            if collides(&other_pos, &other_col, pos, collision) {
                if !all_collisions.contains_key(&entity) {
                    all_collisions.insert(entity, Vec::new());
                }

                all_collisions.get_mut(&entity).unwrap().push(Collision {
                    entity: id,
                    collider: *collision,
                })
            }
        }
    }
    for (id, collisions) in all_collisions {
        world.insert_one(id, Collisions(collisions)).unwrap();
    }
    root
}

pub fn system_handle_collisions(
    world: &mut World,
    query: &mut PreparedQuery<(&mut Position, &mut Collisions)>,
    delta: &Duration,
) {
    for (id, (pos, collisions)) in query.query(world).iter() {
        for collision in &collisions.0 {
            match collision.collider.behaviour {
                Some(ColliderBehaviour::Block) => {
                    let mut vel = world.get::<&mut Velocity>(id).unwrap();
                    pos.x -= vel.dx * delta.as_secs_f32();
                    pos.y -= vel.dy * delta.as_secs_f32();
                    vel.dx = 0.0;
                    vel.dy = 0.0;
                }
                Some(ColliderBehaviour::Event) => todo!(),
                Some(ColliderBehaviour::Physics) => todo!(),
                None => continue,
            }
        }
        *collisions = Collisions(Vec::new());
    }
}
