use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;

#[aoc_generator(day18)]
fn parse1(input: &str) -> Vec<(isize, isize)> {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn path(bad_bytes: Vec<(isize, isize)>, bytes: isize) -> Option<isize> {
    let mut seen = bad_bytes[..bytes as usize].to_vec();
    let mut current = VecDeque::from([(0, (0, 0))]);

    while let Some((d, pos)) = current.pop_front() {
        if pos == (70, 70) {
            return Some(d);
        }

        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos = (pos.0 + dx, pos.1 + dy);

            if new_pos.0 < 0
                || new_pos.0 > 70
                || new_pos.1 < 0
                || new_pos.1 > 70
                || seen.contains(&new_pos)
            {
                continue;
            }

            seen.push(new_pos);
            current.push_back((d + 1, new_pos));
        }
    }

    None
}

#[aoc(day18, part1)]
fn part1(input: &[(isize, isize)]) -> isize {
    path(input.to_vec(), 1024).unwrap()
}

#[aoc(day18, part2)]
fn part2(input: &[(isize, isize)]) -> String {
    let i = (1024..)
        .find(|&i| path(input.to_vec(), i).is_none())
        .unwrap()
        - 1;

    format!("{},{}", input[i as usize].0, input[i as usize].1)
}
