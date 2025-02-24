mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;
mod d21;
mod d22;

use std::env;

fn usage() -> ! {
    eprintln!("Usage: ./aoc2024 day <number> [part <number>]");
    std::process::exit(1);
}

fn solve(part: u32, part1_fn: fn(&str) -> String, part2_fn: fn(&str) -> String, input: &str) {
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
        7 => solve(
            part,
            d07::part1,
            d07::part2,
            include_str!("../../data/d07.txt"),
        ),
        8 => solve(
            part,
            d08::part1,
            d08::part2,
            include_str!("../../data/d08.txt"),
        ),
        9 => solve(
            part,
            d09::part1,
            d09::part2,
            include_str!("../../data/d09.txt"),
        ),
        10 => solve(
            part,
            d10::part1,
            d10::part2,
            include_str!("../../data/d10.txt"),
        ),
        11 => solve(
            part,
            d11::part1,
            d11::part2,
            include_str!("../../data/d11.txt"),
        ),
        12 => solve(
            part,
            d12::part1,
            d12::part2,
            include_str!("../../data/d12.txt"),
        ),
        13 => solve(
            part,
            d13::part1,
            d13::part2,
            include_str!("../../data/d13.txt"),
        ),
        14 => solve(
            part,
            d14::part1,
            d14::part2,
            include_str!("../../data/d14.txt"),
        ),
        15 => solve(
            part,
            d15::part1,
            d15::part2,
            include_str!("../../data/d15.txt"),
        ),
        16 => solve(
            part,
            d16::part1,
            d16::part2,
            include_str!("../../data/d16.txt"),
        ),
        17 => solve(
            part,
            d17::part1,
            d17::part2,
            include_str!("../../data/d17.txt"),
        ),
        18 => solve(
            part,
            d18::part1,
            d18::part2,
            include_str!("../../data/d18.txt"),
        ),
        19 => solve(
            part,
            d19::part1,
            d19::part2,
            include_str!("../../data/d19.txt"),
        ),
        20 => solve(
            part,
            d20::part1,
            d20::part2,
            include_str!("../../data/d20.txt"),
        ),
        21 => solve(
            part,
            d21::part1,
            d21::part2,
            include_str!("../../data/d21.txt"),
        ),
        22 => solve(
            part,
            d22::part1,
            d22::part2,
            include_str!("../../data/d22.txt"),
        ),
        _ => unimplemented!(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (day, part) = match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        &[_, "day", day] => (day, "0"),
        &[_, "day", day, "part", part] if part == "1" || part == "2" => (day, part),
        _ => usage(),
    };

    match (day.parse(), part.parse()) {
        (Ok(day), Ok(part)) => run(day, part),
        _ => usage(),
    }
}
