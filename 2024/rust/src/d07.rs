#[derive(Debug)]
struct Equation {
    result: i64,
    numbers: Vec<i64>,
}

struct Operator {
    func: fn(&Equation) -> Equation,
    cond: fn(&Equation) -> bool,
}

impl Operator {
    fn apply(&self, eq: &Equation) -> Result<Equation, &'static str> {
        if (self.cond)(eq) {
            Ok((self.func)(eq))
        } else {
            Err("cannot apply operator to the equation")
        }
    }
}

const ADD: Operator = Operator {
    func: |eq| match eq.numbers.as_slice() {
        [first, second, tail @ ..] => Equation {
            result: eq.result,
            numbers: vec![&[first + second], tail].concat(),
        },
        _ => panic!("invalid equation"),
    },
    cond: |eq| eq.numbers.len() > 1,
};

const MUL: Operator = Operator {
    func: |eq| match eq.numbers.as_slice() {
        [first, second, tail @ ..] => Equation {
            result: eq.result,
            numbers: [&[first * second], tail].concat(),
        },
        _ => panic!("invalid equation"),
    },
    cond: |eq| eq.numbers.len() > 1,
};

const CONCAT: Operator = Operator {
    func: |eq| match eq.numbers.as_slice() {
        [first, second, tail @ ..] => Equation {
            result: eq.result,
            numbers: vec![&[format!("{first}{second}").parse().unwrap()], tail].concat(),
        },
        _ => panic!("invalid equation"),
    },
    cond: |eq| eq.numbers.len() > 1,
};

fn parse(input: &str) -> Vec<Equation> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (result_str, numbers_str) = line.split_once(':').unwrap();
            Equation {
                result: result_str.trim().parse().unwrap(),
                numbers: numbers_str
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

fn is_valid_equation(eq: &Equation, operators: &[Operator]) -> bool {
    match eq.numbers.as_slice() {
        [a] => eq.result == *a,
        _ => {
            for op in operators {
                if let Ok(next_eq) = op.apply(eq) {
                    if is_valid_equation(&next_eq, operators) {
                        return true;
                    }
                }
            }
            false
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let equations = parse(input);

    equations
        .iter()
        .filter(|&eq| is_valid_equation(eq, &[ADD, MUL]))
        .map(|eq| eq.result)
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let equations = parse(input);

    equations
        .iter()
        .filter(|&eq| is_valid_equation(eq, &[ADD, MUL, CONCAT]))
        .map(|eq| eq.result)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
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
    fn test_part2_example() {
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
