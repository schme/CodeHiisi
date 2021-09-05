use serde::Deserialize;

use platform::events::{Action, KeyEvent, InputKey};
use ecs::{
    Read, Write, System, SystemData, World,
    events::{
        EventChannel, ReaderId
    },
};

use std::path::Path;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct InputAction {
    name: String,
    key: InputKey,
    value: Action,
}

pub struct GameAction {
    pub name: String,
}

impl InputAction {
    pub fn satisfied_by_event(&self, event: &KeyEvent) -> bool {
        self.key == event.key && self.value == event.action

    }
}

pub struct InputSystem {
    reader: Option<ReaderId<KeyEvent>>,
    actions: Vec<InputAction>,
}

impl InputSystem {
    pub fn new() -> Self {
        InputSystem { reader: None, actions: Vec::new() }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(path).unwrap();
        let vec: Vec<InputAction> = match ron::de::from_reader(file) {
            Ok(x) => x,
            Err(e) => {
                log::error!("Failed to load input file: {}", e);
                Vec::new()
            }
        };
        InputSystem { reader: None, actions: vec }
    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Read<'a, EventChannel<KeyEvent>>,
        Write<'a, EventChannel<GameAction>>,
    );

    fn run(&mut self, (event_channel, mut action_channel) : Self::SystemData) {
        for event in event_channel.read(&mut self.reader.as_mut().unwrap()) {
            for action in &self.actions {
                if action.satisfied_by_event(event) {
                    action_channel.single_write(
                        GameAction {
                            name: action.name.clone()
                        }
                    );
                    log::debug!("Action sent: {}", &action.name);
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader = Some(world.fetch_mut::<EventChannel<KeyEvent>>().register_reader());
    }
}
