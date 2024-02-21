#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct Scale {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovementInput {
    pub acceleration: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Friction {
    pub acceleration: f32,
}
