use std::{env, process};
use rust_cli::{Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let current_direc = env::current_dir()
        .expect("Unable to read current directory")
        .to_str().expect("Unable to open directory").to_string();

    println!("Searching for {}", config.query);
    println!("In file {}", config.filepath);
    println!("Current directory is {:?}", current_direc);

    if let Err(e) = rust_cli::run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }

}

