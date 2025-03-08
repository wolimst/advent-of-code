use std::collections::HashMap;

use itertools::Itertools;
use rand::prelude::*;

#[derive(Debug, Clone)]
struct Circuit {
    wire_a: String,
    wire_b: String,
    wire_out: String,
    op: Operator,
}

#[derive(Debug, Clone)]
enum Operator {
    AND,
    OR,
    XOR,
}

fn parse(input: &str) -> (HashMap<String, bool>, Vec<Circuit>) {
    let mut input_section = input.trim().split("\n\n");

    let wires = input_section
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut line = line.trim().split(": ");
            let wire = line.next().unwrap().to_string();
            let value = match line.next().unwrap() {
                "1" => true,
                "0" => false,
                _ => unreachable!("invalid value"),
            };
            (wire, value)
        })
        .collect();

    let circuits = input_section
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut line = line
                .trim()
                .split(|c: char| !c.is_ascii_alphanumeric())
                .filter(|s| !s.is_empty());
            let wire_a = line.next().unwrap().to_string();
            let op = match line.next().unwrap() {
                "AND" => Operator::AND,
                "OR" => Operator::OR,
                "XOR" => Operator::XOR,
                _ => unreachable!("invalid operator"),
            };
            let wire_b = line.next().unwrap().to_string();
            let wire_out = line.next().unwrap().to_string();
            Circuit {
                wire_a,
                wire_b,
                wire_out,
                op,
            }
        })
        .collect();

    (wires, circuits)
}

fn wires_to_number(wires: HashMap<String, bool>) -> usize {
    wires
        .iter()
        .sorted()
        .rev()
        .fold(0, |acc, (_, v)| acc << 1 | if *v { 1 } else { 0 })
}

fn eval(wires: HashMap<String, bool>, mut circuits: Vec<Circuit>) -> Option<usize> {
    let mut wires = wires;
    while !circuits.is_empty() {
        let mut updated = false;
        let mut i = 0;
        while i < circuits.len() {
            let circuit = &circuits[i];
            match (wires.get(&circuit.wire_a), wires.get(&circuit.wire_b)) {
                (Some(a), Some(b)) => {
                    let circuit = circuits.remove(i);
                    let value = match circuit.op {
                        Operator::AND => a & b,
                        Operator::OR => a | b,
                        Operator::XOR => a ^ b,
                    };
                    wires.insert(circuit.wire_out, value);
                    updated = true;
                }
                _ => i += 1,
            }
        }

        if !updated {
            return None;
        }
    }
    let wires = wires
        .into_iter()
        .filter(|(k, _)| k.starts_with("z"))
        .collect();
    Some(wires_to_number(wires))
}

pub fn part1(input: &str) -> String {
    let (wires, circuits) = parse(input);
    eval(wires, circuits).unwrap().to_string()
}

fn diff_bits(a: usize, b: usize) -> usize {
    (a ^ b).count_ones() as usize
}

fn expected(wires: &HashMap<String, bool>) -> usize {
    ["x", "y"]
        .iter()
        .map(|prefix| {
            wires
                .iter()
                .filter(|(k, _)| k.starts_with(prefix))
                .map(|(k, v)| (k.clone(), *v))
                .collect()
        })
        .map(wires_to_number)
        .sum()
}

fn decode(circuits: &Vec<Circuit>, swaps: &Vec<usize>) -> Vec<Circuit> {
    let mut circuits = circuits.clone();
    swaps.chunks(2).for_each(|pair| {
        let (i, j) = (pair[0], pair[1]);
        (circuits[i].wire_out, circuits[j].wire_out) =
            (circuits[j].wire_out.clone(), circuits[i].wire_out.clone());
    });
    circuits
}

fn mean_squared_error(
    wires: &mut HashMap<String, bool>,
    circuits: &Vec<Circuit>,
    swaps: &Vec<usize>,
    n_trials: usize,
) -> f64 {
    if !swaps.iter().all_unique() {
        return f64::INFINITY;
    }
    let circuits = decode(circuits, swaps);
    let mut wires_list = (0..n_trials)
        .map(|_| {
            let mut wires = wires.clone();
            wires.iter_mut().for_each(|(_, v)| *v = rand::random());
            wires
        })
        .collect_vec();
    (0..n_trials)
        .map(|_| {
            let wires = wires_list.pop().unwrap();
            let expected_value = expected(&wires);
            let actual_value = eval(wires.clone(), circuits.clone()).unwrap_or(!expected_value);
            diff_bits(actual_value, expected_value)
        })
        .map(|score| score as f64 * score as f64 / n_trials as f64)
        .sum()
}

fn selection<'a>(
    population: &'a Vec<Vec<usize>>,
    mse: &Vec<f64>,
    rng: &mut ThreadRng,
) -> &'a Vec<usize> {
    let tournament_size = 2;
    let indices = (0..population.len()).choose_multiple(rng, tournament_size);
    let best_index = indices
        .into_iter()
        .min_by(|&i, &j| mse[i].partial_cmp(&mse[j]).unwrap())
        .unwrap();
    &population[best_index]
}

fn crossover(
    parent1: &Vec<usize>,
    parent2: &Vec<usize>,
    rng: &mut ThreadRng,
) -> (Vec<usize>, Vec<usize>) {
    let split = rng.random_range(0..parent1.len());
    let split = split / 2 * 2;
    let mut child1 = parent1[..split].to_vec();
    let mut child2 = parent2[..split].to_vec();
    parent1
        .iter()
        .skip(split)
        .tuple_windows()
        .for_each(|(a, b)| {
            if !child2.contains(a) && !child2.contains(b) {
                child2.push(*a);
                child2.push(*b);
            }
        });
    parent2
        .iter()
        .skip(split)
        .tuple_windows()
        .for_each(|(a, b)| {
            if !child1.contains(a) && !child1.contains(b) {
                child1.push(*a);
                child1.push(*b);
            }
        });
    let mut values = parent1.iter().chain(parent2.iter()).collect_vec();
    values.shuffle(rng);
    values.iter().for_each(|&i| {
        if child1.len() < parent1.len() && !child1.contains(i) {
            child1.push(*i);
        }
        if child2.len() < parent1.len() && !child2.contains(i) {
            child2.push(*i);
        }
    });
    (child1, child2)
}

fn mutate(
    individual: &mut Vec<usize>,
    circuit_length: usize,
    mutation_rate: f64,
    rng: &mut ThreadRng,
) {
    if rng.random::<f64>() < mutation_rate {
        let pos = rng.random_range(0..individual.len());
        let value = loop {
            let value = rng.random_range(0..circuit_length);
            if !individual.contains(&value) {
                break value;
            }
        };
        individual[pos] = value;
    }
}

fn genetic_algorithm(mut wires: HashMap<String, bool>, circuits: Vec<Circuit>) -> Vec<String> {
    let mut rng = rand::rng();

    const N_SWAPS: usize = 8;
    const N_INDIVIDUALS: usize = 250;
    const MUTATION_RATE: f64 = 0.2;
    const SCORE_TRIALS: usize = 25;
    const FINAL_SCORE_TRIALS: usize = SCORE_TRIALS * 20;

    let mut population = (0..N_INDIVIDUALS)
        .map(|_| {
            (0..circuits.len())
                .collect_vec()
                .choose_multiple(&mut rng, N_SWAPS)
                .cloned()
                .collect_vec()
        })
        .collect_vec();
    let mut mses = population
        .iter()
        .map(|seq| mean_squared_error(&mut wires, &circuits, seq, SCORE_TRIALS))
        .collect_vec();
    let mut best_index = mses
        .iter()
        .enumerate()
        .min_by(|(_, mse_a), (_, mse_b)| mse_a.partial_cmp(mse_b).unwrap())
        .unwrap()
        .0;

    let mut gen = 0;
    while mses[best_index] > 0.0
        || mean_squared_error(
            &mut wires,
            &circuits,
            &population[best_index],
            FINAL_SCORE_TRIALS,
        ) > 0.0
    {
        if gen % 10 == 0 {
            println!(
                "gen: {}, best mean squared error: {}",
                gen, mses[best_index]
            );
        }

        let mut new_population = vec![population[best_index].clone()];
        while new_population.len() < N_INDIVIDUALS {
            let parent1 = selection(&population, &mses, &mut rng);
            let parent2 = selection(&population, &mses, &mut rng);

            let (child1, child2) = crossover(parent1, parent2, &mut rng);
            new_population.push(child1);
            new_population.push(child2);
        }

        new_population
            .iter_mut()
            .for_each(|individual| mutate(individual, circuits.len(), MUTATION_RATE, &mut rng));

        let new_mses = new_population
            .iter()
            .map(|seq| mean_squared_error(&mut wires, &circuits, seq, SCORE_TRIALS))
            .collect_vec();

        (population, mses) = new_population
            .into_iter()
            .zip(new_mses.into_iter())
            .k_smallest_by(N_INDIVIDUALS, |(_, mse_a), (_, mse_b)| {
                mse_a.partial_cmp(mse_b).unwrap()
            })
            .unzip();
        best_index = mses
            .iter()
            .enumerate()
            .min_by(|(_, mse_a), (_, mse_b)| mse_a.partial_cmp(mse_b).unwrap())
            .unwrap()
            .0;

        gen += 1;
    }

    let swaps = population[best_index]
        .iter()
        .map(|i| circuits[*i].wire_out.clone())
        .sorted()
        .collect();
    println!(
        "gen: {}, best mean squared error: {}",
        gen, mses[best_index]
    );
    println!("swap sequences: {:?}", swaps);
    swaps
}

pub fn part2(input: &str) -> String {
    let (wires, circuits) = parse(input);

    let swapped_output_wires = genetic_algorithm(wires, circuits);
    swapped_output_wires.iter().sorted().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
        x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj
        ";
        assert_eq!(part1(input), "2024");
    }
}
