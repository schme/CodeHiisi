extern crate log;
extern crate rand;

pub mod components;
pub mod systems;
mod controller;

use std::{fs::File, process};

use rand::Rng;

use hiisi::{
    app::{App, AppConfig},
    components::*,
    ecs::{World, WorldExt},
    prelude::*,
    renderer::components::*,
    collision::CollisionPlugin,
};

use {
    components::*,
    systems::*,
    controller::*,
};


fn setup_logging() {
    use simplelog::{
        ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, TermLogger, TerminalMode,
        WriteLogger,
    };

    let config = ConfigBuilder::new()
        .set_time_to_local(true)
        .build();

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            config,
            File::create("hiisi_game.log").unwrap(),
        ),
    ])
    .unwrap();
}


fn main() -> std::io::Result<()> {
    setup_logging();

    let config = AppConfig {
        name: "Hiisi Game".to_string(),
        window_size: (800, 800),
        input_map: "input_map.ron".to_string(),
    };

    log::info!("Starting {}", &config.name);

    let mut world = World::new();

    world.register::<FollowingMouse>();

    {
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Size>();
        world.register::<Texture>();
        world.register::<Color>();
        world.register::<PlayerController>();
    }

    world
        .create_entity()
        .with(Position::new(100.0, 100.0))
        .with(Size::new(200.0, 200.0))
        .with(Texture("kivi.png".to_string()))
        .with(Color(1.0, 1.0, 1.0))
        .build();

    world
        .create_entity()
        .with(PlayerController::with_speed(100.0))
        .with(Position::new(300.0, 300.0))
        .with(Velocity::new(0.0, 0.0))
        .with(Size::new(32.0, 48.0))
        .with(Texture("white.png".to_string()))
        .with(Color(0.0, 1.0, 1.0))
        .build();

    let mut rng = rand::thread_rng();
    for _ in 1..10_000 {
        let pos = Position::new(
            rng.gen_range(0.0..800.0),
            rng.gen_range(0.0..800.0),
        );
        let vel = Velocity::new(
            rng.gen_range(-10.0..10.0),
            rng.gen_range(-10.0..10.0),
        );
        let color = Color(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        );
        let size = Size::new(20.0, 40.0);
        let texture = Texture("auringonkukka.png".to_string());
        world
            .create_entity()
            .with(pos)
            .with(vel)
            .with(size)
            .with(texture)
            .with(color)
            .build();
    }
    let app = App::builder(config)
        .with_plugin(&mut world, CorePlugin)
        .with_plugin(&mut world, PlayerControlPlugin)
        .with_plugin(&mut world, CollisionPlugin)
        .with(
            UpdatePosition,
            "update_position",
            &[],
        )
        .with(
            FollowMouse,
            "follow mouse",
            &[],
        )
        .with(
            Repelled,
            "repel mouse",
            &[],
        )
        .build(world);

    if let Err(e) = app.run() {
        log::error!("Application error: {}", e);
        process::exit(1);
    };

    Ok(())
}
