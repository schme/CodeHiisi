extern crate glfw;

pub use self::glfw::Key;

pub mod pointer {
    use platform::Action;

    use math::{Vec2, vec2};
    use std::fmt;


    pub struct CursorPos(pub Vec2);
    impl CursorPos {
        pub fn new(x: f32, y: f32) -> CursorPos {
            CursorPos { 0: vec2(x, y) }
        }
    }
    impl Default for CursorPos {
        fn default() -> Self {
            CursorPos::new(0.0, 0.0)
        }
    }

    pub struct MouseButtonState {
        pub left: Action,
        pub right: Action,
        pub middle: Action,
    }

    impl Default for MouseButtonState {
        fn default() -> Self {
            MouseButtonState {
                left: Action::Release,
                right: Action::Release,
                middle: Action::Release,
            }
        }
    }

    impl fmt::Debug for MouseButtonState {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MouseButtonState")
                .field("left", &self.left)
                .field("right", &self.right)
                .field("middle", &self.middle)
                .finish()
        }
    }
}
