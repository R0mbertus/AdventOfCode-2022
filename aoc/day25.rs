use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;

#[aoc_generator(day25)]
fn parse1(input: &str) -> Vec<Vec<String>> {
    input
        .split("\n\n")
        .map(|l| l.lines().map(|l| l.to_string()).collect())
        .collect()
}

#[aoc(day25, part1)]
fn part1(input: &[Vec<String>]) -> usize {
    let patterns: Vec<HashSet<(usize, usize)>> = input
        .iter()
        .map(|pattern| {
            pattern
                .iter()
                .enumerate()
                .flat_map(|(i, line)| {
                    line.chars()
                        .enumerate()
                        .filter_map(move |(j, char)| (char == '#').then(|| (i, j)))
                })
                .collect()
        })
        .collect();

    patterns
        .iter()
        .tuple_combinations()
        .filter(|(pattern1, pattern2)| pattern1.is_disjoint(pattern2))
        .count()
}
