

use std::{
    error::Error,
    path::Path,
};

use crate::{
    *,
    assets::{TextureStorage},
};

use platform::Platform;
use game::Game;

#[derive(Debug)]
pub struct AppConfig {
    pub name: String,
    pub asset_path: String,
    pub window_size: (u32, u32),
}

#[derive(Debug)]
pub struct App {
    config: AppConfig,
}


impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            name: "Hiisi Engine".to_string(),
            asset_path: "../assets/".to_string(),
            window_size: (800, 600),
        }
    }
}


impl App {
    pub fn new(config: AppConfig) -> Self {
        App { config }
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {

        let mut platform = Platform::new(&self.config.name, self.config.window_size);
        let mut textures = TextureStorage::new();

        let asset_path = &self.config.asset_path;

        let texture_path = Path::new(&asset_path).join("textures");
        let audio_path = Path::new(&asset_path).join("audio");

        textures.load_textures_from_path(texture_path)?;

        let mut game = Game::new(platform.window.render_context(), &textures, &audio_path)?;

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

            let window_size = platform.window.get_size();

            let frame_data = game::FrameData {
                delta_time,
                window_size,
                cursor_x,
                cursor_y,
                mouse_state,
            };

            game.update(frame_data);

            platform.poll_events();
            frame_stop = platform.get_time();
        }

        println!("Main loop done!");

        Ok(())
    }
}
