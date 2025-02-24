use std::collections::HashMap;

use itertools::Itertools;

type Code = String;
type Key = char;

fn parse(input: &str) -> Vec<Code> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().to_string())
        .collect()
}

fn arrow_pad_paths() -> HashMap<(Key, Key), Vec<Code>> {
    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    HashMap::from([
        (('^', '^'), vec!["A"]),
        (('^', 'A'), vec![">A"]),
        (('^', '<'), vec!["v<A"]),
        (('^', 'v'), vec!["vA"]),
        (('^', '>'), vec!["v>A"]),
        (('A', '^'), vec!["<A"]),
        (('A', 'A'), vec!["A"]),
        (('A', '<'), vec!["<v<A", "v<<A"]),
        (('A', 'v'), vec!["<vA", "v<A"]),
        (('A', '>'), vec!["vA"]),
        (('<', '^'), vec![">^A"]),
        (('<', 'A'), vec![">^>A", ">>^A"]),
        (('<', '<'), vec!["A"]),
        (('<', 'v'), vec![">A"]),
        (('<', '>'), vec![">>A"]),
        (('v', '^'), vec!["^A"]),
        (('v', 'A'), vec!["^>A", ">^A"]),
        (('v', '<'), vec!["<A"]),
        (('v', 'v'), vec!["A"]),
        (('v', '>'), vec![">A"]),
        (('>', '^'), vec!["<^A", "^<A"]),
        (('>', 'A'), vec!["^A"]),
        (('>', '<'), vec!["<<A"]),
        (('>', 'v'), vec!["<A"]),
        (('>', '>'), vec!["A"]),
    ])
    .iter()
    .map(|(k, v)| (*k, v.iter().map(|s| s.to_string()).collect()))
    .collect()
}

#[rustfmt::skip]
fn number_pad_paths() -> HashMap<(Key, Key), Vec<Code>> {
    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+
    HashMap::from([
        (('6', '8'), vec!["<^A", "^<A"]),
        (('4', 'A'), vec![">>vvA", "v>v>A", "v>>vA", ">vv>A", ">v>vA"]),
        (('2', '6'), vec![">^A", "^>A"]),
        (('4', '9'), vec!["^>>A", ">>^A", ">^>A"]),
        (('4', '6'), vec![">>A"]),
        (('9', '4'), vec!["<<vA", "v<<A", "<v<A"]),
        (('0', 'A'), vec![">A"]),
        (('2', '7'), vec!["^<^A", "^^<A", "<^^A"]),
        (('2', '3'), vec![">A"]),
        (('0', '3'), vec![">^A", "^>A"]),
        (('A', '3'), vec!["^A"]),
        (('5', '1'), vec!["v<A", "<vA"]),
        (('1', '5'), vec![">^A", "^>A"]),
        (('8', 'A'), vec!["vvv>A", "vv>vA", ">vvvA", "v>vvA"]),
        (('A', '5'), vec!["^<^A", "^^<A", "<^^A"]),
        (('8', '4'), vec!["<vA", "v<A"]),
        (('8', '9'), vec![">A"]),
        (('4', '7'), vec!["^A"]),
        (('1', '2'), vec![">A"]),
        (('3', '8'), vec!["^^<A", "^<^A", "<^^A"]),
        (('6', '2'), vec!["<vA", "v<A"]),
        (('9', 'A'), vec!["vvvA"]),
        (('A', '7'), vec!["<^<^^A", "<^^<^A", "^^<^<A", "^<^<^A", "^<<^^A", "^^<<^A", "<^^^<A", "^^^<<A", "^<^^<A"]),
        (('9', '9'), vec!["A"]),
        (('0', '8'), vec!["^^^A"]),
        (('2', '0'), vec!["vA"]),
        (('1', '0'), vec![">vA"]),
        (('7', '4'), vec!["vA"]),
        (('6', 'A'), vec!["vvA"]),
        (('9', '5'), vec!["v<A", "<vA"]),
        (('6', '4'), vec!["<<A"]),
        (('5', '2'), vec!["vA"]),
        (('4', '3'), vec!["v>>A", ">v>A", ">>vA"]),
        (('5', '5'), vec!["A"]),
        (('9', '6'), vec!["vA"]),
        (('3', '2'), vec!["<A"]),
        (('8', '2'), vec!["vvA"]),
        (('9', '8'), vec!["<A"]),
        (('A', 'A'), vec!["A"]),
        (('0', '1'), vec!["^<A"]),
        (('0', '0'), vec!["A"]),
        (('0', '5'), vec!["^^A"]),
        (('1', '9'), vec!["^^>>A", ">>^^A", "^>^>A", ">^^>A", "^>>^A", ">^>^A"]),
        (('3', '1'), vec!["<<A"]),
        (('8', '7'), vec!["<A"]),
        (('9', '3'), vec!["vvA"]),
        (('5', '4'), vec!["<A"]),
        (('1', '7'), vec!["^^A"]),
        (('3', '7'), vec!["<^<^A", "^<^<A", "^<<^A", "<<^^A", "<^^<A", "^^<<A"]),
        (('6', '0'), vec!["<vvA", "v<vA", "vv<A"]),
        (('9', '2'), vec!["v<vA", "vv<A", "<vvA"]),
        (('A', '0'), vec!["<A"]),
        (('1', 'A'), vec![">>vA", ">v>A"]),
        (('A', '8'), vec!["<^^^A", "^^^<A", "^^<^A", "^<^^A"]),
        (('4', '4'), vec!["A"]),
        (('7', 'A'), vec![">vv>vA", "v>vv>A", "vv>v>A", "vv>>vA", ">>vvvA", "v>v>vA", "v>>vvA", ">v>vvA", ">vvv>A"]),
        (('8', '6'), vec![">vA", "v>A"]),
        (('7', '0'), vec![">vvvA", "vv>vA", "v>vvA"]),
        (('7', '5'), vec!["v>A", ">vA"]),
        (('8', '1'), vec!["v<vA", "vv<A", "<vvA"]),
        (('4', '2'), vec!["v>A", ">vA"]),
        (('4', '5'), vec![">A"]),
        (('A', '6'), vec!["^^A"]),
        (('6', '7'), vec!["<<^A", "<^<A", "^<<A"]),
        (('9', '0'), vec!["vv<vA", "<vvvA", "v<vvA", "vvv<A"]),
        (('7', '2'), vec![">vvA", "vv>A", "v>vA"]),
        (('A', '2'), vec!["<^A", "^<A"]),
        (('2', '9'), vec!["^^>A", ">^^A", "^>^A"]),
        (('6', '6'), vec!["A"]),
        (('0', '9'), vec![">^^^A", "^^>^A", "^>^^A", "^^^>A"]),
        (('1', '1'), vec!["A"]),
        (('0', '2'), vec!["^A"]),
        (('0', '4'), vec!["^<^A", "^^<A"]),
        (('1', '3'), vec![">>A"]),
        (('6', '1'), vec!["<<vA", "<v<A", "v<<A"]),
        (('8', '0'), vec!["vvvA"]),
        (('5', '8'), vec!["^A"]),
        (('8', '3'), vec!["vv>A", "v>vA", ">vvA"]),
        (('3', '0'), vec!["<vA", "v<A"]),
        (('9', '7'), vec!["<<A"]),
        (('0', '6'), vec!["^^>A", ">^^A", "^>^A"]),
        (('2', '8'), vec!["^^A"]),
        (('4', '8'), vec!["^>A", ">^A"]),
        (('3', '4'), vec!["<<^A", "<^<A", "^<<A"]),
        (('9', '1'), vec!["v<v<A", "vv<<A", "v<<vA", "<v<vA", "<vv<A", "<<vvA"]),
        (('3', 'A'), vec!["vA"]),
        (('3', '9'), vec!["^^A"]),
        (('3', '3'), vec!["A"]),
        (('2', '4'), vec!["<^A", "^<A"]),
        (('6', '3'), vec!["vA"]),
        (('5', '9'), vec!["^>A", ">^A"]),
        (('A', '4'), vec!["^<^<A", "^<<^A", "<^^<A", "^^<<A", "<^<^A"]),
        (('0', '7'), vec!["^^^<A", "^<^^A", "^^<^A"]),
        (('2', 'A'), vec![">vA", "v>A"]),
        (('7', '3'), vec![">vv>A", "v>v>A", ">v>vA", "v>>vA", ">>vvA", "vv>>A"]),
        (('5', '6'), vec![">A"]),
        (('7', '7'), vec!["A"]),
        (('1', '8'), vec!["^>^A", "^^>A", ">^^A"]),
        (('7', '1'), vec!["vvA"]),
        (('4', '1'), vec!["vA"]),
        (('5', '3'), vec![">vA", "v>A"]),
        (('2', '5'), vec!["^A"]),
        (('5', '0'), vec!["vvA"]),
        (('8', '5'), vec!["vA"]),
        (('7', '9'), vec![">>A"]),
        (('3', '6'), vec!["^A"]),
        (('3', '5'), vec!["^<A", "<^A"]),
        (('1', '4'), vec!["^A"]),
        (('6', '5'), vec!["<A"]),
        (('5', 'A'), vec!["v>vA", "vv>A", ">vvA"]),
        (('A', '1'), vec!["^<<A", "<^<A"]),
        (('2', '2'), vec!["A"]),
        (('6', '9'), vec!["^A"]),
        (('4', '0'), vec!["v>vA", ">vvA"]),
        (('7', '8'), vec![">A"]),
        (('7', '6'), vec!["v>>A", ">v>A", ">>vA"]),
        (('A', '9'), vec!["^^^A"]),
        (('2', '1'), vec!["<A"]),
        (('1', '6'), vec!["^>>A", ">>^A", ">^>A"]),
        (('8', '8'), vec!["A"]),
        (('5', '7'), vec!["^<A", "<^A"]),
    ])
    .iter()
    .map(|(k, v)| (*k, v.iter().map(|s| s.to_string()).collect()))
    .collect()
}

fn next_button_sequences(code: &Code, paths: &HashMap<(Key, Key), Vec<Code>>) -> Vec<Code> {
    let mut seqs = vec![];
    let mut pos = 'A';
    for ch in code.chars() {
        seqs.push(paths.get(&(pos, ch)).unwrap());
        pos = ch;
    }
    seqs.into_iter()
        .multi_cartesian_product()
        .map(|seqs| seqs.iter().join(""))
        .collect()
}

fn button_sequence_lengths(
    code: &Code,
    paths: &HashMap<(Key, Key), Vec<Code>>,
    depth: usize,
    memo: &mut HashMap<(Code, usize), usize>,
) -> usize {
    if depth == 0 {
        return code.len();
    }

    if let Some(seq_lens) = memo.get(&(code.clone(), depth)) {
        return seq_lens.clone();
    }

    let mut seqs = vec![];
    let mut pos = 'A';
    for ch in code.chars() {
        seqs.push(paths.get(&(pos, ch)).unwrap());
        pos = ch;
    }
    let min_seq_len = seqs
        .into_iter()
        .multi_cartesian_product()
        .map(|seqs| {
            seqs.iter()
                .map(|seq| button_sequence_lengths(seq, paths, depth - 1, memo))
                .sum()
        })
        .min()
        .unwrap();
    memo.insert((code.clone(), depth), min_seq_len);
    min_seq_len
}

fn solve(codes: &Vec<Code>, n_robots: usize) -> usize {
    let number_pad_paths = number_pad_paths();
    let arrow_pad_paths = arrow_pad_paths();
    let mut memo = HashMap::new();
    codes
        .iter()
        .map(|code| {
            let num = code[..code.len() - 1].parse::<usize>().unwrap();
            let seqs = next_button_sequences(code, &number_pad_paths);
            let min_seq_len: usize = seqs
                .iter()
                .map(|seq| button_sequence_lengths(seq, &arrow_pad_paths, n_robots, &mut memo))
                .min()
                .unwrap();
            num * min_seq_len
        })
        .sum()
}

pub fn part1(input: &str) -> String {
    let codes = parse(input);
    let n_robots = 2;
    solve(&codes, n_robots).to_string()
}

pub fn part2(input: &str) -> String {
    let codes = parse(input);
    let n_robots = 25;
    solve(&codes, n_robots).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
        029A
        980A
        179A
        456A
        379A
        ";
        assert_eq!(part1(input), "126384");
    }
}
