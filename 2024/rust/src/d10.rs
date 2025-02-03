use itertools::Itertools;
use std::collections::HashMap;

struct Map {
    size: (i64, i64),
    grid: HashMap<(i64, i64), i64>,
    trailheads: Vec<(i64, i64)>,
}

const DIRS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn parse(input: &str) -> Map {
    let lines: Vec<&str> = input.trim().lines().collect();
    let rows = lines.len();
    let cols = lines.get(0).unwrap().trim().len();

    let grid: HashMap<(i64, i64), i64> = lines
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.trim()
                .char_indices()
                .map(move |(j, char)| ((i as i64, j as i64), char.to_digit(10).unwrap() as i64))
        })
        .collect();

    let trailheads = grid
        .iter()
        .filter_map(|(k, v)| if *v == 0 { Some(*k) } else { None })
        .collect();

    Map {
        size: (rows as i64, cols as i64),
        grid,
        trailheads,
    }
}

fn hike(map: &Map, pos: (i64, i64)) -> Vec<(i64, i64)> {
    match map.grid.get(&pos) {
        None => vec![],
        Some(height) if *height == 9 => vec![pos],
        Some(height) => DIRS
            .iter()
            .map(|(di, dj)| (pos.0 + di, pos.1 + dj))
            .filter(|next_pos| {
                (0..map.size.0).contains(&next_pos.0) && (0..map.size.1).contains(&next_pos.1)
            })
            .filter(|next_pos| height + 1 == *map.grid.get(next_pos).unwrap())
            .flat_map(|next_pos| hike(map, next_pos))
            .collect(),
    }
}

pub fn part1(input: &str) -> i64 {
    let map = parse(input);

    map.trailheads
        .iter()
        .map(|trailhead| hike(&map, *trailhead))
        .map(|summits| summits.iter().unique().count() as i64)
        .sum::<i64>()
}

pub fn part2(input: &str) -> i64 {
    let map = parse(input);

    map.trailheads
        .iter()
        .map(|trailhead| hike(&map, *trailhead).len() as i64)
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
        ";
        assert_eq!(part1(input), 36);
    }

    #[test]
    fn test_part2() {
        let input = "
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
        ";
        assert_eq!(part2(input), 81);
    }
}
