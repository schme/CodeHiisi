use std::vec;

use crate::renderer::{Renderer};
use crate::math::primitives::*;

pub fn draw_rectangle(rect : &Rect<f32>, renderer: &mut Renderer ) {
    let mut v = vec![
        rect.x, rect.y,
        rect.x, rect.y + rect.h,
        rect.x + rect.w, rect.y,
        rect.x, rect.y + rect.h,
        rect.x + rect.w, rect.y + rect.h,
        rect.x + rect.w, rect.y];
    renderer.add_to_buffer(&mut v);
}
