import gleam/int
import gleam/io
import gleam/string

import argv
import simplifile

fn usage() {
  panic as "usage: ./aoc2024 day <number> [part <number>]"
}

fn solve(part1_fn, part2_fn, part: Int, input: String) {
  case part {
    1 -> {
      let answer1 = part1_fn(input)
      io.println("Part 1: " <> int.to_string(answer1))
    }
    2 -> {
      let answer2 = part2_fn(input)
      io.println("Part 2: " <> int.to_string(answer2))
    }
    _ -> {
      let answer1 = part1_fn(input)
      io.println("Part 1: " <> int.to_string(answer1))
      let answer2 = part2_fn(input)
      io.println("Part 2: " <> int.to_string(answer2))
    }
  }
}

fn run(day: Int, part: Int) -> Nil {
  let day_str = int.to_string(day)
  io.println("Day " <> day_str)

  let path = "../data/d" <> string.pad_start(day_str, 2, "0") <> ".txt"
  let input = case simplifile.read(path) {
    Ok(i) -> i
    Error(_) -> panic as { "could not read file from \"" <> path <> "\"" }
  }

  case day {
    _ -> panic as "not implemented"
  }
}

pub fn main() {
  let #(day, part) = case argv.load().arguments {
    ["day", d] -> #(d, "0")
    ["day", d, "part", p] if p == "1" || p == "2" -> #(d, p)
    _ -> usage()
  }

  case int.parse(day), int.parse(part) {
    Ok(d), Ok(p) -> run(d, p)
    _, _ -> usage()
  }
}
