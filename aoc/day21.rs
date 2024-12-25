use aoc_runner_derive::{aoc, aoc_generator};
use cached::proc_macro::cached;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[aoc_generator(day21)]
fn parse1(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

#[rustfmt::skip]
static NUMPAD: Lazy<HashMap<char, (isize, isize)>> = Lazy::new(|| {
    HashMap::from([
        ('7', (0, 0)), ('8', (1, 0)), ('9', (2, 0)),
        ('4', (0, 1)), ('5', (1, 1)), ('6', (2, 1)),
        ('1', (0, 2)), ('2', (1, 2)), ('3', (2, 2)),
        (' ', (0, 3)), ('0', (1, 3)), ('A', (2, 3)),
    ])
});

#[rustfmt::skip]
static KEYPAD: Lazy<HashMap<char, (isize, isize)>> = Lazy::new(|| {
    HashMap::from([
        (' ', (0, 0)), ('^', (1, 0)), ('A', (2, 0)),
        ('<', (0, 1)), ('v', (1, 1)), ('>', (2, 1)),
    ])
});

#[cached]
fn path(start: char, end: char) -> String {
    let pad = if NUMPAD.get(&start).is_some() && NUMPAD.get(&end).is_some() {
        &NUMPAD
    } else {
        &KEYPAD
    };

    let (dx, dy) = ((pad[&end].0 - pad[&start].0), (pad[&end].1 - pad[&start].1));

    let x_move = if dx >= 0 {
        ">".repeat(dx as usize)
    } else {
        "<".repeat(-dx as usize)
    };
    let y_move = if dy >= 0 {
        "v".repeat(dy as usize)
    } else {
        "^".repeat(-dy as usize)
    };

    let empty = ((pad[&' '].0 - pad[&start].0), (pad[&' '].1 - pad[&start].1));
    (if (dx > 0 || ((dx, 0) == empty)) && (0, dy) != empty {
        y_move + &x_move
    } else {
        x_move + &y_move
    }) + "A"
}

#[cached]
fn length(code: String, depth: usize) -> usize {
    if depth == 0 {
        return code.len();
    }

    code.chars().enumerate().fold(0, |acc, (i, c)| {
        let j = if i > 0 { i - 1 } else { code.len() - 1 };
        let p = path(code.chars().nth(j).unwrap(), c);
        let t = length(p, depth - 1);
        acc + t
    })
}

#[aoc(day21, part1)]
fn part1(input: &[String]) -> usize {
    input.iter().fold(0, |acc, l| {
        let len = length(l.to_string(), 3);
        let num = l[..l.len() - 1].parse::<usize>().unwrap();
        acc + (num * len)
    })
}

#[aoc(day21, part2)]
fn part2(input: &[String]) -> usize {
    input.iter().fold(0, |acc, l| {
        let len = length(l.to_string(), 26);
        let num = l[..l.len() - 1].parse::<usize>().unwrap();
        acc + (num * len)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&&parse1(INPUT1)), 126384);
    }
}
