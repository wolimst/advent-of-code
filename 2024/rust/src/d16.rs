use std::collections::HashMap;

use itertools::Itertools;

type Coord = (isize, isize);
type Dir = Coord;
type Reindeer = (Coord, Dir);
type Map = HashMap<Coord, char>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Forward,
    Clockwise,
    CounterClockwise,
}

#[derive(Debug, Clone)]
struct Path {
    moves: Vec<Move>,
    score: usize,
}

impl Path {
    fn new(moves: Vec<Move>) -> Self {
        let score = moves
            .iter()
            .map(|m| match m {
                Move::Forward => 1,
                Move::Clockwise => 1000,
                Move::CounterClockwise => 1000,
            })
            .sum();
        Self { moves, score }
    }

    fn last_move(&self) -> &Move {
        self.moves.last().unwrap_or(&Move::Forward)
    }

    fn add(&self, m: Move) -> Self {
        let mut moves = self.moves.clone();
        moves.push(m);
        Path::new(moves)
    }

    fn to_reindeers(&self, start: &Reindeer) -> Vec<Reindeer> {
        let mut reindeer = start.clone();
        let mut reindeers = vec![reindeer];
        for m in self.moves.iter() {
            reindeer = next_reindeer(&reindeer, m);
            reindeers.push(reindeer);
        }
        reindeers
    }
}

const DIRS: [Coord; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn parse(input: &str) -> (Map, Reindeer, Coord) {
    let mut map = HashMap::new();
    let mut reindeer = (Coord::default(), (0, 1));
    let mut end = Coord::default();
    input.trim().lines().enumerate().for_each(|(i, line)| {
        line.trim().char_indices().for_each(|(j, c)| {
            map.insert((i as isize, j as isize), c);
            if c == 'S' {
                reindeer.0 = (i as isize, j as isize);
            } else if c == 'E' {
                end = (i as isize, j as isize);
            }
        });
    });
    (map, reindeer, end)
}

fn next_moves(last_move: &Move) -> Vec<Move> {
    match last_move {
        Move::Forward => vec![Move::Forward, Move::Clockwise, Move::CounterClockwise],
        Move::Clockwise => vec![Move::Forward],
        Move::CounterClockwise => vec![Move::Forward],
    }
}

fn next_reindeer(reindeer: &Reindeer, movement: &Move) -> Reindeer {
    let (pos, dir) = reindeer;
    match movement {
        Move::Forward => ((pos.0 + dir.0, pos.1 + dir.1), *dir),
        Move::Clockwise => (
            *pos,
            match dir {
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                (-1, 0) => (0, 1),
                _ => unreachable!(),
            },
        ),
        Move::CounterClockwise => (
            *pos,
            match dir {
                (0, 1) => (-1, 0),
                (1, 0) => (0, 1),
                (0, -1) => (1, 0),
                (-1, 0) => (0, -1),
                _ => unreachable!(),
            },
        ),
    }
}

fn escape_maze(
    map: &Map,
    reindeer: &Reindeer,
    path: Path,
    visited: &mut HashMap<Reindeer, Vec<Path>>,
) {
    let (pos, _dir) = reindeer;

    if map.get(pos).unwrap_or(&'#') == &'#' {
        return;
    }

    let prev_score = visited
        .get(reindeer)
        .and_then(|paths| paths.iter().map(|p| p.score).min());
    match prev_score {
        Some(prev_score) if path.score > prev_score => return,
        Some(prev_score) if path.score == prev_score => {
            visited.get_mut(reindeer).unwrap().push(path.clone());
        }
        _ => {
            visited.insert(*reindeer, vec![path.clone()]);
        }
    };

    if map.get(pos).unwrap() == &'E' {
        return;
    }

    let next_moves = next_moves(&path.last_move());
    for m in next_moves {
        let next_reindeer = next_reindeer(reindeer, &m);
        let next_path = path.add(m);
        escape_maze(map, &next_reindeer, next_path, visited);
    }
}

pub fn part1(input: &str) -> usize {
    let (map, reindeer, end) = parse(input);
    let mut visited = HashMap::new();
    escape_maze(&map, &reindeer, Path::new(vec![]), &mut visited);
    DIRS.iter()
        .map(|dir| (end, *dir))
        .filter_map(|end| visited.get(&end))
        .flat_map(|paths| paths.iter().map(|p| p.score))
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    let (map, reindeer, end) = parse(input);
    let mut visited = HashMap::new();
    escape_maze(&map, &reindeer, Path::new(vec![]), &mut visited);

    let paths = DIRS
        .iter()
        .map(|dir| (end, *dir))
        .filter_map(|end| visited.get(&end))
        .flat_map(|paths| paths.iter())
        .collect::<Vec<_>>();
    let min_score = paths.iter().map(|p| p.score).min().unwrap();
    paths
        .iter()
        .filter(|p| p.score == min_score)
        .flat_map(|p| p.to_reindeers(&reindeer))
        .map(|(coord, _dir)| coord)
        .unique()
        .count()
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
