use super::entity::{Entity};
use super::component::{Component};

use super::math;
use super::math::{Point2, Vector2, Vector3};
use super::math::{MetricSpace, InnerSpace};
use crate::renderer::{Renderer};
use crate::platform::Action;

pub fn moving(entities : &mut Vec<Entity>, frame_data : &super::FrameData) {
    for entity in entities {

        let invalid_indx = entity.components.len();
        let mut position_component_indx = invalid_indx;
        let mut velocity_component_indx = invalid_indx;

        for (i, component) in entity.components.iter().enumerate() {
            match component {
                Component::Velocity(_) => {
                    velocity_component_indx = i;
                },
                Component::Position(_) => {
                    position_component_indx = i;
                }
                _ => {}
            }
        }
        if velocity_component_indx != invalid_indx && position_component_indx != invalid_indx {
            let prev_position = &entity.components[position_component_indx];
            let velocity = &entity.components[velocity_component_indx];

            if let Component::Position(pos) = prev_position {
                if let Component::Velocity(vel) = velocity {
                    entity.components[position_component_indx] =
                        Component::Position(Point2{x: pos.x + vel.x, y: pos.y + vel.y});
                }
            }
        }
    }
}

pub fn mouse_follow(entities : &mut Vec<Entity>, frame_data : &super::FrameData ) {
    for entity in entities {

        let updated_position = Component::Position(Point2{x: frame_data.cursor_x as f32, y: frame_data.cursor_y as f32});
        let invalid_indx = entity.components.len();
        let mut position_component_indx = invalid_indx;
        let mut has_follow_mouse = false;

        for (i, component) in entity.components.iter().enumerate() {
            match component {
                Component::FollowMouse => {
                    has_follow_mouse = true;
                },
                Component::Position(_) => {
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

pub fn repelled(entities : &mut Vec<Entity>, frame_data : &super::FrameData) {
    let repel_radius2 = 2500f32;
    let attract_radius2 = 10000f32;
    let mouse_pos = math::Point2::new(frame_data.cursor_x as f32, frame_data.cursor_y as f32);

    for entity in entities {

        let invalid_indx = entity.components.len();
        let mut position_component_indx = invalid_indx;
        let mut velocity_component_indx = invalid_indx;
        let mut has_repelled = false;

        for (i, component) in entity.components.iter().enumerate() {
            match component {
                Component::Velocity(_) => {
                    velocity_component_indx = i;
                },
                Component::Position(_) => {
                    position_component_indx = i;
                },
                Component::Repelled => {
                    has_repelled = true;
                },
                _ => {}
            }
        }
        if velocity_component_indx != invalid_indx
            && position_component_indx != invalid_indx
            && has_repelled {
            let prev_position = &entity.components[position_component_indx];
            let velocity = &entity.components[velocity_component_indx];

            if let Component::Position(pos) = prev_position {
                if let Component::Velocity(vel) = velocity {
                    // Here
                    let dir = pos - mouse_pos;

                    match frame_data.mouse_state.left {
                        Action::Press => {
                            if pos.distance2(mouse_pos) < attract_radius2 {
                                let new_vel = - dir / 100.0;
                                entity.components[velocity_component_indx] = Component::Velocity(new_vel);
                            }
                        }
                        Action::Release => {
                            if pos.distance2(mouse_pos) < repel_radius2 {
                                let new_vel = vel + (2.0 / dir);
                                entity.components[velocity_component_indx] = Component::Velocity(new_vel);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

pub fn drawable(entities : &Vec<Entity>, renderer : &mut Renderer) {
    for entity in entities {

        let invalid_indx = entity.components.len();
        let mut position = Point2{x: 0.0, y: 0.0};
        let mut size = Vector2{x: 0.0, y: 0.0};
        let mut color = Vector3{x: 1.0, y: 1.0, z: 1.0};
        let mut texture = None;
        let mut has_drawable = false;

        for component in &entity.components {
            match component {
                Component::Drawable => {
                    has_drawable = true;
                },
                Component::Position(pos) => {
                    position = *pos;
                }
                Component::Size(s) => {
                    size = *s;
                }
                Component::Color(c) => {
                    color = *c;
                }
                Component::Texture(img) => {
                    texture = Some(img);
                }
                _ => {}
            }
        }
        if has_drawable {
            if let Some(img) = texture {
                renderer.add_texture(img);
            }
            renderer.add_quad(position, size, color);
        }
    }
}
