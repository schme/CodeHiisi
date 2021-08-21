use std::error::Error;
use std::time::Instant;

use app::AppConfig;
use ecs::{DispatcherBuilder, System, World, WorldExt, Write};

pub use input::pointer::MouseButtonState;

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
        Timer {
            start_time: Instant::now(),
            prev_instant: None,
        }
    }
}

impl<'a> System<'a> for Timer {
    type SystemData = Write<'a, DeltaTime>;

    fn run(&mut self, mut delta: Self::SystemData) {
        if let None = self.prev_instant {
            self.prev_instant = Some(self.start_time);
        }

        let elapsed = self.prev_instant.unwrap().elapsed();
        delta.0 = elapsed.as_secs_f32();
        self.prev_instant = Some(Instant::now());

        log::trace!("{:?}", elapsed);
    }
}

pub struct PlatformRunner {}

impl PlatformRunner {
    pub fn new(_config: AppConfig) -> Self {
        PlatformRunner {}
    }

    pub fn run_loop(
        self,
        mut world: World,
        dispatcher_builder: DispatcherBuilder,
    ) -> Result<(), Box<dyn Error>> {

        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(&mut world);

        loop {
            log::trace!("Top of loop");

            dispatcher.dispatch(&world);
            world.maintain();

            if world.read_resource::<ShouldQuit>().0 {
                break;
            }
            log::trace!("Bottom of loop");
        }

        Ok(())
    }
}
