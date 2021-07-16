use super::entity::{Entity};
use super::component::{Component};

use super::math;
use super::math::MetricSpace;
use crate::renderer::{Renderer};

pub fn moving(entities : &mut Vec<Entity>, frame_data : &super::FrameData) {
    for entity in entities {

        let invalid_indx = entity.components.len();
        let mut position_component_indx = invalid_indx;
        let mut velocity_component_indx = invalid_indx;

        for (i, component) in entity.components.iter().enumerate() {
            match component {
                Component::Velocity(_, _) => {
                    velocity_component_indx = i;
                },
                Component::Position(_, _) => {
                    position_component_indx = i;
                }
                _ => {}
            }
        }
        if velocity_component_indx != invalid_indx && position_component_indx != invalid_indx {
            let prev_position = &entity.components[position_component_indx];
            let velocity = &entity.components[velocity_component_indx];

            if let Component::Position(x, y) = prev_position {
                if let Component::Velocity(vx, vy) = velocity {
                    entity.components[position_component_indx] =
                        Component::Position(x + vx, y + vy);
                }
            }
        }
    }
}

pub fn mouse_follow(entities : &mut Vec<Entity>, frame_data : &super::FrameData ) {
    for entity in entities {

        let updated_position = Component::Position(frame_data.cursor_x as f32, frame_data.cursor_y as f32);
        let invalid_indx = entity.components.len();
        let mut position_component_indx = invalid_indx;
        let mut has_follow_mouse = false;

        for (i, component) in entity.components.iter().enumerate() {
            match component {
                Component::FollowMouse => {
                    has_follow_mouse = true;
                },
                Component::Position(_, _) => {
                    position_component_indx = i;
                }
                _ => {}
            }
        }
        if has_follow_mouse && position_component_indx != invalid_indx {
            entity.components[position_component_indx] = updated_position;
        }
    }
}

pub fn drawable(entities : &Vec<Entity>, renderer : &mut Renderer) {
    for entity in entities {

        let invalid_indx = entity.components.len();
        let mut position = (0.0, 0.0);
        let mut size = (0.0, 0.0);
        let mut color = (1.0, 1.0, 1.0);
        let mut has_drawable = false;

        for component in &entity.components {
            match component {
                Component::Drawable => {
                    has_drawable = true;
                },
                Component::Position(x, y) => {
                    position = (*x, *y);
                }
                Component::Size(w, h) => {
                    size = (*w, *h);
                }
                Component::Color(r, g, b) => {
                    color = (*r, *g, *b);
                }
                _ => {}
            }
        }
        if has_drawable {
            renderer.add_quad((position.0, position.1, size.0, size.1), color);
        }
    }
}

pub fn repelled(entities : &mut Vec<Entity>, frame_data : &super::FrameData) {
    let repel_radius2 = 100f32;
    let mouse_pos = math::Point2::new(frame_data.cursor_x as f32, frame_data.cursor_y as f32);

    for entity in entities {

        let invalid_indx = entity.components.len();
        let mut position_component_indx = invalid_indx;
        let mut velocity_component_indx = invalid_indx;

        for (i, component) in entity.components.iter().enumerate() {
            match component {
                Component::Velocity(_, _) => {
                    velocity_component_indx = i;
                },
                Component::Position(_, _) => {
                    position_component_indx = i;
                }
                _ => {}
            }
        }
        if velocity_component_indx != invalid_indx && position_component_indx != invalid_indx {
            let prev_position = &entity.components[position_component_indx];
            let velocity = &entity.components[velocity_component_indx];

            if let Component::Position(x, y) = prev_position {
                if let Component::Velocity(vx, vy) = velocity {
                    // Here
                    let pos = math::Point2::new(*x, *y);
                    let dir = pos - mouse_pos;

                    if pos.distance2(mouse_pos) > repel_radius2 {
                        let vel = math::vec2(*vx, *vy);
                        let new_vel = vel + 1.0 / dir;
                        entity.components[velocity_component_indx] = Component::Velocity(new_vel.x, new_vel.y);
                    }
                }
            }
        }
    }
}
