mod world;
mod entity;
mod component;
mod system;

use std::error::Error;

use engine::{
    renderer::{Renderer},
    platform::{MouseButtonState},
};

use self::world::World;

pub struct FrameData {
    pub delta_time: f64,
    pub cursor_x: f64,
    pub cursor_y: f64,
    pub mouse_state: MouseButtonState,
}

pub struct Game {
    world: World,
}

impl Game {

    pub fn new() -> Result<Game, Box<dyn Error>> {
        Ok(Game { world: World::new() })
    }

    pub fn update(&mut self, data: FrameData, renderer : &mut Renderer) {

        system::mouse_follow(&mut self.world.entities, &data);
        system::repelled(&mut self.world.entities, &data);
        system::moving(&mut self.world.entities, &data);
        system::drawable(self.world.entities, renderer);
    }
}

