use lib_rs::utils::read_to_string;
use lib_rs::{run_base, Part};
use std::env;

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 3 {
        panic!("Usage: cargo run <year> <day> <bigboy>");
    }

    let year: u32 = argv[1].parse().expect("Year must be a number");
    let day: u8 = argv[2].parse().expect("Day must be a number");
    let bigboy = argv.get(3).is_some();
    println!("Running year {year}, day {day}");

    let input = read_to_string(year, day, bigboy);
    println!(
        "Part 1: \n{}",
        run_base(year as usize, day as usize, Part::First, &input)
    );
    println!(
        "Part 2: \n{}",
        run_base(year as usize, day as usize, Part::Second, &input)
    );
}
