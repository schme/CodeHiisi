pub use {
    app::{App, AppConfig},
    ecs::{World, WorldExt, System, Plugin, DispatcherBuilder},
};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn load(&mut self, world: &mut World, dispatcher: &mut DispatcherBuilder) {

        use app::{DeltaTime, MouseButtonState, CursorPos};
        use platform::{WindowSize};
        use audio::{AudioQueue};
        use renderer::{QuadBuffer};
        use game::FollowingMouse;
        use renderer::components::{Color, Texture};
        use components::*;

        world.insert(ShouldQuit::default());
        world.insert(DeltaTime::default());
        world.insert(WindowSize::default());
        world.insert(CursorPos::default());
        world.insert(MouseButtonState::default());
        world.insert(QuadBuffer::default());
        world.insert(AudioQueue::default());

        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Size>();
        world.register::<Texture>();
        world.register::<Color>();
        world.register::<FollowingMouse>();
    }
}

#[derive(Default)]
pub struct ShouldQuit(pub bool);
