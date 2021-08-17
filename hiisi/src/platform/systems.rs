use std::sync::mpsc::Receiver;
use std::time::{Instant, Duration};
use std::error::Error;

use app::{AppConfig};
use ecs::{Write, System, SystemData, World, WorldExt, DispatcherBuilder};

use platform::{PlatformEventSystem, WindowSystem};


use math::{Point2, Vector2, Vector3};

pub use crate::platform::MouseButtonState;

#[derive(Default)]
pub struct ShouldQuit(pub bool);

#[derive(Default)]
pub struct DeltaTime(pub f32);

pub struct Timer {
    start_time: Instant,
    prev_instant: Option<Instant>,
}

impl Timer {
    pub fn new() -> Self {
        Timer { start_time: Instant::now(), prev_instant: None, }
    }
}

impl<'a> System<'a> for Timer {
    type SystemData = Write<'a, DeltaTime>;

    fn run(&mut self, (mut delta): Self::SystemData) {

        if let None = self.prev_instant {
            self.prev_instant = Some(self.start_time);
        }

        delta.0 = self.prev_instant.unwrap().elapsed().as_secs_f32();
        self.prev_instant = Some(Instant::now());

        log::trace!("{}", delta.0);
    }
}


pub struct PlatformRunner {
}

impl PlatformRunner {

    pub fn new(config: AppConfig) -> Self {
        PlatformRunner {}
    }

    pub fn run_loop(mut self, mut world: World, mut dispatcher_builder: DispatcherBuilder) -> Result<(), Box<dyn Error>> {

        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(&mut world);

        loop {

            dispatcher.dispatch(&world);
            world.maintain();

            if world.read_resource::<ShouldQuit>().0 {
                break;
            }
        }

        Ok(())
    }

}
