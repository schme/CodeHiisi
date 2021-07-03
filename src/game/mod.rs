use std::error::Error;

use super::math::{Rect};
use super::renderer::primitives::*;

pub struct FrameData {
    pub delta_time: f64,
    pub cursor_x: f64,
    pub cursor_y: f64,
}

pub struct Game {
    window_width: u32,
    window_height: u32,
}

pub fn new(window_width: u32, window_height: u32) -> Result<Game, Box<dyn Error>> {

    Ok(Game { window_width, window_height })
}

pub fn update(data: FrameData, renderer : &mut super::renderer::Renderer) {
    let object = Rect::new(data.cursor_x as f32, data.cursor_y as f32, 10.0, 10.0);
    //let object = Rect::new(400.0, 300.0, 100.0, 100.);

    draw_rectangle(&object, renderer);
}
