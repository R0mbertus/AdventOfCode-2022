use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn parse1(input: &str) -> (Vec<Vec<char>>, (usize, usize, char)) {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let guard = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter().enumerate().find_map(|(x, c)| {
                if c != &'#' && c != &'.' {
                    Some((x, y, c.clone()))
                } else {
                    None
                }
            })
        })
        .expect("No guard found!");

    (map, guard)
}

fn change_direction(guard: char) -> char {
    match guard {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => unreachable!(),
    }
}

fn get_direction(guard: char) -> (i32, i32) {
    match guard {
        '^' => (0, -1),
        '>' => (1, 0),
        'v' => (0, 1),
        '<' => (-1, 0),
        _ => unreachable!(),
    }
}

fn traverse_map(
    map: &mut Vec<Vec<char>>,
    x: &mut usize,
    y: &mut usize,
    guard: &mut char,
) -> Option<u32> {
    let mut path = vec![];
    while let Some(pos) = map.get(*y).get_or_insert(&vec![]).get(*x) {
        if path.contains(&(*x, *y, *guard)) {
            return None;
        } else if pos == &'#' {
            let (dx, dy) = get_direction(*guard);
            *x = x.wrapping_sub(dx as usize);
            *y = y.wrapping_sub(dy as usize);
            *guard = change_direction(*guard);
        }

        path.push((*x, *y, *guard));

        map[*y][*x] = 'X';

        let (dx, dy) = get_direction(*guard);
        *x = x.wrapping_add(dx as usize);
        *y = y.wrapping_add(dy as usize);
    }

    Some(
        map.iter()
            .map(|l| l.iter().fold(0, |acc, c| acc + (c == &'X') as u32))
            .sum(),
    )
}

#[aoc(day6, part1)]
fn part1(input: &(Vec<Vec<char>>, (usize, usize, char))) -> u32 {
    let mut map = input.0.to_owned();
    let (mut x, mut y, mut guard) = input.1.to_owned();
    traverse_map(&mut map, &mut x, &mut y, &mut guard).expect("Loop in input!")
}

#[aoc(day6, part2)]
fn part2(input: &(Vec<Vec<char>>, (usize, usize, char))) -> u32 {
    let mut loops = 0;
    let mut map = input.0.clone();
    for (nx, ny) in input.0.iter().enumerate().flat_map(|(y, v)| {
        v.iter()
            .enumerate()
            .map(move |(x, _)| (x.clone(), y.clone()))
    }) {
        let (mut x, mut y, mut guard) = input.1.clone();
        let old = map[ny][nx];
        map[ny][nx] = '#';
        loops += traverse_map(&mut map, &mut x, &mut y, &mut guard).is_none() as u32;
        map[ny][nx] = old;
    }

    loops
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&&parse1(INPUT)), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&&parse1(INPUT)), 6);
    }
}
