use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

struct Mul {
    x: u32,
    y: u32,
}

#[aoc_generator(day3, part1)]
fn parse1(input: &str) -> Vec<Mul> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    regex
        .captures_iter(input)
        .map(|m| Mul {
            x: m[1].parse().unwrap(),
            y: m[2].parse().unwrap(),
        })
        .collect()
}

#[aoc_generator(day3, part2)]
fn parse2(input: &str) -> Vec<Mul> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut multiply = true;
    let mut results = Vec::new();
    for m in regex.captures_iter(input) {
        match m.get(0).unwrap().as_str() {
            "do()" => multiply = true,
            "don't()" => multiply = false,
            _ if multiply => results.push(Mul {
                x: m[1].parse().unwrap(),
                y: m[2].parse().unwrap(),
            }),
            _ => (),
        };
    }
    results
}

#[aoc(day3, part1)]
fn part1(input: &[Mul]) -> u32 {
    input.iter().map(|m| m.x * m.y).sum()
}

#[aoc(day3, part2)]
fn part2(input: &[Mul]) -> u32 {
    input.iter().map(|m| m.x * m.y).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&&parse1(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            )),
            161
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&&parse2(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            )),
            48
        );
    }
}
