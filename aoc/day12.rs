use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day12)]
fn parse1(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|c| c.chars().collect()).collect()
}

fn get_area_perimeter(
    input: &[Vec<char>],
    visited: &mut [Vec<bool>],
    x: usize,
    y: usize,
    c: &char,
) -> (usize, usize) {
    if x >= input[0].len() || y >= input.len() || input[y][x] != *c {
        (0, 1)
    } else if visited[y][x] {
        (0, 0)
    } else {
        visited[y][x] = true;
        let (mut area, mut perimeter) = (1, 0);
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let (nx, ny) = (x.wrapping_add_signed(*dx), y.wrapping_add_signed(*dy));
            let (a, p) = get_area_perimeter(input, visited, nx, ny, c);
            area += a;
            perimeter += p;
        }
        (area, perimeter)
    }
}

#[aoc(day12, part1)]
fn part1(input: &[Vec<char>]) -> usize {
    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    let mut total = 0;
    for (y, r) in input.iter().enumerate() {
        for (x, c) in r.iter().enumerate() {
            if !visited[y][x] {
                let (area, perimeter) = get_area_perimeter(input, &mut visited, x, y, c);
                total += area * perimeter;
            }
        }
    }
    total
}

fn mark_side_visited(
    input: &[Vec<char>],
    side: &mut [Vec<Vec<(isize, isize)>>],
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    c: &char,
) {
    if dx != 0 {
        for i in y..0 {
            if &input[i][x] != c {
                break;
            }
            side[i][x].push((dx, dy));
        }
        for i in y..input.len() {
            if &input[i][x] != c {
                break;
            }
            side[i][x].push((dx, dy));
        }
    } else {
        for i in x..0 {
            if &input[i][x] != c {
                break;
            }
            side[y][i].push((dx, dy));
        }
        for i in x..input[0].len() {
            if &input[y][i] != c {
                break;
            }
            side[y][i].push((dx, dy));
        }
    }
}

fn get_area_sides(
    input: &[Vec<char>],
    visited: &mut [Vec<bool>],
    side: &mut [Vec<Vec<(isize, isize)>>],
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    c: &char,
) -> (usize, usize) {
    if x >= input[0].len() || y >= input.len() || input[y][x] != *c {
        let (nx, ny) = (x.wrapping_sub(dx as usize), y.wrapping_sub(dy as usize));
        if side[ny][nx].iter().any(|&s| s == (dx, dy))
            || (y < input.len() && x < input[0].len() && input[y][x] == *c)
        {
            (0, 0)
        } else {
            println!("{} {} {} {} {:?}", dx, dy, nx, ny, side[ny][nx]);
            mark_side_visited(input, side, nx, ny, dx, dy, c);
            (0, 1)
        }
    } else if visited[y][x] {
        (0, 0)
    } else {
        visited[y][x] = true;
        let (mut area, mut sides) = (1, 0);
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let (nx, ny) = (x.wrapping_add_signed(*dx), y.wrapping_add_signed(*dy));
            let (a, s) = get_area_sides(input, visited, side, nx, ny, *dx, *dy, c);
            area += a;
            sides += s;
        }
        (area, sides)
    }
}

#[aoc(day12, part2)]
fn part2(input: &[Vec<char>]) -> usize {
    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    let mut total = 0;
    for (y, r) in input.iter().enumerate() {
        for (x, c) in r.iter().enumerate() {
            if !visited[y][x] {
                let mut side = vec![vec![vec![]; input[0].len()]; input.len()];
                let (area, sides) = get_area_sides(input, &mut visited, &mut side, x, y, 0, 0, c);
                println!("{} {} {}", area, sides, c);
                total += area * sides;
                for r in side.iter() {
                    for c in r.iter() {
                        print!("{:?} ", c);
                    }
                    println!();
                }
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&&parse1(INPUT1)), 1930);
    }

    const INPUT2: &str = "AAAA
BBCD
BBCC
EEEC";

    const INPUT3: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    #[test]
    fn part2_example() {
        assert_eq!(part2(&&parse1(INPUT2)), 80);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2(&&parse1(INPUT3)), 236);
    }
}
