use app::{App, AppConfig};
use ecs::{World, System, DispatcherBuilder};

pub trait Plugin {

    fn load(&mut self, world: &mut World, dispatcher: &mut DispatcherBuilder, config: &AppConfig);

    fn unload(&mut self) {
    }
}
