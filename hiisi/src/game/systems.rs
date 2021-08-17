use {
    game::{
        components::*,
    },
    crate::{
        prelude::*,
        ecs::{
            System, Read, Write, ReadStorage, WriteStorage, Join,
        },
        components::*,
        math::{MetricSpace},
        platform::{Action, MouseButtonState},
        input::pointer,
    },
};

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


pub struct FollowMouse;
impl<'a> System<'a> for FollowMouse {
    type SystemData =
        (ReadStorage<'a, FollowingMouse>,
        Read<'a, pointer::CursorPos>,
        WriteStorage<'a, Position>);

    fn run(&mut self, (follow, cursor_pos, mut positions): Self::SystemData) {
        for (pos, _) in (&mut positions, &follow).join() {
            *pos = Position(cursor_pos.0);
        }
    }
}


pub struct Repelled;
impl<'a> System<'a> for Repelled {
    type SystemData =
        (Read<'a, pointer::CursorPos>,
         Read<'a, MouseButtonState>,
         ReadStorage<'a, Position>,
         WriteStorage<'a, Velocity>);

    fn run(&mut self, (cursor_pos, mouse_state, positions, mut velocities): Self::SystemData) {

        let repel_radius2 = 2500f32;
        let attract_radius2 = 10000f32;

        for (vel, pos) in (&mut velocities, &positions).join() {
            let dir = pos.0 - cursor_pos.0;
            let lmb = mouse_state.left;

            if lmb == Action::Press {
                if pos.0.distance2(cursor_pos.0) < attract_radius2 {
                    vel.0 = - dir / 5.0;
                }
            } else if lmb == Action::Release {
                if pos.0.distance2(cursor_pos.0) < repel_radius2 {
                    vel.0 += dir;
                }
            }
        }
    }
}

