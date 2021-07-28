use {
    engine::ecs::{
        Component, VecStorage, NullStorage,
    },
    math::{Vector2, Point2, Vector3},
};


#[derive(Debug)]
pub struct Position(pub Point2<f32>);
impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Velocity(pub Vector2<f32>);
impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Color(pub Vector3<f32>);
impl Component for Color {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Size(pub Vector2<f32>);
impl Component for Size {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Default)]
pub struct Texture(pub String);
impl Component for Texture {
    type Storage = VecStorage<Self>;
}



#[derive(Debug, Default)]
pub struct FollowingMouse;
impl Component for FollowingMouse {
    type Storage = NullStorage<Self>;
}
