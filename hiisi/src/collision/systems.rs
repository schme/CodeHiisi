use super::*;

use crate::ecs::{ReadStorage, System, SystemData, World};

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = ReadStorage<'a, Cuboid>;

    fn run(&mut self, _cuboids: Self::SystemData) {}

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
    }
}
