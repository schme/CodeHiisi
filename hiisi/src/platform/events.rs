extern crate glfw;

use std::sync::mpsc::Receiver;

use ecs::events::EventChannel;
use ecs::{System, SystemData, World, Write};
use input::pointer;
use platform::systems::ShouldQuit;

pub use self::glfw::{Action, Key, Scancode, Modifiers, WindowEvent};

pub struct KeyEvent {
    key: Key,
    scancode: Scancode,
    action: Action,
    modifiers: Modifiers,
}
impl KeyEvent {
    pub fn new(key: Key, scancode: Scancode, action: Action, modifiers: Modifiers) -> Self {
        KeyEvent { key, scancode, action, modifiers }
    }
}

pub struct PlatformEventSystem {
    events: Receiver<(f64, WindowEvent)>,
}

impl PlatformEventSystem {
    pub fn new(events: Receiver<(f64, WindowEvent)>) -> Self {
        PlatformEventSystem { events }
    }
}

impl<'a> System<'a> for PlatformEventSystem {
    type SystemData = (
        Write<'a, EventChannel<WindowEvent>>,
        Write<'a, EventChannel<KeyEvent>>,
        Write<'a, ShouldQuit>,
        Write<'a, pointer::CursorPos>,
    );

    fn run(&mut self, (_we, mut ke, mut quit, mut cursor_pos): Self::SystemData) {
        log::trace!("Running PlatformEventSystem");
        for (_, event) in glfw::flush_messages(&self.events) {
            //println!("{:?}", event);
            match event {
                WindowEvent::Key(key, scancode, action, modifiers) => {
                    *ke.write( KeyEvent::new(key, scancode, action, modifiers) );
                }
                WindowEvent::Key(Key::Escape, _, Action::Press, _) | WindowEvent::Close => {
                    (*quit).0 = true;
                }
                WindowEvent::Size(width, height) => {
                    // FIXME: This is kinda out of place here
                    use renderer;
                    renderer::resize_viewport(width, height);
                }
                WindowEvent::CursorPos(x, y) => {
                    *cursor_pos = pointer::CursorPos::new(x as f32, y as f32);
                }

                _ => {}
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
    }
}
