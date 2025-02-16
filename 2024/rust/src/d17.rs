use itertools::Itertools;

#[derive(Debug, Clone)]
struct State {
    ra: u64,
    rb: u64,
    rc: u64,
    instruction_index: usize,
    output: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operend {
    Literal(u64),
    Combo(u64),
}

impl Operend {
    fn value(&self, state: &State) -> u64 {
        match self {
            Self::Literal(n) => *n,
            Self::Combo(n) => match n {
                n if (0..=3).contains(n) => *n,
                4 => state.ra,
                5 => state.rb,
                6 => state.rc,
                _ => unreachable!(),
            },
        }
    }

    fn raw_value(&self) -> u64 {
        match self {
            Self::Literal(n) => *n,
            Self::Combo(n) => *n,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Adv(Operend),
    Bxl(Operend),
    Bst(Operend),
    Jnz(Operend),
    Bxc(Operend),
    Out(Operend),
    Bdv(Operend),
    Cdv(Operend),
}

impl Instruction {
    fn new(opcode: u64, operand: u64) -> Self {
        match opcode {
            0 => Self::Adv(Operend::Combo(operand)),
            1 => Self::Bxl(Operend::Literal(operand)),
            2 => Self::Bst(Operend::Combo(operand)),
            3 => Self::Jnz(Operend::Literal(operand)),
            4 => Self::Bxc(Operend::Literal(operand)),
            5 => Self::Out(Operend::Combo(operand)),
            6 => Self::Bdv(Operend::Combo(operand)),
            7 => Self::Cdv(Operend::Combo(operand)),
            _ => unreachable!(),
        }
    }

    fn opcode(&self) -> u64 {
        match self {
            Self::Adv(_) => 0,
            Self::Bxl(_) => 1,
            Self::Bst(_) => 2,
            Self::Jnz(_) => 3,
            Self::Bxc(_) => 4,
            Self::Out(_) => 5,
            Self::Bdv(_) => 6,
            Self::Cdv(_) => 7,
        }
    }

    fn operand(&self) -> u64 {
        match self {
            Self::Adv(op) => op.raw_value(),
            Self::Bxl(op) => op.raw_value(),
            Self::Bst(op) => op.raw_value(),
            Self::Jnz(op) => op.raw_value(),
            Self::Bxc(op) => op.raw_value(),
            Self::Out(op) => op.raw_value(),
            Self::Bdv(op) => op.raw_value(),
            Self::Cdv(op) => op.raw_value(),
        }
    }

    fn execute(&self, state: &mut State) {
        let mut jumped = false;
        match self {
            Self::Adv(op) => {
                state.ra = state.ra / u64::pow(2, op.value(state) as u32);
            }
            Self::Bxl(op) => {
                state.rb = state.rb ^ op.value(state);
            }
            Self::Bst(op) => {
                state.rb = op.value(state) % 8;
            }
            Self::Jnz(op) => {
                if state.ra != 0 {
                    state.instruction_index = op.value(state) as usize;
                    jumped = true;
                }
            }
            Self::Bxc(_op) => {
                state.rb = state.rb ^ state.rc;
            }
            Self::Out(op) => {
                let value = op.value(state) % 8;
                state.output.push(value);
            }
            Self::Bdv(op) => {
                state.rb = state.ra / u64::pow(2, op.value(state) as u32);
            }
            Self::Cdv(op) => {
                state.rc = state.ra / u64::pow(2, op.value(state) as u32);
            }
        }

        if !jumped {
            state.instruction_index += 1;
        }
    }
}

fn parse(input: &str) -> (State, Vec<Instruction>) {
    let numbers: Vec<u64> = input
        .trim()
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    (
        State {
            ra: numbers[0],
            rb: numbers[1],
            rc: numbers[2],
            instruction_index: 0,
            output: Vec::new(),
        },
        numbers[3..]
            .chunks(2)
            .map(|n| Instruction::new(n[0], n[1]))
            .collect(),
    )
}

fn run(state: &mut State, instructions: &[Instruction]) {
    while state.instruction_index < instructions.len() {
        instructions[state.instruction_index].execute(state);
    }
}

pub fn part1(input: &str) -> String {
    let (mut state, instructions) = parse(input);
    run(&mut state, &instructions);
    state.output.iter().join(",")
}

fn get_program_value(instructions: &[Instruction], program_pointer: usize) -> Option<u64> {
    let instruction_index = program_pointer / 2;
    let instruction = instructions.get(instruction_index);
    instruction.map(|instruction| {
        if program_pointer % 2 == 0 {
            instruction.opcode()
        } else {
            instruction.operand()
        }
    })
}

fn find_self_duplication_ra(
    initial_state: &State,
    instructions: &[Instruction],
    ra: u64,
    pointer: usize,
) -> Option<u64> {
    for ra in ra..ra + 8 {
        let mut state = initial_state.clone();
        state.ra = ra;
        run(&mut state, &instructions);

        if state.output[0] != get_program_value(&instructions, pointer).unwrap() {
            continue;
        }

        if pointer == 0 {
            return Some(ra);
        }

        let result = find_self_duplication_ra(initial_state, instructions, ra * 8, pointer - 1);
        if result.is_some() {
            return result;
        }
    }
    None
}

pub fn part2(input: &str) -> String {
    let (initial_state, instructions) = parse(input);

    find_self_duplication_ra(&initial_state, &instructions, 1, instructions.len() * 2 - 1)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
        ";
        assert_eq!(part1(input), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        let input = "
        Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0
        ";
        assert_eq!(part2(input), "117440");
    }
}
