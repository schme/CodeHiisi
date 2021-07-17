extern crate rand;

mod platform;
mod game;
mod renderer;
pub mod math;

use std::error::Error;

use game::Game;
use platform::Platform;
use renderer::Renderer;

#[derive(Debug)]
pub struct Config {
    pub program_name: String,
    pub asset_path: String,
    pub window_width: u32,
    pub window_height: u32,
}

impl Config {
    pub fn new(_args: &[String]) -> Result<Config, &str> {
        let program_name = "RustyRay".to_string();
        let asset_path = "assets".to_string();
        let window_width = 800;
        let window_height = 800;
        Ok(Config { program_name, asset_path, window_width, window_height })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let mut platform = Platform::new(&config.program_name, config.window_width, config.window_height);
    let mut renderer = Renderer::new(&mut platform.window);
    let mut game = Game::new().unwrap();

    let mut frame_stop: f64 = 0.0;
    let mut frame_start: f64 = 0.0;
    let mut frame_count = 0;

    while !platform.should_close() {
        frame_count += 1;
        let delta_time = frame_stop - frame_start;

        // Relaxed printing
        if frame_count % 60 == 0 {
            println!("delta_time: {:.5} -- fps: {:2.1} -- frame_count: {}", delta_time, 1.0/delta_time, frame_count);
        }

        frame_start = platform.get_time();

        platform.handle_events();

        let (cursor_x, cursor_y) = platform.get_cursor_pos();
        let mouse_state = platform.get_mouse_state();

        let frame_data = game::FrameData {
            delta_time,
            cursor_x,
            cursor_y,
            mouse_state,
        };

        game.update(frame_data, &mut renderer);

        renderer.render(&mut platform.window);

        platform.poll_events();
        frame_stop = platform.get_time();
    }

    println!("Main loop done!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thing() {
        assert!(true);
    }
}
