use crate::rand::{self, Rng};

use math::{Point2, Vector2, Vector3};
use super::entity::Entity;
use super::component::{OldComponent};

pub struct World {
    pub entities : Vec<Entity>,
}

impl World {
    pub fn new() -> World {
        let mut rng = rand::thread_rng();
        let mut world = World { entities: Vec::new() };

        let mut entity = Entity::new();
        entity.add_component(OldComponent::Position(Point2{x: 0.0, y: 0.0}));
        entity.add_component(OldComponent::Size(Vector2{x: 10.0, y: 10.0}));
        entity.add_component(OldComponent::Color(Vector3{x: 1.0, y: 0.0, z: 1.0}));
        entity.add_component(OldComponent::FollowMouse);
        entity.add_component(OldComponent::Drawable);
        world.add_entity(entity);

        let mut entity = Entity::new();
        entity.add_component(OldComponent::Position(Point2{x: 100.0, y: 100.0}));
        entity.add_component(OldComponent::Size(Vector2{x: 400.0, y: 400.0}));
        entity.add_component(OldComponent::Color(Vector3{x: 1.0, y: 1.0, z: 1.0}));
        entity.add_component(OldComponent::Texture("kivi.png".to_string()));
        entity.add_component(OldComponent::Drawable);
        world.add_entity(entity);

        for _n in 1..5000 {
            let mut entity = Entity::new();
            entity.add_component(OldComponent::Position(Point2{x: rng.gen_range(0.0..800.0), y: rng.gen_range(0.0..800.0)}));
            entity.add_component(OldComponent::Velocity(Vector2{x: rng.gen_range(-10.0..10.0), y: rng.gen_range(-10.0..10.0)}));
            entity.add_component(OldComponent::Size(Vector2{x: 20.0, y: 40.0}));
            let color = Vector3{x: rng.gen_range(0.0..1.0), y: rng.gen_range(0.0..1.0), z: rng.gen_range(0.0..1.0)};
            entity.add_component(OldComponent::Color(color));
            entity.add_component(OldComponent::Drawable);
            entity.add_component(OldComponent::Texture("auringonkukka.png".to_string()));
            entity.add_component(OldComponent::Repelled);
            world.add_entity(entity);
        }

        world
    }

    pub fn add_entity(&mut self, entity : Entity) {
        self.entities.push(entity);
    }
}
