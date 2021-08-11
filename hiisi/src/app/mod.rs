extern crate rand;

use std::{
    error::Error,
    path::Path,
};

use crate::{
    ecs::{World, System, RunNow, Dispatcher, DispatcherBuilder, Plugin},
    assets::{TextureStorage},
};

use platform::Platform;


// TODO: Move these!
// vvvvvvvvvvvvvvvvvvv


use math::{Point2, Vector2, Vector3};

pub use crate::platform::MouseButtonState;

#[derive(Default, Debug)]
pub struct FrameData {
    pub delta_time: f64,
    pub window_size: (i32, i32),
    pub cursor_x: f64,
    pub cursor_y: f64,
    pub mouse_state: MouseButtonState,
}

#[derive(Default)]
pub struct DeltaTime(pub f32);

pub struct CursorPos(pub Point2<f32>);
impl Default for CursorPos {
    fn default() -> Self {
        CursorPos(Point2::new(0.0,0.0))
    }
}

// TODO: Move these!
// ^^^^^^^^^^^^^^^^^^^


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
        let mut dispatcher = self.dispatcher_builder.build();
        dispatcher.setup(&mut world);
        App { config: self.config, dispatcher, world }
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
        plugin.load(world, &mut self.dispatcher_builder);
        self
    }

}


pub struct App<'a, 'b> {
    config: AppConfig,
    dispatcher: Dispatcher<'a, 'b>,
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

    pub fn get_world(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn run(mut self) -> Result<(), Box<dyn Error>> {

        use std::env;
        use game::*;
        use app::rand::{Rng};
        use prelude::{ShouldQuit};


        let current_dir = env::current_dir()?;
        let asset_path = String::from(current_dir.join("assets").to_str().expect("Could not parse current directory"));
        let texture_path = Path::new(&asset_path).join("textures");
        let audio_path = Path::new(&asset_path).join("audio");

        let mut platform = Platform::new(&self.config.name, self.config.window_size);

        let mut textures = TextureStorage::new();
        textures.load_textures_from_path(texture_path)?;

        {
            let world = &mut self.world;

            textures.push_loaded_textures();

            let mut rng = rand::thread_rng();

            for _ in 1..5_000 {
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

            self.dispatcher = DispatcherBuilder::new()
                .with(FollowMouse, "follow_mouse", &[])
                .with(Repelled, "repelled", &[])
                .with(UpdatePosition, "update_position", &["follow_mouse", "repelled"])
                .with(SpriteSystem, "sprite_system", &[])
                .with_thread_local(RenderSystem::new(platform.window.render_context()))
                .build();

            self.dispatcher.setup(&mut self.world);
        }

        self.world.insert(textures);

        let mut frame_stop: f64 = 0.0;
        let mut frame_start: f64 = 0.0;
        let mut frame_count = 0;

        loop {
            frame_count += 1;
            let delta_time = frame_stop - frame_start;

            // Relaxed printing
            if frame_count % 60 == 0 {
                println!("delta_time: {:.5} -- fps: {:2.1} -- frame_count: {}", delta_time, 1.0/delta_time, frame_count);
            }

            frame_start = platform.get_time();

            platform.handle_events();

            let (cursor_x, cursor_y) = platform.get_cursor_pos();
            let mouse_state = platform.get_mouse_state();
            let window_size = platform.window.get_size();

            let frame_data = FrameData {
                delta_time,
                window_size,
                cursor_x,
                cursor_y,
                mouse_state,
            };

            {
                *self.world.write_resource::<DeltaTime>() = DeltaTime(frame_data.delta_time as f32);
                *self.world.write_resource::<WindowSize>() = WindowSize(frame_data.window_size.0, frame_data.window_size.1);
                *self.world.write_resource::<CursorPos>() = CursorPos(Point2::new(frame_data.cursor_x as f32, frame_data.cursor_y as f32));
                *self.world.write_resource::<MouseButtonState>() = frame_data.mouse_state;
            }

            self.dispatcher.dispatch(&self.world);

            self.world.maintain();

            platform.poll_events();
            frame_stop = platform.get_time();

            if self.world.read_resource::<ShouldQuit>().0 {
                break;
            }
    
        }

        println!("Main loop done!");

        Ok(())
    }
}
