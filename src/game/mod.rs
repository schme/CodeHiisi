mod world;
mod entity;
mod component;
mod system;

use std::error::Error;

use engine::{
    ecs::{
        world::{World, Read},
        system::System,
        storage::{SimpleStorage},
    },
    renderer::{Renderer},
    platform::{MouseButtonState},
};

use self::{
    component::{Position, Velocity},
    system::*,
};

#[derive(Default, Debug)]
pub struct FrameData {
    pub delta_time: f64,
    pub cursor_x: f64,
    pub cursor_y: f64,
    pub mouse_state: MouseButtonState,
}

pub struct Game {
    old_world: self::world::World,
    world: World,
}

impl Game {

    pub fn new() -> Result<Game, Box<dyn Error>> {

        let mut world = World::new();
        world.insert::<FrameData>(Default::default());
        //world.insert::<SimpleStorage<Position>>(Default::default());
        //world.insert::<SimpleStorage<Velocity>>(Default::default());

        Ok(Game {
            old_world: self::world::World::new(),
            world,
        })
    }

    pub fn update(&mut self, data: FrameData, renderer : &mut Renderer) {

        *self.world.get_mut::<FrameData>() = data;

        MySystem.run_now(&self.world);
        Moving.run_now(&self.world);

        //system::mouse_follow(&mut self.old_world.entities, &data);
        //system::repelled(&mut self.old_world.entities, &data);
        //system::moving(&mut self.old_world.entities, &data);
        //system::drawable(&self.old_world.entities, renderer);
    }
}

