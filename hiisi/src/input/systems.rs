use platform::events::KeyEvent;
use ecs::{
    Read, Write, System, SystemData, World,
    events::{
        EventChannel, ReaderId
    },
};

use super::{GameAction, InputMapping};

use std::path::Path;
use std::fs::File;

pub struct InputSystem {
    reader: Option<ReaderId<KeyEvent>>,
    mapping: Vec<InputMapping>,
}

impl InputSystem {
    pub fn new() -> Self {
        InputSystem { reader: None, mapping: Vec::new() }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(path).unwrap();
        let vec: Vec<InputMapping> = match ron::de::from_reader(file) {
            Ok(x) => x,
            Err(e) => {
                log::error!("Failed to load input file: {}", e);
                Vec::new()
            }
        };
        InputSystem { reader: None, mapping: vec }
    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Read<'a, EventChannel<KeyEvent>>,
        Write<'a, EventChannel<GameAction>>,
    );

    fn run(&mut self, (event_channel, mut action_channel) : Self::SystemData) {
        for event in event_channel.read(&mut self.reader.as_mut().unwrap()) {
            for mapped in &self.mapping {
                if let Some(action) = mapped.get_action(event) {
                    action_channel.single_write(action);
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader = Some(world.fetch_mut::<EventChannel<KeyEvent>>().register_reader());
    }
}
