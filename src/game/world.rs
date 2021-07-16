use crate::rand::{self, Rng};

use super::entity::Entity;
use super::component::{Component};

#[derive(Debug)]
pub struct World {
    pub entities : Vec<Entity>,
}

impl World {
    pub fn new() -> World {
        let mut rng = rand::thread_rng();
        let mut world = World { entities: Vec::new() };

        let mut entity = Entity::new();
        entity.add_component(Component::Position(0.0, 0.0));
        entity.add_component(Component::Size(10.0, 10.0));
        entity.add_component(Component::Color(1.0, 0.0, 1.0));
        entity.add_component(Component::FollowMouse);
        entity.add_component(Component::Drawable);
        world.add_entity(entity);

        entity = Entity::new();
        entity.add_component(Component::Position(10.0, 10.0));
        entity.add_component(Component::Size(50.0, 50.0));
        entity.add_component(Component::Color(1.0, 1.0, 0.0));
        entity.add_component(Component::Drawable);
        world.add_entity(entity);

        for n in 1..5000 {
            let f = n as f32;
            entity = Entity::new();
            entity.add_component(Component::Position(rng.gen_range(0.0..800.0),rng.gen_range(0.0..800.0)));
            entity.add_component(Component::Velocity(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)));
            entity.add_component(Component::Size(5.0, 5.0));
            let color = (rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0));
            entity.add_component(Component::Color(color.0, color.1, color.2));
            entity.add_component(Component::Drawable);
            world.add_entity(entity);
        }

        world
    }

    pub fn add_entity(&mut self, entity : Entity) {
        self.entities.push(entity);
    }
}
