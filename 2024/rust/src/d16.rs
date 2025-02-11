use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

use itertools::Itertools;

type Coord = (isize, isize);
type Dir = Coord;
type Map = HashMap<Coord, char>;

const DIRS: [Dir; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Reindeer {
    pos: Coord,
    dir: Dir,
    can_turn: bool,
}

impl Reindeer {
    fn forward(&self) -> Self {
        Reindeer {
            pos: (self.pos.0 + self.dir.0, self.pos.1 + self.dir.1),
            dir: self.dir,
            can_turn: true,
        }
    }

    fn cw(&self) -> Self {
        Reindeer {
            pos: self.pos,
            dir: match self.dir {
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                (-1, 0) => (0, 1),
                _ => unreachable!(),
            },
            can_turn: false,
        }
    }

    fn ccw(&self) -> Self {
        Reindeer {
            pos: self.pos,
            dir: match self.dir {
                (0, 1) => (-1, 0),
                (1, 0) => (0, 1),
                (0, -1) => (1, 0),
                (-1, 0) => (0, -1),
                _ => unreachable!(),
            },
            can_turn: false,
        }
    }
}

fn parse(input: &str) -> (Map, Reindeer, Vec<Reindeer>) {
    let mut map = HashMap::new();
    let mut start = Coord::default();
    let mut end = Coord::default();
    input.trim().lines().enumerate().for_each(|(i, line)| {
        line.trim().char_indices().for_each(|(j, c)| {
            map.insert((i as isize, j as isize), c);
            if c == 'S' {
                start = (i as isize, j as isize);
            } else if c == 'E' {
                end = (i as isize, j as isize);
            }
        });
    });

    let start = Reindeer {
        pos: start,
        dir: (0, 1),
        can_turn: true,
    };
    let ends = DIRS
        .into_iter()
        .map(|dir| Reindeer {
            pos: end,
            dir,
            can_turn: true,
        })
        .collect();
    (map, start, ends)
}

fn neighbors(reindeer: Reindeer, map: &Map) -> Vec<(Reindeer, usize)> {
    let mut result = Vec::new();

    let forward = reindeer.forward();
    if *map.get(&forward.pos).unwrap_or(&'#') != '#' {
        result.push((forward, 1));
    }

    if reindeer.can_turn {
        result.push((reindeer.cw(), 1000));
        result.push((reindeer.ccw(), 1000));
    }
    result
}

fn dijkstra(
    map: &Map,
    start: Reindeer,
) -> (HashMap<Reindeer, usize>, HashMap<Reindeer, Vec<Reindeer>>) {
    let mut costs = HashMap::new();
    let mut trace = HashMap::new();
    let mut min_heap = BinaryHeap::new();
    costs.insert(start, 0);
    min_heap.push(Reverse((0, start)));
    while let Some(Reverse((cost, reindeer))) = min_heap.pop() {
        if cost > costs[&reindeer] {
            continue;
        }
        for (neighbor, step_cost) in neighbors(reindeer, map) {
            let cost = cost + step_cost;
            if cost < *costs.get(&neighbor).unwrap_or(&usize::MAX) {
                costs.insert(neighbor, cost);
                trace.insert(neighbor, vec![reindeer]);
                min_heap.push(Reverse((cost, neighbor)));
            } else if cost == *costs.get(&neighbor).unwrap() {
                trace.entry(neighbor).or_default().push(reindeer);
            }
        }
    }
    (costs, trace)
}

pub fn part1(input: &str) -> usize {
    let (map, start, ends) = parse(input);
    let (costs, _trace) = dijkstra(&map, start);
    let min_cost = *ends.iter().filter_map(|end| costs.get(end)).min().unwrap();
    min_cost
}

pub fn part2(input: &str) -> usize {
    let (map, start, ends) = parse(input);
    let (costs, trace) = dijkstra(&map, start);

    let min_cost = *ends.iter().filter_map(|end| costs.get(end)).min().unwrap();
    let min_cost_ends = ends
        .iter()
        .filter(|end| costs.get(end).filter(|&c| *c == min_cost).is_some())
        .collect_vec();

    let mut coords = HashSet::new();
    let mut pool = Vec::from(min_cost_ends);
    while let Some(reindeer) = pool.pop() {
        coords.insert(reindeer.pos);
        if let Some(prev) = trace.get(&reindeer) {
            prev.iter().for_each(|prev| pool.push(prev));
        }
    }
    coords.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input1 = "
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
        ";
        assert_eq!(part1(input1), 7036);

        let input2 = "
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################
        ";
        assert_eq!(part1(input2), 11048);
    }

    #[test]
    fn test_part2() {
        let input1 = "
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
        ";
        assert_eq!(part2(input1), 45);

        let input2 = "
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################
        ";
        assert_eq!(part2(input2), 64);
    }
}
