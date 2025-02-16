use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    sync::atomic::{AtomicIsize, AtomicUsize, Ordering},
};

use itertools::Itertools;

static X: AtomicIsize = AtomicIsize::new(71);
static Y: AtomicIsize = AtomicIsize::new(71);
static N_BLOCKS: AtomicUsize = AtomicUsize::new(1024);

type Coord = (isize, isize);
type Grid = HashMap<Coord, char>;

fn parse(input: &str) -> Vec<Coord> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.trim().parse().unwrap())
                .next_tuple()
                .unwrap()
        })
        .collect()
}

fn neighbors(coord: &Coord, grid: &Grid) -> Vec<Coord> {
    let (x, y) = *coord;
    [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)]
        .into_iter()
        .filter(|c| grid.get(c) == Some(&'.'))
        .collect()
}

fn dijkstra(grid: &Grid, start: Coord, end: Coord) -> Option<usize> {
    let mut steps: HashMap<Coord, usize> = HashMap::new();
    let mut min_heap: BinaryHeap<Reverse<(usize, Coord)>> = BinaryHeap::new();
    min_heap.push(Reverse((0, start)));

    while let Some(Reverse((step, coord))) = min_heap.pop() {
        if coord == end {
            return Some(step);
        }

        for neighbor in neighbors(&coord, grid) {
            let step = step + 1;
            if step < *steps.get(&neighbor).unwrap_or(&usize::MAX) {
                steps.insert(neighbor, step);
                min_heap.push(Reverse((step, neighbor)));
            }
        }
    }
    None
}

pub fn part1(input: &str) -> String {
    let start = (0, 0);
    let end = (X.load(Ordering::Relaxed) - 1, Y.load(Ordering::Relaxed) - 1);

    let mut grid = HashMap::new();
    for x in 0..X.load(Ordering::Relaxed) {
        for y in 0..Y.load(Ordering::Relaxed) {
            grid.insert((x, y), '.');
        }
    }

    let blocks = parse(input);
    for block in blocks.iter().take(N_BLOCKS.load(Ordering::Relaxed)) {
        grid.insert(*block, '#');
    }

    let steps = dijkstra(&grid, start, end);
    steps.unwrap().to_string()
}

pub fn part2(input: &str) -> String {
    let start = (0, 0);
    let end = (X.load(Ordering::Relaxed) - 1, Y.load(Ordering::Relaxed) - 1);

    let mut grid = HashMap::new();
    for x in 0..X.load(Ordering::Relaxed) {
        for y in 0..Y.load(Ordering::Relaxed) {
            grid.insert((x, y), '.');
        }
    }

    let blocks = parse(input);
    blocks
        .iter()
        .filter_map(|block| {
            grid.insert(*block, '#');
            match dijkstra(&grid, start, end) {
                Some(_) => None,
                None => Some(*block),
            }
        })
        .next()
        .map(|block| format!("{},{}", block.0, block.1))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        X.store(7, Ordering::Relaxed);
        Y.store(7, Ordering::Relaxed);
        N_BLOCKS.store(12, Ordering::Relaxed);
        let input = "
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
        ";
        assert_eq!(part1(input), "22");
    }

    #[test]
    fn test_part2() {
        X.store(7, Ordering::Relaxed);
        Y.store(7, Ordering::Relaxed);
        N_BLOCKS.store(12, Ordering::Relaxed);
        let input = "
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
        ";
        assert_eq!(part2(input), "6,1");
    }
}
