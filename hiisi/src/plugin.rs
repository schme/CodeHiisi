
use {
    app::AppConfig,
    ecs::{World, WorldExt, DispatcherBuilder},
    assets::TextureAssets,
    utils,
};

use std::path::Path;
use platform::{events::PlatformEventSystem, window::WindowSystem, systems::Timer};
use input::InputSystem;
use control::{PlayerControlPlugin};



pub trait Plugin {
    type Config;
    fn new(config: &Self::Config) -> Self;
    // XXX: config should be &Self::Config. No idea how to make the app call this generically in
    // that case. Maybe it can't, unless the load function is generic and the caller is responsible
    // for the specifics?
    fn load(self, world: &mut World, dispatcher: &mut DispatcherBuilder, config: &AppConfig);
}


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
    type Config = AppConfig;

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
    type Config = AppConfig;

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
        let input_system = InputSystem::from_file(&config.input_map);

        dispatcher.add(input_system, "input_system", &[]);

        PlayerControlPlugin.load(world, dispatcher, &config);
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

