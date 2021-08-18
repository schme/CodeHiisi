extern crate log;
extern crate rand;

pub mod components;
pub mod systems;

use std::{fs::File, process};

use rand::Rng;

use hiisi::{
    app::{App, AppConfig},
    components::*,
    ecs::{World, WorldExt},
    game::UpdatePosition,
    math::*,
    prelude::*,
    renderer::components::*,
};

use {components::*, systems::*};

fn setup_logging() {
    use simplelog::{
        ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, TermLogger, TerminalMode,
        ThreadLogMode, WriteLogger,
    };

    let config = ConfigBuilder::new()
        .set_thread_mode(ThreadLogMode::Names)
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
    }

    world
        .create_entity()
        .with(Position(Point2::new(100.0, 100.0)))
        .with(Size(Vector2::new(200.0, 200.0)))
        .with(Texture("kivi.png".to_string()))
        .with(Color(Vector3::new(1.0, 1.0, 1.0)))
        .build();

    let mut rng = rand::thread_rng();
    for _ in 1..10_000 {
        let pos = Position(Point2::new(
            rng.gen_range(0.0..800.0),
            rng.gen_range(0.0..800.0),
        ));
        let vel = Velocity(Vector2::new(
            rng.gen_range(-10.0..10.0),
            rng.gen_range(-10.0..10.0),
        ));
        let color = Color(Vector3::new(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        ));
        let size = Size(Vector2::new(20.0, 40.0));
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
        .with(FollowMouse, "following_mouse", &[])
        .with(Repelled, "repelled", &[])
        .with(
            UpdatePosition,
            "update_position",
            &["following_mouse", "repelled"],
        )
        .build(world);

    if let Err(e) = app.run() {
        println!("Application error: {}", e);
        process::exit(1);
    };

    Ok(())
}
