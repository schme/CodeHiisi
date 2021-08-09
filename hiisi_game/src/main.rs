use std::{
    env,
    process,
};

use hiisi::app::{App, AppConfig};

fn main() -> std::io::Result<()> {

    let current_dir = env::current_dir()?;

    let name = "Hiisi Game".to_string();
    let asset_path = String::from(current_dir.join("assets").to_str().expect("Could not parse current directory"));
    let window_size = (800, 800);

    let config = AppConfig {
        name, asset_path, window_size
    };

    let app = App::new(config);

    if let Err(e) = app.run() {
        println!("Application error: {}", e);
        process::exit(1);
    };

    Ok(())
}

