use std::collections::HashMap;

fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .lines()
        .flat_map(|line| line.trim().split_whitespace().map(|s| s.parse().unwrap()))
        .collect()
}

fn blink(number: usize) -> Vec<usize> {
    match number {
        0 => vec![1],
        _ if number.to_string().len() % 2 == 0 => {
            let str = number.to_string();
            let (a, b) = str.split_at(str.len() / 2);
            vec![a, b].iter().map(|s| s.parse().unwrap()).collect()
        }
        _ => vec![number * 2024],
    }
}

fn count_length(
    number: usize,
    blink_count: usize,
    max_blink_count: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if blink_count == max_blink_count {
        return 1;
    }

    if let Some(&length) = memo.get(&(number, blink_count)) {
        return length;
    }

    let next_numbers = blink(number);
    let length = next_numbers
        .iter()
        .map(|n| count_length(*n, blink_count + 1, max_blink_count, memo))
        .sum();
    memo.insert((number, blink_count), length);
    length
}

pub fn part1(input: &str) -> String {
    let numbers = parse(input);

    let mut memo = HashMap::new();
    numbers
        .iter()
        .map(|n| count_length(*n, 0, 25, &mut memo))
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let numbers = parse(input);

    let mut memo = HashMap::new();
    numbers
        .iter()
        .map(|n| count_length(*n, 0, 75, &mut memo))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "125 17";
        assert_eq!(part1(input), "55312");
    }

    #[test]
    fn test_part2() {
        let input = "125 17";
        assert_eq!(part2(input), "65601038650482");
    }
}
