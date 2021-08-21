extern crate glfw;

pub mod event_keys;

use serde::Deserialize;

use std::sync::mpsc::Receiver;

use ecs::events::EventChannel;
use ecs::{System, SystemData, World, Write};
use input::pointer;
use platform::systems::ShouldQuit;

pub use self::event_keys::*;

pub use self::glfw::Key as PlatformKey;
pub use self::glfw::MouseButton as PlatformMouseButton;
pub use self::glfw::Action as PlatformAction;

pub use self::glfw::{Scancode, WindowEvent, Modifiers};


#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
pub enum Action {
    Release,
    Press,
    Repeat,
}

impl Action {
    #[inline]
    pub fn from_platform_action(pa: PlatformAction) -> Self {
        match pa {
            PlatformAction::Release => Action::Release,
            PlatformAction::Press => Action::Press,
            PlatformAction::Repeat => Action::Repeat,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub enum InputKey {
    Key(Key),
    MouseButton(MouseButton),
}

pub struct KeyEvent {
    pub key: InputKey,
    pub action: Action,
    pub modifiers: Modifiers,
}

impl KeyEvent {
    pub fn from_platform_button(b: PlatformMouseButton, action: PlatformAction, modifiers: Modifiers) -> Self {
        KeyEvent {
            key: InputKey::MouseButton(MouseButton::from_platform_button(b)),
            action: Action::from_platform_action(action),
            modifiers
        }
    }

    pub fn from_platform_key(k: PlatformKey, action: PlatformAction, modifiers: Modifiers) -> Self {
        KeyEvent {
            key: InputKey::Key(Key::from_platform_key(k)),
            action: Action::from_platform_action(action),
            modifiers
        }
    }
}


impl KeyEvent {
    pub fn new(key: InputKey, action: Action, modifiers: Modifiers) -> Self {
        KeyEvent { key, action, modifiers }
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
        for (_, event) in glfw::flush_messages(&self.events) {
            //println!("{:?}", event);
            match event {
                WindowEvent::Key(key, _, action, modifiers) => {
                    ke.single_write(KeyEvent::from_platform_key(key, action, modifiers));
                    log::debug!("{:?}, {:?} with modifiers: {:?}", key, action, modifiers);

                    // TODO: Move this 
                    if key == PlatformKey::Escape && action == PlatformAction::Press {
                        (*quit).0 = true;
                    }
                }
                WindowEvent::MouseButton(button, action, modifiers) => {
                    ke.single_write(KeyEvent::from_platform_button(button, action, modifiers));
                    log::debug!("{:?}, {:?} with modifiers: {:?}", button, action, modifiers);
                }
                WindowEvent::Close => {
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
