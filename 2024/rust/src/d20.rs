use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    sync::atomic::{AtomicUsize, Ordering},
};

type Coord = (isize, isize);
type Grid = HashMap<Coord, char>;

const DIRS: [Coord; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

static MIN_CHEAT_VALUE_PART1: AtomicUsize = AtomicUsize::new(100);
static MIN_CHEAT_VALUE_PART2: AtomicUsize = AtomicUsize::new(100);

fn parse(input: &str) -> (Grid, Coord, Coord) {
    let mut grid = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, line) in input.trim().lines().enumerate() {
        for (j, c) in line.trim().char_indices() {
            let coord = (i as isize, j as isize);
            grid.insert(coord, c);
            if c == 'S' {
                start = coord;
            } else if c == 'E' {
                end = coord;
            }
        }
    }
    (grid, start, end)
}

fn neighbors(coord: &Coord, grid: &Grid) -> Vec<Coord> {
    let (i, j) = *coord;
    DIRS.iter()
        .map(|(di, dj)| (i + di, j + dj))
        .filter(|c| grid.get(c) != Some(&'#'))
        .collect()
}

fn dijkstra(grid: &Grid, start: &Coord) -> HashMap<Coord, usize> {
    let mut visited: HashMap<Coord, usize> = HashMap::new();
    let mut min_heap: BinaryHeap<Reverse<(usize, Coord)>> = BinaryHeap::new();
    visited.insert(*start, 0);
    min_heap.push(Reverse((0, *start)));
    while let Some(Reverse((time, coord))) = min_heap.pop() {
        for neighbor in neighbors(&coord, grid) {
            let time = time + 1;
            if time < *visited.get(&neighbor).unwrap_or(&usize::MAX) {
                visited.insert(neighbor, time);
                min_heap.push(Reverse((time, neighbor)));
            }
        }
    }
    visited
}

fn find_cheats_2sec(times: &HashMap<Coord, usize>) -> HashMap<(Coord, Coord), usize> {
    times
        .iter()
        .flat_map(|(coord, time)| {
            let time = time + 2;
            DIRS.iter().filter_map(move |(di, dj)| {
                let (i, j) = coord;
                let coord1 = (i + di, j + dj);
                let coord2 = (i + di * 2, j + dj * 2);

                match (times.get(&coord1), times.get(&coord2)) {
                    (None, Some(time2)) if time2 > &time => Some(((*coord, coord2), time2 - time)),
                    _ => None,
                }
            })
        })
        .collect()
}

fn find_cheats_within(
    duration: isize,
    times: &HashMap<Coord, usize>,
) -> HashMap<(Coord, Coord), usize> {
    let mut cheats = HashMap::new();
    for (coord, time) in times {
        for di in -duration..=duration {
            let dj_duration = duration - di.abs();
            for dj in -dj_duration..=dj_duration {
                let (i, j) = coord;
                let cheat_end_coord = (i + di, j + dj);
                if !times.contains_key(&cheat_end_coord) {
                    continue;
                }

                let time = time + di.abs() as usize + dj.abs() as usize;
                let cheat_end_time = *times.get(&cheat_end_coord).unwrap();
                if cheat_end_time > time {
                    cheats.insert((*coord, cheat_end_coord), cheat_end_time - time);
                }
            }
        }
    }
    cheats
}

pub fn part1(input: &str) -> String {
    let (grid, start, _end) = parse(input);
    let min_cheat_value = MIN_CHEAT_VALUE_PART1.load(Ordering::Relaxed);

    let times = dijkstra(&grid, &start);

    find_cheats_2sec(&times)
        .iter()
        .filter(|(_coords, &time)| time >= min_cheat_value)
        .count()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let (grid, start, _end) = parse(input);
    let min_cheat_value = MIN_CHEAT_VALUE_PART2.load(Ordering::Relaxed);
    let max_cheat_duration = 20;

    let times = dijkstra(&grid, &start);

    find_cheats_within(max_cheat_duration, &times)
        .iter()
        .filter(|(_coords, &time)| time >= min_cheat_value)
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        MIN_CHEAT_VALUE_PART1.store(10, Ordering::Relaxed);
        let input = "
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
        ";
        assert_eq!(part1(input), "10");
    }

    #[test]
    fn test_part2() {
        MIN_CHEAT_VALUE_PART2.store(50, Ordering::Relaxed);
        let input = "
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
        ";
        assert_eq!(part2(input), "285");
    }
}
