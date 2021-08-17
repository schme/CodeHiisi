pub mod pointer {
    use math::{Point2};

    pub struct CursorPos(pub Point2<f32>);
    impl CursorPos {
        pub fn new(x: f32, y: f32) -> CursorPos {
            CursorPos{ 0: Point2{ x, y }}
        }
    }
    impl Default for CursorPos {
        fn default() -> Self {
            CursorPos(Point2::new(0.0,0.0))
        }
    }


}
