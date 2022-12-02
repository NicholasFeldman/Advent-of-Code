use std::env;
use std::fs;

mod days;

pub fn read_file(folder: &str, name: &str) -> String {
    let cwd = env::current_dir().unwrap();
    let path = cwd.join("src").join(folder).join(name);

    let f = fs::read_to_string(path);
    f.expect("Failed to read file")
}

fn main() {
    println!("=== Day One ===");
    println!("{}", days::day01::part_one(&read_file("inputs", "day01.txt")));
    println!("{}", days::day01::part_two(&read_file("inputs", "day01.txt")));
}
