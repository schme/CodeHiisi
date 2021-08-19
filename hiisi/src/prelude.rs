pub use {
    app::{App, AppConfig},
    ecs::{World, WorldExt, System, Builder, Read, Write, Plugin},
    platform::{
        systems::{
            Timer,
            ShouldQuit,
            DeltaTime,
        },
    },
    input::pointer::{
        CursorPos,
    },
};

use {
    ecs::{DispatcherBuilder},
    assets::TextureAssets,
    utils,
};

use std::path::Path;
use platform::{events::PlatformEventSystem, window::WindowSystem};

pub struct WindowPlugin {
    windows: WindowSystem,
    events: PlatformEventSystem,
}

impl Plugin for WindowPlugin {
    type Config = AppConfig;

    fn new(config: &Self::Config) -> Self {
        use platform;
        let (windows, events) = platform::init_window(&config.name, config.window_size);

        WindowPlugin { windows, events }
    }

    fn load(self, _world: &mut World, dispatcher: &mut DispatcherBuilder, _config: &AppConfig) {

        log::info!("Loading plugin: WindowPlugin");

        dispatcher.add_thread_local(self.windows);
        // Technically here goes frame gap, so timer here
        dispatcher.add_thread_local(Timer::new());
        dispatcher.add_thread_local(self.events);
    }
}

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    type Config = ();

    fn new(_config: &Self::Config) -> Self {
        Self
    }

    fn load(self, world: &mut World, dispatcher: &mut DispatcherBuilder, config: &AppConfig) {

        log::info!("Loading plugin: RenderPlugin");

        use renderer::components::{Color, Texture};
        use renderer::systems::{RenderSystem, SpriteSystem};

        world.register::<Texture>();
        world.register::<Color>();

        let window_plugin = WindowPlugin::new(config);

        dispatcher.add_thread_local(SpriteSystem);
        dispatcher.add_thread_local(RenderSystem::new());

        window_plugin.load(world, dispatcher, config);
    }
}



pub struct CorePlugin;

impl Plugin for CorePlugin {
    type Config = ();

    fn new(_config: &Self::Config) -> Self {
        Self
    }

    fn load(self, world: &mut World, dispatcher: &mut DispatcherBuilder, config: &AppConfig) {

        log::info!("Loading plugin: CorePlugin");

        use components::*;

        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Size>();

        RenderPlugin.load(world, dispatcher, config);

        {
            let asset_path = utils::get_asset_path().unwrap();
            let texture_path = Path::new(&asset_path).join("textures");
            let texture_path_str = String::from(texture_path.to_str().expect("Could not parse texture directory"));

            let mut textures = TextureAssets::new(texture_path_str);
            textures.gen_loaded_textures();
            textures.push_loaded_textures();

            world.insert(textures);
        }

    }
}

