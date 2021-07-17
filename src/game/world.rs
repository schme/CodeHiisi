use crate::rand::{self, Rng};
use crate::platform;
use crate::platform::file::image::LoadResult;

use math::{Point2, Vector2, Vector3};
use super::entity::Entity;
use super::component::{Component};

pub struct World {
    pub entities : Vec<Entity>,
}

impl World {
    pub fn new() -> World {
        let mut rng = rand::thread_rng();
        let mut world = World { entities: Vec::new() };

        let mut entity = Entity::new();
        entity.add_component(Component::Position(Point2{x: 0.0, y: 0.0}));
        entity.add_component(Component::Size(Vector2{x: 20.0, y: 20.0}));
        entity.add_component(Component::Color(Vector3{x: 1.0, y: 0.0, z: 1.0}));
        entity.add_component(Component::FollowMouse);
        entity.add_component(Component::Drawable);
        world.add_entity(entity);

        entity = Entity::new();
        entity.add_component(Component::Position(Point2{x: 100.0, y: 100.0}));
        entity.add_component(Component::Size(Vector2{x: 600.0, y: 600.0}));
        entity.add_component(Component::Color(Vector3{x: 1.0, y: 1.0, z: 1.0}));

        let txtr_path = "assets/textures/kivi.png";
        let txtr = platform::file::image::load(txtr_path);
        match txtr {
            LoadResult::ImageU8(img) =>  {
                println!("ImageU8 found at {}:", txtr_path);
                println!("{}, {}, {}", img.width, img.height, img.depth);
                entity.add_component(Component::Texture(img));
            },
            LoadResult::ImageF32(img) => {
                println!("ImageF32 found at {}:", txtr_path);
                println!("{}, {}, {}", img.width, img.height, img.depth);
            },
            LoadResult::Error(s) => {
                println!("Failed to load image at {}: {}", txtr_path, s);
            },
        }
        entity.add_component(Component::Drawable);
        world.add_entity(entity);

        for n in 1..5000 {
            let f = n as f32;
            entity = Entity::new();
            entity.add_component(Component::Position(Point2{x: rng.gen_range(0.0..800.0), y: rng.gen_range(0.0..800.0)}));
            entity.add_component(Component::Velocity(Vector2{x: rng.gen_range(-1.0..1.0), y: rng.gen_range(-1.0..1.0)}));
            entity.add_component(Component::Size(Vector2{x: 20.0, y: 20.0}));
            let color = Vector3{x: rng.gen_range(0.0..1.0), y: rng.gen_range(0.0..1.0), z: rng.gen_range(0.0..1.0)};
            entity.add_component(Component::Color(color));
            entity.add_component(Component::Drawable);
            entity.add_component(Component::Repelled);
            world.add_entity(entity);
        }

        world
    }

    pub fn add_entity(&mut self, entity : Entity) {
        self.entities.push(entity);
    }
}
