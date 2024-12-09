use std::{i64, usize, vec};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse1(input: &str) -> Vec<i64> {
    let mut id = 0;
    input
        .chars()
        .map(|c| c.to_digit(10).expect("Malformed input!"))
        .collect::<Vec<u32>>()
        .chunks(2)
        .flat_map(|c| {
            let file = vec![id; c[0] as usize];
            id += 1;
            if c.len() == 1 {
                file
            } else {
                vec![file, vec![-1; c[1] as usize]].concat()
            }
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[i64]) -> usize {
    let mut compressed = input.to_vec();
    let mut i = 0;
    while i < compressed.len() {
        if compressed[i] == -1 {
            while let Some(e) = compressed.pop() {
                if i >= compressed.len() {
                    break;
                }
                if e != -1 {
                    compressed[i] = e;
                    break;
                }
            }
        }
        i += 1;
    }
    compressed
        .into_iter()
        .enumerate()
        .map(|(i, n)| n as usize * i)
        .sum()
}

fn find_spot(compressed: &[i64], size: usize, max: usize) -> Option<usize> {
    let mut found_size = 0;
    for (i, e) in compressed.iter().enumerate() {
        if *e == -1 {
            found_size += 1;
        } else {
            found_size = 0;
        }
        if found_size == size {
            return Some(i - size + 1);
        }
        if i == max {
            return None;
        }
    }
    None
}

#[aoc(day9, part2)]
fn part2(input: &[i64]) -> usize {
    let mut compressed = input.to_vec();
    let mut i = compressed.len() - 1;
    let mut current_id = i64::MAX;
    while i > 0 {
        if compressed[i] != -1 && compressed[i] < current_id {
            let mut block = vec![];
            current_id = compressed[i];
            while compressed[i] == current_id && i > 0 {
                block.push(compressed[i]);
                i -= 1;
            }
            i += 1;
            if let Some(spot) = find_spot(&compressed, block.len(), i) {
                let size = block.len();
                compressed.splice(spot..spot + size, block);
                compressed.splice(i..i + size, vec![-1; size]);
            }
        }
        i -= 1;
    }
    compressed
        .into_iter()
        .map(|n| if n == -1 { 0 } else { n })
        .enumerate()
        .map(|(i, n)| n as usize * i)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&&parse1(INPUT)), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&&parse1(INPUT)), 2858);
    }
}
