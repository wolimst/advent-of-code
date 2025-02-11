use std::collections::{HashMap, HashSet};

type Pos = (isize, isize);
type Region = HashSet<Pos>;

const DIRS: [Pos; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn parse(input: &str) -> Vec<Region> {
    let grid: HashMap<Pos, char> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.trim()
                .char_indices()
                .map(move |(j, c)| ((i as isize, j as isize), c))
        })
        .collect();

    let mut regions: Vec<Region> = Vec::new();
    let mut visited: HashSet<Pos> = HashSet::new();
    for (pos, char) in &grid {
        if visited.contains(&pos) {
            continue;
        }
        let mut region = HashSet::new();
        let mut queue = vec![*pos];
        while let Some(pos) = queue.pop() {
            if region.contains(&pos) {
                continue;
            }
            region.insert(pos);
            for dir in DIRS {
                let neighbor_pos = (pos.0 + dir.0, pos.1 + dir.1);
                if grid.get(&neighbor_pos).is_some_and(|c| c == char) {
                    queue.push(neighbor_pos);
                }
            }
        }
        visited.extend(region.iter());
        regions.push(region);
    }
    regions
}

fn get_area(region: &Region) -> usize {
    region.len()
}

fn get_perimeter(region: &Region) -> usize {
    region
        .iter()
        .map(|pos| {
            DIRS.iter()
                .map(|dir| (pos.0 + dir.0, pos.1 + dir.1))
                .filter(|neighbor_pos| region.contains(neighbor_pos))
                .count()
        })
        .map(|region_facing_sides| DIRS.len() - region_facing_sides)
        .sum()
}

fn get_sides(region: &Region) -> usize {
    let mut edges: HashMap<(isize, Pos), HashSet<isize>> = HashMap::new();
    for pos in region {
        for dir in DIRS {
            let neighbor_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if !region.contains(&neighbor_pos) {
                let (key, value) = if dir.0 == 0 {
                    (pos.1, pos.0)
                } else {
                    (pos.0, pos.1)
                };
                edges.entry((key, dir)).or_default().insert(value);
            }
        }
    }

    edges
        .values()
        .map(|indices| {
            indices
                .iter()
                .filter(|&index| !indices.contains(&(index + 1)))
                .count()
        })
        .sum()
}

pub fn part1(input: &str) -> String {
    let regions = parse(input);
    regions
        .iter()
        .map(|region| get_area(region) * get_perimeter(region))
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let regions = parse(input);
    regions
        .iter()
        .map(|region| get_area(region) * get_sides(region))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input1 = "
        AAAA
        BBCD
        BBCC
        EEEC
        ";
        assert_eq!(part1(input1), "140");

        let input2 = "
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO
        ";
        assert_eq!(part1(input2), "772");

        let input3 = "
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
        ";
        assert_eq!(part1(input3), "1930");
    }

    #[test]
    fn test_part2() {
        let input1 = "
        AAAA
        BBCD
        BBCC
        EEEC
        ";
        assert_eq!(part2(input1), "80");

        let input2 = "
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO
        ";
        assert_eq!(part2(input2), "436");

        let input3 = "
        EEEEE
        EXXXX
        EEEEE
        EXXXX
        EEEEE
        ";
        assert_eq!(part2(input3), "236");

        let input4 = "
        AAAAAA
        AAABBA
        AAABBA
        ABBAAA
        ABBAAA
        AAAAAA
        ";
        assert_eq!(part2(input4), "368");

        let input5 = "
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
        ";
        assert_eq!(part2(input5), "1206");
    }
}
