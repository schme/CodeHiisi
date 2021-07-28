use {
    game::{
        DeltaTime, CursorPos, MouseButtonState,
        components::*,
    },
    engine::{
        ecs::{
            System, Read, Write, ReadStorage, WriteStorage, Join,
        },
        math::{MetricSpace},
        renderer::{Renderer},
        platform::Action,
    },
};

pub struct UpdatePosition;
impl<'a> System<'a> for UpdatePosition {
    type SystemData = (Read<'a, DeltaTime>,
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Velocity>);

    fn run(&mut self, (delta, mut pos, vel): Self::SystemData) {
        use ecs::specs::rayon::prelude::*;
        use engine::ecs::ParJoin;

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
        Read<'a, CursorPos>,
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
        (Read<'a, CursorPos>,
         Read<'a, DeltaTime>,
         Read<'a, MouseButtonState>,
         ReadStorage<'a, Position>,
         WriteStorage<'a, Velocity>);

    fn run(&mut self, (cursor_pos, delta_time, mouse_state, positions, mut velocities): Self::SystemData) {

        let repel_radius2 = 2500f32;
        let attract_radius2 = 10000f32;

        for (vel, pos) in (&mut velocities, &positions).join() {
            let dir = pos.0 - cursor_pos.0;
            let lmb = mouse_state.left;

            if let lmb = Action::Release {
                if pos.0.distance2(cursor_pos.0) < attract_radius2 {
                    vel.0 = - dir / 5.0;
                }
            } else if let lmb = Action::Press {
                if pos.0.distance2(cursor_pos.0) < repel_radius2 {
                    vel.0 += dir;
                }
            }
        }
    }
}


pub struct SpriteSystem;
impl<'a> System<'a> for SpriteSystem {
    type SystemData = 
        (ReadStorage<'a, Position>,
         ReadStorage<'a, Size>,
         ReadStorage<'a, Texture>,
         ReadStorage<'a, Color>);

    fn run(&mut self, (poss, sizes, txtrs, cols): Self::SystemData) {
        for (pos, size, txtr, col) in (&poss, &sizes, &txtrs, &cols).join() {
            //renderer.add_quad(pos.0, size.0, col.0, {
                //if txtr.0.is_empty() {
                    //renderer.get_white_id()
                //}
                //else {
                    //renderer.get_texture_id(txtr)
                //}
            //});
        }
    }
}
