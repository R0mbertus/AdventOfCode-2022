use aoc_runner_derive::{aoc, aoc_generator};
use regex::{Captures, Regex};
use std::ops::{Add, Mul};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(cap: &Captures<'_>) -> Self {
        Self {
            x: cap.get(1).unwrap().as_str().parse().unwrap(),
            y: cap.get(2).unwrap().as_str().parse().unwrap(),
        }
    }
}

impl Mul<isize> for Pos {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Machine {
    a: Pos,
    b: Pos,
    target: Pos,
}

impl Add<isize> for Machine {
    type Output = Self;

    fn add(self, rhs: isize) -> Self {
        Self {
            a: self.a,
            b: self.b,
            target: self.target + Pos { x: rhs, y: rhs },
        }
    }
}

#[aoc_generator(day13, part1)]
fn parse1(input: &str) -> Vec<Machine> {
    let r = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();

    r.captures_iter(input)
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|w| Machine {
            a: Pos::new(&w[0]),
            b: Pos::new(&w[1]),
            target: Pos::new(&w[2]),
        })
        .collect()
}

fn find_multipliers(machine: &Machine) -> Vec<(isize, isize)> {
    (0..100)
        .flat_map(|a| {
            (0..100).filter_map(move |b| {
                if machine.a * a + machine.b * b == machine.target {
                    Some((a, b))
                } else {
                    None
                }
            })
        })
        .collect()
}

#[aoc(day13, part1)]
fn part1(input: &[Machine]) -> isize {
    input.iter().fold(0, |acc, m| {
        acc + find_multipliers(m)
            .iter()
            .map(|(a, b)| a * 3 + b)
            .min()
            .unwrap_or(0)
    })
}

#[aoc_generator(day13, part2)]
fn parse2(input: &str) -> Vec<Machine> {
    parse1(input)
        .into_iter()
        .map(|m| m + 10000000000000)
        .collect()
}

fn find_multiplier_linear(machine: &Machine) -> (f64, f64) {
    let divisor = (machine.a.x * machine.b.y - machine.a.y * machine.b.x) as f64;
    (
        (machine.target.x * machine.b.y - machine.target.y * machine.b.x) as f64 / divisor,
        (machine.a.x * machine.target.y - machine.a.y * machine.target.x) as f64 / divisor,
    )
}

#[aoc(day13, part2)]
fn part2(input: &[Machine]) -> isize {
    input
        .iter()
        .map(|m| {
            let (x, y) = find_multiplier_linear(m);
            if x.fract() != 0.0 || y.fract() != 0.0 || x < 0.0 || y < 0.0 {
                0
            } else {
                (x * 3.0 + y) as isize
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Button A: X+94, Y+34
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
Prize: X=18641, Y=10279";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&&parse1(INPUT)), 480);
    }
}
