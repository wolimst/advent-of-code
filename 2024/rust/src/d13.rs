use itertools::Itertools;

#[derive(Debug)]
struct Game {
    ax: usize,
    ay: usize,
    bx: usize,
    by: usize,
    px: usize,
    py: usize,
}

fn parse(input: &str) -> Vec<Game> {
    input
        .trim()
        .split("\n\n")
        .map(|input| {
            let (ax, ay, bx, by, px, py) = input
                .split(|c: char| !c.is_ascii_digit())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            Game {
                ax,
                ay,
                bx,
                by,
                px,
                py,
            }
        })
        .collect()
}

fn is_close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-4
}

fn inverse(matrix: (f64, f64, f64, f64)) -> Option<(f64, f64, f64, f64)> {
    let (a, b, c, d) = matrix;
    let det = a * d - b * c;
    if is_close(det, 0.0) {
        None
    } else {
        Some((d / det, -b / det, -c / det, a / det))
    }
}

fn product(matrix: (f64, f64, f64, f64), vector: (f64, f64)) -> (f64, f64) {
    (
        matrix.0 * vector.0 + matrix.1 * vector.1,
        matrix.2 * vector.0 + matrix.3 * vector.1,
    )
}

fn calc_button_clicks(game: &Game) -> Option<(usize, usize)> {
    let (ax, ay, bx, by, px, py) = (
        game.ax as f64,
        game.ay as f64,
        game.bx as f64,
        game.by as f64,
        game.px as f64,
        game.py as f64,
    );
    if let Some(inverse_matrix) = inverse((ax, bx, ay, by)) {
        let (a, b) = product(inverse_matrix, (px, py));
        if is_close(a, a.round()) && is_close(b, b.round()) && a >= 0.0 && b >= 0.0 {
            return Some((a.round() as usize, b.round() as usize));
        }
    }
    None
}

pub fn part1(input: &str) -> String {
    let games = parse(input);

    games
        .iter()
        .filter_map(calc_button_clicks)
        .map(|(a, b)| a * 3 + b)
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let games = parse(input);

    games
        .into_iter()
        .map(|mut game| {
            game.px += 10000000000000;
            game.py += 10000000000000;
            game
        })
        .filter_map(|game| calc_button_clicks(&game))
        .map(|(a, b)| a * 3 + b)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
        ";
        assert_eq!(part1(input), "480");
    }

    #[test]
    fn test_part2() {
        let input = "
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
        ";
        assert_eq!(part2(input), "875318608908");
    }
}
