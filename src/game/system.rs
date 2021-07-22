use {
    game::{
        entity::{Entity},
        component::{self, OldComponent},
    },
    engine::{
        ecs::{
            system::System,
            storage::UncheckedStorage,
        },
        math::{self,Point2, Vector2, Vector3, MetricSpace},
        renderer::{Renderer},
        platform::Action,
    },
};

//struct Moving;
//struct MovingData {
    //frame_data: &super::FrameData,
    //position: &mut UncheckedStorage<Position>,
    //velocity: &UncheckedStorage<Velocity>,
//}

//impl System for Moving {
    //pub fn run(&mut self, data: MovingData) {
    //}
//}

pub fn moving(entities : &mut Vec<Entity>, frame_data : &super::FrameData) {
    for entity in entities {

        let invalid_indx = entity.components.len();
        let mut position_component_indx = invalid_indx;
        let mut velocity_component_indx = invalid_indx;

        for (i, component) in entity.components.iter().enumerate() {
            match component {
                OldComponent::Velocity(_) => {
                    velocity_component_indx = i;
                },
                OldComponent::Position(_) => {
                    position_component_indx = i;
                }
                _ => {}
            }
        }
        if velocity_component_indx != invalid_indx && position_component_indx != invalid_indx {
            let prev_position = &entity.components[position_component_indx];
            let velocity = &entity.components[velocity_component_indx];

            if let OldComponent::Position(pos) = prev_position {
                if let OldComponent::Velocity(vel) = velocity {
                    let new_point = pos + vel * frame_data.delta_time as f32;
                    entity.components[position_component_indx] = OldComponent::Position(new_point);
                }
            }
        }
    }
}

pub fn mouse_follow(entities : &mut Vec<Entity>, frame_data : &super::FrameData ) {
    for entity in entities {

        let mouse_pos = Point2{x: frame_data.cursor_x as f32, y: frame_data.cursor_y as f32};
        let invalid_indx = entity.components.len();
        let mut position_component_indx = invalid_indx;
        let mut has_follow_mouse = false;
        let mut size = Vector2{x: 0.0, y: 0.0};

        for (i, component) in entity.components.iter().enumerate() {
            match component {
                OldComponent::FollowMouse => {
                    has_follow_mouse = true;
                },
                OldComponent::Position(_) => {
                    position_component_indx = i;
                }
                OldComponent::Size(s) => {
                    size = *s;
                }
                _ => {}
            }
        }
        if has_follow_mouse && position_component_indx != invalid_indx {
            entity.components[position_component_indx] = OldComponent::Position(mouse_pos - size * 0.5);
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
                OldComponent::Velocity(_) => {
                    velocity_component_indx = i;
                },
                OldComponent::Position(_) => {
                    position_component_indx = i;
                },
                OldComponent::Repelled => {
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

            if let OldComponent::Position(pos) = prev_position {
                if let OldComponent::Velocity(vel) = velocity {
                    // Here
                    let dir = pos - mouse_pos;

                    match frame_data.mouse_state.left {
                        Action::Press => {
                            if pos.distance2(mouse_pos) < attract_radius2 {
                                let new_vel = - dir / 5.0;
                                entity.components[velocity_component_indx] = OldComponent::Velocity(new_vel);
                            }
                        }
                        Action::Release => {
                            if pos.distance2(mouse_pos) < repel_radius2 {
                                let new_vel = vel + dir;
                                entity.components[velocity_component_indx] = OldComponent::Velocity(new_vel);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

pub fn drawable(entities : Vec<Entity>, renderer : &mut Renderer) {
    for entity in entities {

        let mut position = Point2{x: 0.0, y: 0.0};
        let mut size = Vector2{x: 0.0, y: 0.0};
        let mut color = Vector3{x: 1.0, y: 1.0, z: 1.0};
        let mut texture = None;
        let mut has_drawable = false;

        for component in &entity.components {
            match component {
                OldComponent::Drawable => {
                    has_drawable = true;
                },
                OldComponent::Position(pos) => {
                    position = *pos;
                },
                OldComponent::Size(s) => {
                    size = *s;
                },
                OldComponent::Color(c) => {
                    color = *c;
                },
                OldComponent::Texture(txtr) => {
                    texture = Some(txtr);
                },
                _ => {}
            }
        }
        if has_drawable {
            renderer.add_quad(position, size, color, {
                if let Some(txtr) = texture {
                    renderer.get_texture_id(txtr)
                }
                else {
                    renderer.get_white_id()
                }
            });
        }
    }
}
