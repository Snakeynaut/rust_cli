use std::{env, process};
use rust_cli::{Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // let current_direc = env::current_dir()
    //     .expect("Unable to read current directory")
    //     .to_str().expect("Unable to open directory").to_string();

    if let Err(e) = rust_cli::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }

}

