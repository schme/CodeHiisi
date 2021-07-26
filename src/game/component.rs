use {
    engine::ecs::{
        component::Component,
        storage::SimpleStorage,
    },
    math::{Vector2, Point2, Vector3},
};

#[derive(Debug)]
pub struct Position(pub Point2<f32>);

impl Default for Position {
    fn default() -> Self {
        Position(Point2{x: 0.0, y: 0.0})
    }
}

impl Component for Position {
    type Storage = SimpleStorage<Self>;
}

#[derive(Debug)]
pub struct Velocity(pub Vector2<f32>);

impl Default for Velocity {
    fn default() -> Self {
        Velocity(Vector2{x: 0.0, y: 0.0})
    }
}

impl Component for Velocity {
    type Storage = SimpleStorage<Self>;
}


//#[derive(Debug)]
pub enum OldComponent {
    Position(Point2<f32>),
    Velocity(Vector2<f32>),
    Size(Vector2<f32>),
    Color(Vector3<f32>),
    Texture(String),
    FollowMouse,
    Drawable,
    Repelled,
}
