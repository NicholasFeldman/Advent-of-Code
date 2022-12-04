use std::env;
use std::fs;

mod days;

pub fn read_file(folder: &str, name: &str) -> String {
    let cwd = env::current_dir().unwrap();
    let path = cwd.join("src").join(folder).join(name);

    let f = fs::read_to_string(path);
    f.expect("Failed to read file")
}

fn solve_day(day: u8) -> (u32, u32){
    let input = &read_file("inputs", &format!("day{:02}.txt", day));

    match day {
        1 => (days::day01::part_one(input), days::day01::part_two(input)),
        2 => (days::day02::part_one(input), days::day02::part_two(input)),
        3 => (days::day03::part_one(input), days::day03::part_two(input)),
        4 => (days::day04::part_one(input), days::day04::part_two(input)),
        _ => unimplemented!(),
    }
}

fn main() {
    println!("=== Day One ===");
    println!("Part One: {}. Part Two: {}", solve_day(1).0, solve_day(1).1);
    println!("=== Day Two ===");
    println!("Part One: {}. Part Two: {}", solve_day(2).0, solve_day(2).1);
    println!("=== Day Three ===");
    println!("Part One: {}. Part Two: {}", solve_day(3).0, solve_day(3).1);
    println!("=== Day Four ===");
    println!("Part One: {}. Part Two: {}", solve_day(4).0, solve_day(4).1);
}
