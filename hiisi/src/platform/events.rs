extern crate glfw;

use std::sync::mpsc::Receiver;

use ecs::{System, SystemData, Write, World};
use ecs::events::{EventChannel};
use platform::systems::ShouldQuit;
use input::pointer;

pub use self::glfw::{
    Action,
    Key,
    WindowEvent,
};

pub struct PlatformEventSystem {
    events: Receiver<(f64, WindowEvent)>,    
}

impl PlatformEventSystem {
    pub fn new(events: Receiver<(f64, WindowEvent)>) -> Self {
        PlatformEventSystem { events }
    }
}

impl<'a> System<'a> for PlatformEventSystem {
    type SystemData =
        (Write<'a, EventChannel<WindowEvent>>,
        Write<'a, ShouldQuit>,
        Write<'a, pointer::CursorPos>);

    fn run(&mut self, (_channel, mut quit, mut cursor_pos): Self::SystemData) {

        for (_, event) in glfw::flush_messages(&self.events) {
            //println!("{:?}", event);
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) | WindowEvent::Close => {
                    (*quit).0 = true;
                },
                WindowEvent::Size(width, height) => {
                    // FIXME: This is kinda ugly here
                    use renderer;
                    renderer::resize_viewport(width, height);
                },
                WindowEvent::CursorPos(x, y) => {
                    *cursor_pos = pointer::CursorPos::new(x as f32, y as f32);
                }
                _ => {},
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        log::debug!("Setting up: PlatformEventSystem");
    }
}
