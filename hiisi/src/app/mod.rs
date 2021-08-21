extern crate rand;

use std::{
    error::Error,
};

use platform::systems::PlatformRunner;
use ecs::{World, System, RunNow, DispatcherBuilder};
use plugin::Plugin;


#[derive(Debug)]
pub struct AppConfig {
    pub name: String,
    pub window_size: (u32, u32),
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            name: "Hiisi Engine".to_string(),
            window_size: (800, 600),
        }
    }
}


pub struct AppBuilder<'a, 'b> {
    config: AppConfig,
    dispatcher_builder: DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> AppBuilder<'a, 'b> {

    pub fn build(self, world: World) -> App<'a, 'b> {
        App { config: self.config, dispatcher_builder: self.dispatcher_builder, world }
    }

    pub fn with<T>(mut self, system: T, name: &str, dep: &[&str]) -> Self
    where
        T: for<'c> System<'c> + Send + 'a,
    {
        self.dispatcher_builder = self.dispatcher_builder.with(system, name, dep);
        self
    }

    pub fn with_thread_local<T>(mut self, system: T) -> Self
    where
        T: for<'c> RunNow<'c> + 'b,
    {
        self.dispatcher_builder = self.dispatcher_builder.with_thread_local(system);
        self
    }

    pub fn with_plugin<T>(mut self, world: &mut World, plugin: T) -> Self
        where
            T: Plugin,
    {
        plugin.load(world, &mut self.dispatcher_builder, &self.config);
        self
    }

}


pub struct App<'a, 'b> {
    config: AppConfig,
    dispatcher_builder: DispatcherBuilder<'a, 'b>,
    world: World,
}

impl<'a, 'b> App<'a, 'b> {
    pub fn new(_config: AppConfig) -> AppBuilder<'a, 'b> {
        todo!()
    }

    pub fn builder(config: AppConfig) -> AppBuilder<'a, 'b> {
        AppBuilder {
            config,
            dispatcher_builder: DispatcherBuilder::new(),
        }
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {

        PlatformRunner::new(self.config)
            .run_loop(self.world, self.dispatcher_builder)?;

        println!("Main loop done!");

        Ok(())
    }
}
