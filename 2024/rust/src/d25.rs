const PINS: usize = 5;

type Lock = [usize; PINS];
type Key = [usize; PINS];

fn parse(input: &str) -> (usize, Vec<Lock>, Vec<Key>) {
    let max_height = input.trim().split("\n\n").next().unwrap().lines().count() - 2;
    let mut locks = vec![];
    let mut keys = vec![];
    input.trim().split("\n\n").for_each(|schema| {
        let mut pin_heights = [0; PINS];
        schema.trim().lines().for_each(|line| {
            line.trim().char_indices().for_each(|(i, c)| {
                if c == '#' {
                    pin_heights[i] += 1;
                }
            });
        });
        pin_heights = pin_heights.map(|v| v - 1);

        if schema.trim().lines().next().unwrap().trim() == "#".repeat(PINS) {
            locks.push(pin_heights);
        } else {
            keys.push(pin_heights);
        }
    });
    (max_height, locks, keys)
}

fn fits(max_height: usize, lock: &Lock, key: &Key) -> bool {
    for i in 0..PINS {
        if lock[i] + key[i] > max_height {
            return false;
        }
    }
    true
}

pub fn part1(input: &str) -> String {
    let (max_height, locks, keys) = parse(input);
    locks
        .iter()
        .map(|lock| {
            keys.iter()
                .filter(|key| fits(max_height, lock, key))
                .count()
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(_input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
        #####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####
        ";
        assert_eq!(part1(input), "3");
    }
}
