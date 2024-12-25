use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day20)]
fn parse1(input: &str) -> HashMap<(isize, isize), isize> {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start: (isize, isize) = map
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.iter()
                .enumerate()
                .map(|(x, c)| ((x as isize, y as isize), *c))
                .collect::<Vec<((isize, isize), char)>>()
        })
        .find(|(_, c)| *c == 'S')
        .unwrap()
        .0;

    let mut path = vec![];
    let mut current = vec![start];

    while let Some(pos) = current.pop() {
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let new_pos = (pos.0 as isize + dx, pos.1 as isize + dy);
            if new_pos.0 < 0
                || new_pos.1 < 0
                || new_pos.0 >= map[0].len() as isize
                || new_pos.1 >= map.len() as isize
                || map[new_pos.1 as usize][new_pos.0 as usize] == '#'
                || path.contains(&new_pos)
            {
                continue;
            }
            path.push(new_pos);
            current.push(new_pos);
        }
    }

    path.iter()
        .enumerate()
        .map(|(i, &pos)| (pos, i as isize))
        .collect()
}

#[aoc(day20, part1)]
fn part1(input: &HashMap<(isize, isize), isize>) -> isize {
    input.keys().clone().into_iter().fold(0, |mut acc, (x, y)| {
        for (dx, dy) in [(-2, 0), (2, 0), (0, -2), (0, 2)].iter() {
            let new_pos = (x + dx, y + dy);
            if let Some(v) = input.get(&new_pos) {
                acc += ((v - input[&(*x, *y)]) > 100) as isize;
            }
        }
        acc
    })
}

#[aoc(day20, part2)]
fn part2(input: &HashMap<(isize, isize), isize>) -> isize {
    input.keys().into_iter().combinations(2).fold(0, |acc, c| {
        let (a, b) = (c[0], c[1]);
        let dist = (a.0 - b.0).abs() + (a.1 - b.1).abs();
        acc + (dist < 21 && (input[&b] - input[&a]).abs() >= 100 + dist) as isize
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&&parse1(INPUT1)), 0);
    }
}
