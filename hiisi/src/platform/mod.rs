extern crate glfw;

pub mod file;
pub mod systems;
pub mod window;
pub mod events;

use std::{str, fmt};

pub use self::events::{PlatformEventSystem};
pub use self::window::{WindowSystem};

pub use self::glfw::{RenderContext, Context};
pub use self::glfw::{
    Action,
};


#[derive(Default)]
pub struct WindowSize(pub i32, pub i32);


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

pub fn init_window(title: &str, window_size: (u32, u32)) -> (WindowSystem, PlatformEventSystem) {

    // FIXME: This doesn't belong here, but kinda just needs to be somewhere
    file::image::setup();

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));

    let (width, height) = window_size;
    let (window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW Window");

    (WindowSystem::new(window), PlatformEventSystem::new(events))
}
