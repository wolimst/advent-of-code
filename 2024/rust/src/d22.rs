use std::iter;

use itertools::Itertools;

fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

fn next(mut n: usize) -> usize {
    n = mix(n, n * 64);
    n = prune(n);
    n = mix(n, n / 32);
    n = prune(n);
    n = mix(n, n * 2048);
    n = prune(n);
    n
}

fn mix(n: usize, a: usize) -> usize {
    n ^ a
}

fn prune(n: usize) -> usize {
    n % 16777216
}

pub fn part1(input: &str) -> String {
    let numbers = parse(input);
    numbers
        .into_iter()
        .map(|mut n| {
            for _ in 0..2000 {
                n = next(n);
            }
            n
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let numbers = parse(input);

    let prices_list: Vec<Vec<usize>> = numbers
        .into_iter()
        .map(|mut n| {
            iter::once(n % 10)
                .chain((1..2000).map(move |_| {
                    n = next(n);
                    n % 10
                }))
                .collect()
        })
        .collect();

    let diffs_list: Vec<Vec<isize>> = prices_list
        .iter()
        .map(|prices| {
            prices
                .windows(2)
                .map(|s| s[1] as isize - s[0] as isize)
                .collect()
        })
        .collect();

    prices_list
        .iter()
        .zip(diffs_list.iter())
        .flat_map(|(prices, diffs)| {
            diffs
                .windows(4)
                .zip(prices.iter().skip(4).cloned())
                .unique_by(|(diff_seq, _)| *diff_seq)
        })
        .into_grouping_map()
        .sum()
        .values()
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
        1
        10
        100
        2024
        ";
        assert_eq!(part1(input), "37327623");
    }

    #[test]
    fn test_part2() {
        let input = "
        1
        2
        3
        2024
        ";
        assert_eq!(part2(input), "23");
    }
}
