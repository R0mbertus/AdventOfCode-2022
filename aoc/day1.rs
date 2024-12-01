use aoc_runner_derive::{aoc, aoc_generator};
use std::{cmp, collections::HashMap};

#[aoc_generator(day1)]
fn parse1(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|l| {
            let nums = l
                .split_whitespace()
                .map(|i| i.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            (nums[0], nums[1])
        })
        .unzip()
}

#[aoc(day1, part1)]
fn part1(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut list1 = input.0.to_owned();
    let mut list2 = input.1.to_owned();

    list1.sort();
    list2.sort();

    list1
        .iter()
        .zip(list2)
        .map(|(e1, e2)| cmp::max(e1, &e2) - cmp::min(e1, &e2))
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut m: HashMap<u32, u32> = HashMap::new();
    for e in input.1.iter().copied() {
        *m.entry(e).or_default() += 1;
    }

    input.0.iter().map(|e| m.get(e).unwrap_or(&0) * e).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&&parse1(INPUT)), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&&parse1(INPUT)), 31);
    }
}
