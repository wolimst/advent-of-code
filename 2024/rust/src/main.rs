mod d07;

use std::env;

fn usage() -> !{
    eprintln!("Usage: ./aoc2024 day <number> [part <number>]");
    std::process::exit(1);
}

fn solve(part: u32, part1_fn: fn(&str) -> i64, part2_fn: fn(&str) -> i64, input: &str) {
    match part {
        1 => {
            let answer1 = part1_fn(input);
            println!("Part 1: {}", answer1);
        }
        2 => {
            let answer2 = part2_fn(input);
            println!("Part 2: {}", answer2);
        }
        _ => {
            let answer1 = part1_fn(input);
            println!("Part 1: {}", answer1);
            let answer2 = part2_fn(input);
            println!("Part 2: {}", answer2);
        }
    }
}

fn run(day: u32, part: u32) {
    match day {
        _ => unimplemented!(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (day, part) = match args.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
        &[_, "day", day] => (day, "0"),
        &[_, "day", day, "part", part] if part == "1" || part == "2" => (day, part),
        _ => usage(),
    };

    match (day.parse(), part.parse()) {
        (Ok(day), Ok(part)) => run(day, part),
        _ => usage(),
    }
}
