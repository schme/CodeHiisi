extern crate rand;

use std::{
    error::Error,
    path::Path,
};

use crate::{
    ecs::{World, System, RunNow, Dispatcher, DispatcherBuilder, Plugin},
    assets::{TextureAssets},
    utils,
};

use platform::systems::{Timer, PlatformRunner};


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

    pub fn build(self, mut world: World) -> App<'a, 'b> {
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

    pub fn with_plugin<T>(mut self, world: &mut World, mut plugin: T) -> Self
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
    pub fn new(config: AppConfig) -> AppBuilder<'a, 'b> {
        todo!()
    }

    pub fn builder(config: AppConfig) -> AppBuilder<'a, 'b> {
        AppBuilder {
            config,
            dispatcher_builder: DispatcherBuilder::new(),
        }
    }

    pub fn run(mut self) -> Result<(), Box<dyn Error>> {



        let asset_path = utils::get_asset_path()?;
        let texture_path = Path::new(&asset_path).join("textures");
        let mut textures = TextureAssets::new(String::from(texture_path.to_str().expect("Could not parse texture directory")));

        {
            use game::*;
            use app::rand::{Rng};

            let world = &mut self.world;
            let mut rng = rand::thread_rng();

            for _ in 1..10_000 {
                let pos = Position( Point2::new( rng.gen_range(0.0..800.0), rng.gen_range(0.0..800.0)));
                let vel = Velocity( Vector2::new( rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0)));
                let color = Color( Vector3::new( rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)));
                let size = Size( Vector2::new(20.0, 40.0));
                let texture = Texture( textures.get_texture_id("auringonkukka.png").expect("Failed to load texture!"));
                world.create_entity()
                    .with(pos)
                    .with(vel) 
                    .with(size)
                    .with(texture)
                    .with(color)
                    .build();
            }
        }

        self.world.insert(textures);

        PlatformRunner::new(self.config)
            .run_loop(self.world, self.dispatcher_builder);

        println!("Main loop done!");

        Ok(())
    }
}
