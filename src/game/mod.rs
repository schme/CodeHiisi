mod world;
mod entity;
mod component;
mod system;

use std::error::Error;

use engine::{
    ecs::{
        entity::{Entities},
        world::{World},
        system::{System},
    },
    renderer::{Renderer},
    platform::{MouseButtonState},
};

use self::{
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
    _old_world: self::world::World,
    world: World,
}

impl Game {

    pub fn new() -> Result<Game, Box<dyn Error>> {
        let mut world = World::new();

        {
            let mut entities = Entities::new();
            for _ in 0..5000 {
                entities.new_entity();
            }
            world.insert(entities);
        }
        world.insert::<FrameData>(Default::default());
        <Moving as System>::setup(&mut world);

        Ok(Game {
            _old_world: self::world::World::new(),
            world,
        })
    }

    pub fn update(&mut self, data: FrameData, renderer : &mut Renderer) {

        *self.world.get_mut::<FrameData>() = data;

        MySystem.run_now(&self.world);
        Moving.run_now(&self.world);

        //system::_mouse_follow(&mut self._old_world.entities, &data);
        //system::_repelled(&mut self._old_world.entities, &data);
        //system::_moving(&mut self._old_world.entities, &data);
        //system::_drawable(&self._old_world.entities, renderer);
    }
}

