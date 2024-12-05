use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day5)]
fn parse1(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let (ordering_raw, produced_raw) = input.split_once("\n\n").expect("Input is invalid!");
    let ordering = ordering_raw
        .lines()
        .map(|l| l.split_once('|').expect("Malformed ordering!"))
        .fold(HashMap::<u32, Vec<u32>>::new(), |mut acc, (b, a)| {
            acc.entry(b.parse().unwrap())
                .or_insert_with(Vec::new)
                .push(a.parse().unwrap());
            acc
        });

    let produced = produced_raw
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse().expect("Malformed production lists!"))
                .collect()
        })
        .collect();

    (ordering, produced)
}

fn filter_swap(v: &[u32], map: &HashMap<u32, Vec<u32>>, cond: bool) -> bool {
    v.iter().enumerate().any(|(i, e)| {
        map.get(e)
            .unwrap_or(&Vec::new())
            .iter()
            .any(|n| v[..i].contains(n))
    }) != cond
}

#[aoc(day5, part1)]
fn part1(input: &(HashMap<u32, Vec<u32>>, Vec<Vec<u32>>)) -> u32 {
    input
        .1
        .iter()
        .filter(|v| filter_swap(v, &input.0, true))
        .map(|v| v[v.len() / 2])
        .sum()
}

#[aoc(day5, part2)]
fn part2(input: &(HashMap<u32, Vec<u32>>, Vec<Vec<u32>>)) -> u32 {
    input
        .1
        .clone()
        .into_iter()
        .filter(|v| filter_swap(v, &input.0, false))
        .map(|mut v| {
            let mut i = 0;
            while i < v.len() {
                if let Some(pre) = input.0.get(&v[i]) {
                    if let Some(j) = v[..i].iter().position(|e| pre.contains(e)) {
                        let val = v.remove(i);
                        v.insert(j, val);
                    }
                }
                i += 1;
            }
            v[v.len() / 2]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&&parse1(INPUT)), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&&parse1(INPUT)), 123);
    }
}
