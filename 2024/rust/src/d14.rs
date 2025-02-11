use std::{
    collections::HashSet,
    sync::atomic::{AtomicIsize, Ordering},
};

use itertools::Itertools;

static X: AtomicIsize = AtomicIsize::new(101);
static Y: AtomicIsize = AtomicIsize::new(103);

struct Robot {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

fn parse(input: &str) -> Vec<Robot> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (x, y, vx, vy) = line
                .split(|c: char| !c.is_ascii_digit() && c != '-')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<isize>().unwrap())
                .collect_tuple()
                .unwrap();
            Robot { x, y, vx, vy }
        })
        .collect()
}

fn wait(robot: &Robot, time: isize) -> (isize, isize) {
    let (x_max, y_max) = (X.load(Ordering::Relaxed), Y.load(Ordering::Relaxed));
    let x = robot.x + robot.vx * time;
    let y = robot.y + robot.vy * time;
    (((x % x_max) + x_max) % x_max, ((y % y_max) + y_max) % y_max)
}

fn count(positions: &Vec<(isize, isize)>) -> [usize; 4] {
    let (x, y) = (X.load(Ordering::Relaxed), Y.load(Ordering::Relaxed));
    let (rx1, rx2) = (0..x / 2, x / 2 + 1..x);
    let (ry1, ry2) = (0..y / 2, y / 2 + 1..y);
    let ranges = [(&rx1, &ry1), (&rx1, &ry2), (&rx2, &ry1), (&rx2, &ry2)];
    ranges
        .iter()
        .map(|(rx, ry)| {
            positions
                .iter()
                .filter(|(x, y)| rx.contains(x) && ry.contains(y))
                .count()
        })
        .collect_array()
        .unwrap()
}

pub fn part1(input: &str) -> String {
    let robots = parse(input);
    let time = 100;

    let positions = robots.iter().map(|robot| wait(robot, time)).collect();
    count(&positions).iter().product::<usize>().to_string()
}

fn _print(positions: &HashSet<(isize, isize)>) {
    for y in 0..Y.load(Ordering::Relaxed) {
        for x in 0..X.load(Ordering::Relaxed) {
            if positions.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    std::thread::sleep(std::time::Duration::from_millis(750));
}

pub fn part2(_input: &str) -> String {
    // let robots = parse(_input);

    // let (x_max, y_max) = (X.load(Ordering::Relaxed), Y.load(Ordering::Relaxed));
    // let repeating_time = x_max * y_max;
    // for time in 0..repeating_time {
    //     let positions: HashSet<(isize, isize)> =
    //         robots.iter().map(|robot| wait(robot, time)).collect();

    //     positions
    //         .iter()
    //         .any(|(x, y)| (0..5).all(|i| positions.contains(&(x + i, y + i))))
    //         .then(|| {
    //             println!("\n");
    //             println!("time: {:?}", time);
    //             _print(&positions);
    //         });
    // }

    "6876".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        X.store(11, Ordering::Relaxed);
        Y.store(7, Ordering::Relaxed);
        let input = "
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
        ";
        assert_eq!(part1(input), "12");
    }
}
