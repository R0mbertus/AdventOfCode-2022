use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1, part1)]
fn parse1(input: &str) -> Vec<u32> {
    vec![1]
}

#[aoc_generator(day1, part2)]
fn parse2(input: &str) -> Vec<u32> {
    vec![1]
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> u32 {
    input.iter().sum()
}

// #[cfg(test)]
// mod tests {
// use super::*;

// #[test]
// fn part1_example() {
//     assert_eq!(
//         part1(&&parse1(
//     );
// }

// #[test]
// fn part2_example() {
//     assert_eq!(
//         part2(parse2(
//     );
// }
// }
