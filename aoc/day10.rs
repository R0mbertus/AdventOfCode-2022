use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day10)]
fn parse1(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("Malformed input!"))
                .collect()
        })
        .collect()
}

fn get_paths(input: &[Vec<u32>], x: i64, y: i64, previous: u32) -> Vec<(i64, i64)> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .flat_map(|(dx, dy)| {
            let nx = x + dx;
            let ny = y + dy;
            if let Some(pos) = input
                .get(ny as usize)
                .get_or_insert(&vec![])
                .get(nx as usize)
            {
                if *pos == previous + 1 {
                    if *pos == 9 {
                        return vec![(nx, ny)];
                    }
                    return get_paths(input, nx, ny, *pos);
                }
            }
            vec![]
        })
        .collect()
}

#[aoc(day10, part1)]
fn part1(input: &[Vec<u32>]) -> usize {
    let mut total = 0;
    for (y, r) in input.iter().enumerate() {
        for (x, e) in r.iter().enumerate() {
            if *e == 0 {
                total += get_paths(input, x as i64, y as i64, 0)
                    .into_iter()
                    .collect::<HashSet<_>>()
                    .len();
            }
        }
    }
    total
}

#[aoc(day10, part2)]
fn part2(input: &[Vec<u32>]) -> usize {
    let mut total = 0;
    for (y, r) in input.iter().enumerate() {
        for (x, e) in r.iter().enumerate() {
            if *e == 0 {
                total += get_paths(input, x as i64, y as i64, 0).len()
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&&parse1(INPUT)), 36);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&&parse1(INPUT)), 81);
    }
}
