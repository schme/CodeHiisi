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
    renderer::systems::{SpriteSystem},
    ecs::{DispatcherBuilder},
};

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn load(&mut self, world: &mut World, dispatcher: &mut DispatcherBuilder, config: &AppConfig) {
        use platform::window::{WindowSystem};
        use platform::events::{PlatformEventSystem};
        use platform;

        let (windows, events) = platform::init_window(&config.name, config.window_size);

        dispatcher.add_thread_local(windows);
        dispatcher.add_thread_local(events);
    }
}

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn load(&mut self, world: &mut World, dispatcher: &mut DispatcherBuilder, config: &AppConfig) {

        use renderer::components::{Color, Texture};
        use renderer::systems::{RenderSystem};
        use components::*;

        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Size>();
        world.register::<Texture>();
        world.register::<Color>();

        dispatcher.add_thread_local(Timer::new());
        WindowPlugin.load(world, dispatcher, config);
        dispatcher.add_thread_local(SpriteSystem);
        dispatcher.add_thread_local(RenderSystem::new());
    }
}

