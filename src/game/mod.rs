mod world;
mod components;
mod systems;

use std::error::Error;

use crate::rand::{self, Rng};

use engine::{
    ecs::{
        World, WorldExt, Builder, Dispatcher, DispatcherBuilder,
    },
    math::{Point2, Vector2},
    renderer::{Renderer},
};

use self::{
    components::*,
    systems::*,
};

pub use engine::platform::MouseButtonState;

#[derive(Default, Debug)]
pub struct FrameData {
    pub delta_time: f64,
    pub cursor_x: f64,
    pub cursor_y: f64,
    pub mouse_state: MouseButtonState,
}

#[derive(Default)]
pub struct DeltaTime(f32);

pub struct CursorPos(Point2<f32>);

impl Default for CursorPos {
    fn default() -> Self {
        CursorPos(Point2::new(0.0,0.0))
    }
}

pub struct Game<'a> {
    world: World,
    dispatcher: Dispatcher<'a, 'a>,
}

impl<'a> Game<'a> {

    pub fn new() -> Result<Game<'a>, Box<dyn Error>> {

        let mut world = World::new();
        let mut rng = rand::thread_rng();

        world.insert(DeltaTime(Default::default()));
        world.insert(CursorPos::default());
        world.insert(MouseButtonState::default());


        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Size>();
        world.register::<Texture>();
        world.register::<Color>();

        world.register::<FollowingMouse>();

        for _ in 1..5000 {
            let pos = Position( Point2::new( rng.gen_range(0.0..800.0), rng.gen_range(0.0..800.0)));
            let vel = Velocity( Vector2::new( rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0)));
            let size = Size( Vector2::new(20.0, 20.0));
            let texture = Texture( "auringonkukka.png".to_string());
            world.create_entity()
                .with(pos)
                .with(vel) 
                .with(size)
                .with(texture)
                .build();
        }

        let mut dispatcher = DispatcherBuilder::new()
            .with(FollowMouse, "follow_mouse", &[])
            .with(Repelled, "repelled", &[])
            .with(UpdatePosition, "update_position", &["follow_mouse", "repelled"])
            .with(SpriteSystem, "sprite_system", &["update_position"])
            .build();

        Ok(Game {
            world,
            dispatcher,
        })
    }

    pub fn update(&mut self, data: FrameData) {

        let world = &self.world;

        *world.write_resource::<DeltaTime>() = DeltaTime(data.delta_time as f32);
        *world.write_resource::<CursorPos>() = CursorPos(Point2::new(data.cursor_x as f32, data.cursor_y as f32));
        *world.write_resource::<MouseButtonState>() = data.mouse_state;

        self.dispatcher.dispatch(&mut self.world);
    }
}

