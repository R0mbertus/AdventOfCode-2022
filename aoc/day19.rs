use aoc_runner_derive::{aoc, aoc_generator};
use cached::proc_macro::cached;

#[aoc_generator(day19)]
fn parse1(input: &str) -> (Vec<String>, Vec<String>) {
    let (towels, targets) = input.split_once("\n\n").unwrap();

    (
        towels.split(", ").map(|s| s.to_string()).collect(),
        targets.lines().map(|s| s.to_string()).collect(),
    )
}

#[cached]
fn is_possible(towels: Vec<String>, target: String) -> bool {
    target.is_empty()
        || towels.iter().any(|towel| {
            target.starts_with(towel)
                && is_possible(towels.clone(), target[towel.len()..].to_string())
        })
}

#[aoc(day19, part1)]
fn part1(input: &(Vec<String>, Vec<String>)) -> usize {
    let (towels, targets) = input.to_owned();

    targets
        .iter()
        .filter(|t| is_possible(towels.clone(), t.to_string()))
        .count()
}

#[cached]
fn count_possible(towels: Vec<String>, target: String) -> usize {
    if target.is_empty() {
        return 1;
    }

    towels
        .iter()
        .map(|towel| {
            if target.starts_with(towel) {
                count_possible(towels.clone(), target[towel.len()..].to_string())
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day19, part2)]
fn part2(input: &(Vec<String>, Vec<String>)) -> usize {
    let (towels, targets) = input.to_owned();

    targets
        .iter()
        .map(|t| count_possible(towels.clone(), t.to_string()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&&parse1(INPUT1)), 6);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2(&&parse1(INPUT1)), 16);
    }
}
