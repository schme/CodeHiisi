use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = rustyray::Config::new(&args).unwrap_or_else(|err| {
        println!("Could not create config: {}", err);

        process::exit(1);
    });

    if let Err(e) = rustyray::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

