use std::env;

pub fn main() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
}
