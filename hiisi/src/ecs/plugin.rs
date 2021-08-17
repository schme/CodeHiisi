use app::AppConfig;
use ecs::{DispatcherBuilder, World};

pub trait Plugin {
    fn load(&mut self, world: &mut World, dispatcher: &mut DispatcherBuilder, config: &AppConfig);

    fn unload(&mut self) {}
}
