use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filepath = &args[2];
    let current_direc = env::current_dir()
        .expect("Unable to read current directory")
        .to_str().expect("Unable to open directory").to_string();

    println!("Searching for {}", query);
    println!("In file {}", filepath);
    println!("Current directory is {:?}", current_direc);

    let contents = fs::read_to_string(filepath)
        .expect("Should have been able to read file");

    println!("With text:\n{contents}");
}
