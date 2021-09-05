use crate::{
    plugin::Plugin,
    ecs::{Component, System, SystemData, Entities, Read, ReadStorage, WriteStorage, HashMapStorage, World, WorldExt, Join, DispatcherBuilder},
    ecs::events::{EventChannel, ReaderId},
    input::{GameAction, ActionValue},
    components::{Position, Velocity},
    math::{Vec2, vec2},
    app::AppConfig,
};

#[derive(Default)]
pub struct PlayerController {
    pub speed: f32,
}

impl PlayerController {
    pub fn new() -> Self {
        Self {speed: 10.0}
    }
}

impl Component for PlayerController {
    type Storage = HashMapStorage<Self>;
}

pub struct PlayerControlSystem {
    reader: Option<ReaderId<GameAction>>,
    dir_vec: Vec2,
}

impl PlayerControlSystem {
    pub fn new() -> Self {
        Self { reader: None, dir_vec: vec2(0.0, 0.0) }
    }
}

impl<'a> System<'a> for PlayerControlSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, EventChannel<GameAction>>,
        ReadStorage<'a, PlayerController>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (entities, event_channel, pcs, mut vel) : Self::SystemData) {
        let events: Vec<_> = event_channel.read(&mut self.reader.as_mut().unwrap()).collect();
        if events.is_empty() {
            return;
        }

        for (entity, pc) in (&*entities, &pcs).join() {
            let mut direction_vec = vec2(0.0, 0.0);
            let mut changed = false;
            for event in &events {
                if let ActionValue::Value(val) = event.value {
                    let dir = match event.name.as_str() {
                        "Up" => {
                            changed = true;
                            vec2(0.0, val)
                        },
                        "Right" => {
                            changed = true;
                            vec2(val, 0.0)
                        },
                        _ => vec2(0.0, 0.0)
                    };
                    direction_vec += dir;
                };
            }
            if changed {
                // TODO: check if nalgebra does this the right way behind the scenes or if this is just
                // dumb
                if direction_vec != vec2(0.0, 0.0) {
                    direction_vec = direction_vec.normalize();
                    self.dir_vec = direction_vec;
                } else {
                    self.dir_vec = vec2(0.0, 0.0);
                }
                log::debug!("direction changed: {}", self.dir_vec);
            }

            *vel.get_mut(entity).unwrap() = Velocity(self.dir_vec * pc.speed);
        }

    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader = Some(world.fetch_mut::<EventChannel<GameAction>>().register_reader());
    }
}

pub struct PlayerControlPlugin;

impl Plugin for PlayerControlPlugin {
    type Config = ();
    fn new(_: &Self::Config) -> Self {
        Self
    }
    fn load(self, world: &mut World, dispatcher: &mut DispatcherBuilder, _: &AppConfig) {
        world.register::<PlayerController>();

        dispatcher.add(PlayerControlSystem::new(), "player_control_System", &[]);
    }
}
