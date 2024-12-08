use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

#[aoc_generator(day8)]
fn parse1(input: &str) -> (HashMap<char, Vec<Pos>>, usize, usize) {
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                map.entry(c).or_insert(vec![]).push(Pos { x, y });
            }
        }
    }
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    (map, width, height)
}

#[aoc(day8, part1)]
fn part1(input: &(HashMap<char, Vec<Pos>>, usize, usize)) -> usize {
    let (map, w, h) = input;
    let mut visited = vec![vec![false; *w]; *h];
    for e in map.values() {
        for (a, b) in e.iter().tuple_combinations() {
            let dx = b.x as i64 - a.x as i64;
            let dy = b.y as i64 - a.y as i64;
            if let Some(e) = visited
                .get_mut(a.y.wrapping_sub(dy as usize))
                .get_or_insert(&mut vec![])
                .get_mut(a.x.wrapping_sub(dx as usize))
            {
                *e = true;
            }
            if let Some(e) = visited
                .get_mut(b.y.wrapping_add(dy as usize))
                .get_or_insert(&mut vec![])
                .get_mut(b.x.wrapping_add(dx as usize))
            {
                *e = true;
            }
        }
    }
    visited.iter().flatten().filter(|&&v| v).count()
}

fn repeat_measure(
    visited: &mut [Vec<bool>],
    mut x: usize,
    mut y: usize,
    dx: usize,
    dy: usize,
    op: fn(usize, usize) -> usize,
) {
    while let Some(e) = visited.get_mut(y).get_or_insert(&mut vec![]).get_mut(x) {
        *e = true;
        y = op(y, dy);
        x = op(x, dx);
    }
}

#[aoc(day8, part2)]
fn part2(input: &(HashMap<char, Vec<Pos>>, usize, usize)) -> usize {
    let (map, w, h) = input;
    let mut visited = vec![vec![false; *w]; *h];
    for e in map.values() {
        for (a, b) in e.iter().tuple_combinations() {
            let dx = b.x.wrapping_sub(a.x);
            let dy = b.y.wrapping_sub(a.y);
            repeat_measure(&mut visited, a.x, a.y, dx, dy, usize::wrapping_sub);
            repeat_measure(&mut visited, b.x, b.y, dx, dy, usize::wrapping_add);
        }
    }
    visited.iter().flatten().filter(|&&v| v).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&&parse1(INPUT)), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&&parse1(INPUT)), 34);
    }
}
