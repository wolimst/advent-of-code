use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> (HashSet<String>, Vec<String>) {
    let mut lines = input.trim().lines();

    let patterns: HashSet<String> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();

    let designs = lines
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect();

    (patterns, designs)
}

fn arrangements(
    design: &str,
    patterns: &HashSet<String>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if design == "" {
        return 1;
    }

    if let Some(&count) = memo.get(design) {
        return count;
    }

    let mut count = 0;
    for i in 0..design.len() {
        let pattern = &design[..=i];
        if patterns.contains(pattern) {
            count += arrangements(&design[i + 1..], patterns, memo)
        }
    }

    memo.insert(design.to_string(), count);
    count
}

pub fn part1(input: &str) -> String {
    let (patterns, designs) = parse(input);
    let mut memo = HashMap::new();
    designs
        .iter()
        .filter(|design| arrangements(design, &patterns, &mut memo) != 0)
        .count()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let (patterns, designs) = parse(input);
    let mut memo = HashMap::new();
    designs
        .iter()
        .map(|design| arrangements(design, &patterns, &mut memo))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
        ";
        assert_eq!(part1(input), "6");
    }

    #[test]
    fn test_part2() {
        let input = "
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
        ";
        assert_eq!(part2(input), "16");
    }
}
