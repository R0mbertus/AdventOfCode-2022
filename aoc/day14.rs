use aoc_runner_derive::{aoc, aoc_generator};
use regex::{Captures, Regex};
use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: isize,
    y: isize,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: (self.x + rhs.x).rem_euclid(101),
            y: (self.y + rhs.y).rem_euclid(103),
        }
    }
}

#[derive(Clone, Copy)]
struct Robot {
    pos: Pos,
    dir: Pos,
}

impl Robot {
    fn new(w: &Captures<'_>) -> Self {
        Self {
            pos: Pos {
                x: w[1].parse().unwrap(),
                y: w[2].parse().unwrap(),
            },
            dir: Pos {
                x: w[3].parse().unwrap(),
                y: w[4].parse().unwrap(),
            },
        }
    }

    fn move_forward(&self) -> Self {
        Self {
            pos: self.pos + self.dir,
            dir: self.dir,
        }
    }
}

#[derive(Clone)]
struct Map {
    map: Vec<Vec<Vec<Robot>>>,
}

impl Map {
    fn new(robots: Vec<Robot>) -> Self {
        let mut map = vec![vec![Vec::new(); 101]; 103];
        for robot in robots {
            map[(robot.pos.y) as usize][(robot.pos.x) as usize].push(robot);
        }
        Self { map }
    }

    fn run_second(&self) -> Self {
        let mut new_map = vec![vec![Vec::new(); 101]; 103];
        for r in self.map.iter() {
            for c in r.iter() {
                for robot in c {
                    let new_robot = robot.move_forward();
                    new_map[(new_robot.pos.y) as usize][(new_robot.pos.x) as usize].push(new_robot);
                }
            }
        }
        Self { map: new_map }
    }

    fn count_quadrants(&self) -> isize {
        let (mut tl, mut tr, mut bl, mut br) = (0, 0, 0, 0);
        for r in self.map.iter() {
            for c in r.iter() {
                for robot in c {
                    if robot.pos.x < 50 && robot.pos.y < 51 {
                        tl += 1;
                    } else if robot.pos.x > 50 && robot.pos.y < 51 {
                        tr += 1;
                    } else if robot.pos.x < 50 && robot.pos.y > 51 {
                        bl += 1;
                    } else if robot.pos.x > 50 && robot.pos.y > 51 {
                        br += 1;
                    }
                }
            }
        }
        tl * tr * bl * br
    }

    fn print(&self) -> String {
        self.map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|e| if e.is_empty() { '.' } else { '#' })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[aoc_generator(day14)]
fn parse1(input: &str) -> Map {
    let r = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    Map::new(r.captures_iter(input).map(|w| Robot::new(&w)).collect())
}

#[aoc(day14, part1)]
fn part1(input: &Map) -> isize {
    (0..100)
        .fold(input.clone(), |acc, _| acc.run_second())
        .count_quadrants()
}

#[aoc(day14, part2)]
fn part2(input: &Map) -> String {
    let mut map = input.clone();
    let mut map_str = map.print();

    for _ in 1..=7858 {
        map = map.run_second();
        map_str = map.print();
    }

    format!("\n{}{}", map_str, "\n7857")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&&parse1(INPUT)), 21);
    }
}
