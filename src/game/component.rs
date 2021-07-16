use math::{Vector2};

#[derive(Debug)]
pub enum Component {
    Position(f32, f32),
    Velocity(f32, f32),
    Size(f32, f32),
    Color(f32, f32, f32),
    FollowMouse,
    Drawable,
    Repelled,
}
