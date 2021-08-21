extern crate glfw;

pub mod file;
pub mod systems;
pub mod window;
pub mod events;

pub use self::events::PlatformEventSystem;
pub use self::window::WindowSystem;
pub use self::systems::DeltaTime;

pub use self::glfw::{RenderContext, Context};
pub use self::events::{Action, Key, MouseButton};


pub fn init_window(title: &str, window_size: (u32, u32)) -> (WindowSystem, PlatformEventSystem) {

    // FIXME: This doesn't belong here, but kinda just needs to be somewhere right now
    file::image::setup();

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));

    let (width, height) = window_size;
    let (window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW Window");

    (WindowSystem::new(window), PlatformEventSystem::new(events))
}
