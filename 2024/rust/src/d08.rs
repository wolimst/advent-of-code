use std::collections::HashMap;

use itertools::Itertools;

struct Map {
    size: (usize, usize),
    antennas: HashMap<char, Vec<(i64, i64)>>,
}

fn parse(input: &str) -> Map {
    let lines: Vec<&str> = input.trim().lines().collect();
    let rows = lines.len();
    let cols = lines.get(0).unwrap().len();
    let mut antennas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    lines.iter().enumerate().for_each(|(i, line)| {
        line.trim().char_indices().for_each(|(j, char)| {
            if char != '.' {
                antennas.entry(char).or_default().push((i as i64, j as i64))
            }
        })
    });

    Map {
        size: (rows, cols),
        antennas,
    }
}

pub fn part1(input: &str) -> usize {
    let map = parse(input);

    map.antennas
        .values()
        .flat_map(|freq_antennas| {
            freq_antennas.iter().permutations(2).map(|pair| {
                let ((i1, j1), (i2, j2)) = (pair[0], pair[1]);
                let (di, dj) = (i2 - i1, j2 - j1);
                (i2 + di, j2 + dj)
            })
        })
        .filter(|(i, j)| (0..map.size.0 as i64).contains(i) && (0..map.size.1 as i64).contains(j))
        .unique()
        .count()
}

pub fn part2(input: &str) -> usize {
    let map = parse(input);

    map.antennas
        .values()
        .flat_map(|freq_antennas| {
            freq_antennas.iter().permutations(2).flat_map(|pair| {
                let ((i1, j1), (i2, j2)) = (pair[0], pair[1]);
                let (di, dj) = (i2 - i1, j2 - j1);
                (0..)
                    .map(move |n| (i2 + n * di, j2 + n * dj))
                    .take_while(|(i, j)| {
                        (0..map.size.0 as i64).contains(i) && (0..map.size.1 as i64).contains(j)
                    })
            })
        })
        .unique()
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
        ";

        assert_eq!(part1(input), 14);
    }

    #[test]
    fn test_part2() {
        let input = "
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
        ";

        assert_eq!(part2(input), 34);
    }
}
