use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
fn parse1(input: &str) -> Vec<String> {
    input.split(' ').map(|s| s.to_string()).collect()
}

fn perform_stone_ritual(input: &[String]) -> Vec<String> {
    input
        .iter()
        .flat_map(|e| match e.as_str() {
            "0" => vec!["1".to_string()],
            x if x.len() % 2 == 0 => vec![
                x[..(x.len() / 2)]
                    .parse::<usize>()
                    .expect("Malformed input!")
                    .to_string(),
                x[(x.len() / 2)..]
                    .parse::<usize>()
                    .expect("Malformed input!")
                    .to_string(),
            ],
            x => vec![(x.parse::<usize>().expect("Malformed input!") * 2024).to_string()],
        })
        .collect()
}

#[aoc(day11, part1)]
fn part1(input: &[String]) -> usize {
    (0..25)
        .fold(input.to_vec(), |acc, _| perform_stone_ritual(&acc))
        .len()
}

fn perform_stone_ritual_hash(input: &HashMap<String, usize>) -> HashMap<String, usize> {
    input
        .iter()
        .fold(HashMap::new(), |mut acc, (s, c)| match s.as_str() {
            "0" => {
                *acc.entry("1".to_string()).or_insert(0) += c;
                acc
            }
            x if x.len() % 2 == 0 => {
                *acc.entry(
                    x[..(x.len() / 2)]
                        .parse::<usize>()
                        .expect("Malformed input!")
                        .to_string(),
                )
                .or_insert(0) += c;
                *acc.entry(
                    x[(x.len() / 2)..]
                        .parse::<usize>()
                        .expect("Malformed input!")
                        .to_string(),
                )
                .or_insert(0) += c;
                acc
            }
            x => {
                *acc.entry((x.parse::<usize>().expect("Malformed input!") * 2024).to_string())
                    .or_insert(0) += c;
                acc
            }
        })
}

#[aoc(day11, part2)]
fn part2(input: &[String]) -> usize {
    let map = input.iter().fold(HashMap::new(), |mut acc, s| {
        *acc.entry(s.to_string()).or_insert(0) += 1;
        acc
    });

    (0..75)
        .fold(map, |acc, _| perform_stone_ritual_hash(&acc))
        .values()
        .fold(0, |acc, x| acc + x)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&&parse1(INPUT)), 55312);
    }
}
