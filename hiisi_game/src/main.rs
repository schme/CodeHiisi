use std::{
    env,
    process,
};

use hiisi::{
    prelude::*,
    app::{App, AppConfig},
    ecs::{World, WorldExt},
};

fn main() -> std::io::Result<()> {

    let config = AppConfig {
        name: "Hiisi Game".to_string(),
        window_size: (800, 800),
    };


    let mut world = World::new();

    let mut app = App::builder(config)
        .with_plugin(&mut world, CorePlugin)
        .build(world);

    if let Err(e) = app.run() {
        println!("Application error: {}", e);
        process::exit(1);
    };

    Ok(())
}

