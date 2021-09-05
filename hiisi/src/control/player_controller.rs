use crate::{
    plugin::Plugin,
    ecs::{Component, System, SystemData, Entities, Read, ReadStorage, WriteStorage, HashMapStorage, World, WorldExt, Join, DispatcherBuilder},
    ecs::events::{EventChannel, ReaderId},
    input::{GameAction},
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
}

impl PlayerControlSystem {
    pub fn new() -> Self {
        Self { reader: None }
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
        let events : Vec<_> = event_channel.read(&mut self.reader.as_mut().unwrap()).collect();
        if events.is_empty() {
            return;
        }

        for (entity, pc) in (&*entities, &pcs).join() {
            // XXX: Since this is an iterator, we probably can handle only one player_controller
            // like this. Copy the events and the run the entities to fix.
            let mut direction_vec = vec2(0.0, 0.0);
            for event in &events {
                let dir = match event.name.as_str() {
                    "Left" => vec2(-1.0, 0.0),
                    "Right" => vec2(1.0, 0.0),
                    "Up" => vec2(0.0, 1.0),
                    "Down" => vec2(0.0, -1.0),
                    _ => vec2(0.0, 0.0),
                };
                direction_vec += dir;
            }
            direction_vec = direction_vec.normalize();
            *vel.get_mut(entity).unwrap() = Velocity(direction_vec * pc.speed);
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
