use hiisi::{
    plugin::Plugin,
    ecs::{Component, System, SystemData, Entities, Read, ReadStorage, WriteStorage, HashMapStorage, World, WorldExt, Join, DispatcherBuilder},
    ecs::events::{EventChannel, ReaderId},
    input::{GameAction, ActionValue},
    components::{Position, Velocity},
    math::{self, Vec2, vec2},
    app::AppConfig,
};

#[derive(Default)]
pub struct PlayerController {
    pub speed: f32,
    pub up_vec: Vec2,
    pub right_vec: Vec2,
}

impl PlayerController {
    pub fn new() -> Self {
        Self {speed: 10.0, up_vec: vec2(0.0, 0.0), right_vec: vec2(0.0, 0.0)}
    }
    pub fn with_speed(speed :f32) -> Self {
        Self {speed, up_vec: vec2(0.0, 0.0), right_vec: vec2(0.0, 0.0) }
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
        WriteStorage<'a, PlayerController>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (entities, event_channel, mut pcs, mut vel) : Self::SystemData) {
        let events: Vec<_> = event_channel.read(&mut self.reader.as_mut().unwrap()).collect();
        if events.is_empty() {
            return;
        }

        for (entity, pc) in (&*entities, &mut pcs).join() {
            for event in &events {
                if let ActionValue::Value(val) = event.value {
                    match event.name.as_str() {
                        "Up" => pc.up_vec = vec2(0.0, val),
                        "Right" => pc.right_vec = vec2(val, 0.0),
                        _ => (),
                    };
                };
            }

            let mut dir_vec = pc.up_vec + pc.right_vec;
            if math::all(&math::equal_eps(&dir_vec, &vec2(0.0, 0.0), f32::EPSILON)) {
                dir_vec = vec2(0.0, 0.0);
            }
            else {
                dir_vec = dir_vec.normalize();
            }
            *vel.get_mut(entity).unwrap() = Velocity(dir_vec * pc.speed);
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
