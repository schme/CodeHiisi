use app::AppConfig;
use ecs::{DispatcherBuilder, World};

pub trait Plugin {
    type Config;
    fn new(config: &Self::Config) -> Self;
    fn load(self, world: &mut World, dispatcher: &mut DispatcherBuilder, config: &AppConfig);
}
