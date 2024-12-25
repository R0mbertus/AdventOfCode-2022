use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[aoc_generator(day22)]
fn parse1(input: &str) -> Vec<isize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day22, part1)]
fn part1(input: &[isize]) -> isize {
    input
        .iter()
        .map(|&s| {
            (0..2000).fold(s, |acc, _| {
                let s1 = ((acc * 64) ^ acc) % 16777216;
                let s2 = ((s1 / 32) ^ s1) % 16777216;
                ((s2 * 2048) ^ s2) % 16777216
            })
        })
        .sum()
}

#[aoc(day22, part2)]
fn part2(input: &[isize]) -> isize {
    let values: Vec<Vec<_>> = input
        .iter()
        .map(|&s| {
            let mut last = vec![];
            (0..2000).fold(s, |acc, _| {
                let s1 = ((acc * 64) ^ acc) % 16777216;
                let s2 = ((s1 / 32) ^ s1) % 16777216;
                let s3 = ((s2 * 2048) ^ s2) % 16777216;
                last.push(s3 % 10);
                s3
            });
            last
        })
        .collect();

    let changes: Vec<Vec<_>> = values
        .iter()
        .map(|e| {
            e.iter()
                .tuple_windows::<(_, _)>()
                .map(|(&a, &b)| b - a)
                .collect()
        })
        .collect();

    let mut max: HashMap<Vec<isize>, isize> = HashMap::new();
    for (i, c) in changes.iter().enumerate() {
        let mut keys = HashSet::new();
        for j in 0..(c.len() - 3) {
            let key = c[j..(j + 4)].to_vec();
            if keys.contains(&key) {
                continue;
            }
            max.entry(key.clone())
                .and_modify(|e| *e += values[i][j + 4])
                .or_insert(values[i][j + 4]);
            keys.insert(key);
        }
    }
    max.into_values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "1
10
100
2024";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&&parse1(INPUT1)), 37327623);
    }

    const INPUT2: &str = "1
2
3
2024";

    #[test]
    fn part1_example2() {
        assert_eq!(part2(&&parse1(INPUT2)), 23);
    }
}
