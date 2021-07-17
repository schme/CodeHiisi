use math::{Vector2, Point2, Vector3};
use platform::file::image::Image;

//#[derive(Debug)]
pub enum Component {
    Position(Point2<f32>),
    Velocity(Vector2<f32>),
    Size(Vector2<f32>),
    Color(Vector3<f32>),
    Texture(Image<u8>),
    FollowMouse,
    Drawable,
    Repelled,
}
