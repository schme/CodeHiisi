use super::*;

use crate::{
    app::AppConfig,
    ecs::{DispatcherBuilder, World, WorldExt},
    plugin::Plugin,
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    type Config = ();
    fn new(_: &Self::Config) -> Self {
        Self
    }

    fn load(self, world: &mut World, dispatcher: &mut DispatcherBuilder, _: &AppConfig) {
        world.register::<Cuboid>();

        dispatcher.add_thread_local(CollisionSystem);
    }
}
