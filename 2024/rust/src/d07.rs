pub fn part1(input: &str) -> usize {
    let equations = parse(input);
    let ops = &[Operator::Add, Operator::Multiply];

    equations
        .iter()
        .filter(|eq| eq.is_valid(ops))
        .map(|eq| eq.result)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let equations = parse(input);
    let ops = &[Operator::Add, Operator::Multiply, Operator::Concat];

    equations
        .iter()
        .filter(|eq| eq.is_valid(ops))
        .map(|eq| eq.result)
        .sum()
}

fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .filter(|&line| !line.trim().is_empty())
        .map(|line| {
            let (result, numbers) = line.trim().split_once(':').unwrap();
            Equation {
                result: result.trim().parse().unwrap(),
                numbers: numbers
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Equation {
    result: usize,
    numbers: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

impl Operator {
    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concat => format!("{a}{b}").parse().unwrap(),
        }
    }
}

impl Equation {
    fn apply(&self, op: Operator) -> Option<Equation> {
        match self.numbers.as_slice() {
            [a, b, rest @ ..] => Some(Equation {
                result: self.result,
                numbers: vec![&[op.apply(*a, *b)], rest].concat(),
            }),
            _ => None,
        }
    }

    fn is_valid(&self, ops: &[Operator]) -> bool {
        match self.numbers.as_slice() {
            &[n] => self.result == n,
            &[n, ..] if self.result < n => false,
            _ => ops
                .iter()
                .any(|&op| self.apply(op).map_or(false, |eq| eq.is_valid(ops))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
        ";

        assert_eq!(part1(input), 3749);
    }

    #[test]
    fn test_part2() {
        let input = "
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
        ";

        assert_eq!(part2(input), 11387);
    }
}
