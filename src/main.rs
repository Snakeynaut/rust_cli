use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filepath = &args[2];
    let current_direc = env::current_dir().expect("Unable to read current directory")
                                    .to_str().expect("Unable to open directory").to_string();

    println!("Searching for {}", query);
    println!("In file {}", filepath);
    println!("Current director is {:?}", current_direc);
}
