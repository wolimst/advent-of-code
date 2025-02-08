use std::collections::HashSet;

type Coord = (isize, isize);

mod part1 {
    use super::*;

    pub struct Map {
        robot: Coord,
        boxes: HashSet<Coord>,
        walls: HashSet<Coord>,
    }

    impl Map {
        pub fn boxes_gps(&self) -> HashSet<usize> {
            self.boxes
                .iter()
                .map(|(i, j)| (i * 100 + j) as usize)
                .collect()
        }

        pub fn move_robot(&mut self, dir: Coord) {
            if let Some(coords) = self._move_check(self.robot, dir) {
                self.robot = (self.robot.0 + dir.0, self.robot.1 + dir.1);
                self.boxes.retain(|coord| !coords.contains(coord));
                coords.iter().for_each(|coord| {
                    self.boxes.insert((coord.0 + dir.0, coord.1 + dir.1));
                });
            }
        }

        fn _move_check(&self, pos: Coord, dir: Coord) -> Option<HashSet<Coord>> {
            let next_coord = (pos.0 + dir.0, pos.1 + dir.1);

            if self.walls.contains(&next_coord) {
                return None;
            }

            if self.boxes.contains(&next_coord) {
                return self._move_check(next_coord, dir).map(|mut set| {
                    set.insert(next_coord);
                    set
                });
            }

            Some(HashSet::new())
        }
    }

    pub fn parse(input: &str) -> (Map, Vec<Coord>) {
        let (input_map, input_dirs) = input.trim().split_once("\n\n").unwrap();

        let mut robot = (0, 0);
        let mut boxes = HashSet::new();
        let mut walls = HashSet::new();
        input_map
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .map(move |(j, c)| ((i as isize, j as isize), c))
            })
            .for_each(|((i, j), c)| {
                if c == '@' {
                    robot = (i, j);
                } else if c == 'O' {
                    boxes.insert((i, j));
                } else if c == '#' {
                    walls.insert((i, j));
                };
            });

        let dirs = input_dirs
            .trim()
            .lines()
            .flat_map(|line| line.trim().chars())
            .map(|c| match c {
                '<' => (0, -1),
                '>' => (0, 1),
                '^' => (-1, 0),
                'v' => (1, 0),
                _ => unreachable!(),
            })
            .collect();

        (
            Map {
                robot,
                boxes,
                walls,
            },
            dirs,
        )
    }
}

pub fn part1(input: &str) -> usize {
    let (mut map, dirs) = part1::parse(input);

    for dir in dirs {
        map.move_robot(dir);
    }

    map.boxes_gps().iter().sum()
}

mod part2 {
    use super::*;

    type Box = (Coord, Coord);

    pub struct Map {
        robot: Coord,
        boxes: HashSet<Box>,
        walls: HashSet<Coord>,
    }

    impl Map {
        pub fn boxes_gps(&self) -> HashSet<usize> {
            self.boxes
                .iter()
                .map(|((i, j), _)| (i * 100 + j) as usize)
                .collect()
        }

        pub fn move_robot(&mut self, dir: &Coord) {
            if let Some(boxes) = self._move_check(&self.robot, dir) {
                self.robot = (self.robot.0 + dir.0, self.robot.1 + dir.1);
                self.boxes.retain(|b| !boxes.contains(b));
                boxes.iter().for_each(|((i1, j1), (i2, j2))| {
                    self.boxes
                        .insert(((i1 + dir.0, j1 + dir.1), (i2 + dir.0, j2 + dir.1)));
                });
            }
        }

        fn _move_check(&self, coord: &Coord, dir: &Coord) -> Option<HashSet<Box>> {
            let next_coord = (coord.0 + dir.0, coord.1 + dir.1);

            if self.walls.contains(&next_coord) {
                return None;
            }

            if let Some(b) = self._get_box(&next_coord) {
                let edge_coords = self._get_edge_coords(b, dir);
                return edge_coords
                    .into_iter()
                    .map(|c| self._move_check(&c, dir))
                    .reduce(|acc, boxes| {
                        if let (Some(mut acc), Some(boxes)) = (acc, boxes) {
                            acc.extend(boxes);
                            Some(acc)
                        } else {
                            None
                        }
                    })
                    .flatten()
                    .map(|mut boxes| {
                        boxes.insert(*b);
                        boxes
                    });
            }

            Some(HashSet::new())
        }

        fn _get_box(&self, &(i, j): &Coord) -> Option<&Box> {
            self.boxes
                .get(&((i, j), (i, j + 1)))
                .or_else(|| self.boxes.get(&((i, j - 1), (i, j))))
        }

        fn _get_edge_coords(&self, b: &Box, dir: &Coord) -> Vec<Coord> {
            let (left, right) = *b;
            if dir == &(0, 1) {
                vec![right]
            } else if dir == &(0, -1) {
                vec![left]
            } else {
                vec![left, right]
            }
        }
    }

    pub fn parse(input: &str) -> (Map, Vec<Coord>) {
        let (input_map, input_dirs) = input.trim().split_once("\n\n").unwrap();

        let input_map_resized = input_map
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| match c {
                        '#' => "##",
                        'O' => "[]",
                        '.' => "..",
                        '@' => "@.",
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n");

        let mut robot = (0, 0);
        let mut boxes = HashSet::new();
        let mut walls = HashSet::new();
        input_map_resized
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .map(move |(j, c)| ((i as isize, j as isize), c))
            })
            .for_each(|((i, j), c)| {
                if c == '@' {
                    robot = (i, j);
                } else if c == '[' {
                    boxes.insert(((i, j), (i, j + 1)));
                } else if c == '#' {
                    walls.insert((i, j));
                };
            });

        let dirs = input_dirs
            .trim()
            .lines()
            .flat_map(|line| line.trim().chars())
            .map(|c| match c {
                '<' => (0, -1),
                '>' => (0, 1),
                '^' => (-1, 0),
                'v' => (1, 0),
                _ => unreachable!(),
            })
            .collect();

        (
            Map {
                robot,
                boxes,
                walls,
            },
            dirs,
        )
    }
}

pub fn part2(input: &str) -> usize {
    let (mut map, dirs) = part2::parse(input);

    for dir in dirs {
        map.move_robot(&dir);
    }

    map.boxes_gps().iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input1 = "
        ########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########

        <^^>>>vv<v>>v<<
        ";
        assert_eq!(part1(input1), 2028);

        let input2 = "
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
        ";
        assert_eq!(part1(input2), 10092);
    }

    #[test]
    fn test_part2() {
        let input1 = "
        #######
        #...#.#
        #.....#
        #..OO@#
        #..O..#
        #.....#
        #######

        <vv<<^^<<^^
        ";
        assert_eq!(part2(input1), 618);

        let input2 = "
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
        ";
        assert_eq!(part2(input2), 9021);
    }
}
