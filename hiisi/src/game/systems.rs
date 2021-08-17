use ecs::{System, SystemData, Read, WriteStorage, ReadStorage};
use platform::DeltaTime;
use components::*;

pub struct UpdatePosition;
impl<'a> System<'a> for UpdatePosition {
    type SystemData = (Read<'a, DeltaTime>,
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Velocity>);

    fn run(&mut self, (delta, mut pos, vel): Self::SystemData) {
        use ecs::parallel::prelude::*;
        use ecs::ParJoin;

        let dt = delta.0;

        (&mut pos, &vel)
            .par_join()
            .for_each(|(pos, vel)| {
                pos.0 += vel.0 * dt;
            });
    }
}


