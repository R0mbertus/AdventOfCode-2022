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

fn get_area_corners(
    input: &[Vec<char>],
    visited: &mut [Vec<bool>],
    x: usize,
    y: usize,
    c: &char,
) -> (usize, usize) {
    if x >= input[0].len() || y >= input.len() || input[y][x] != *c || visited[y][x] {
        return (0, 0);
    }

    visited[y][x] = true;
    let (mut area, mut sides) = (1, 0);

    for (dx, dy) in [(-1, -1), (1, -1), (-1, 1), (1, 1)].iter() {
        let (nx, ny) = (x.wrapping_add_signed(*dx), y.wrapping_add_signed(*dy));

        if nx >= input[0].len() || ny >= input.len() {
            if nx >= input[0].len() && ny >= input.len() {
                sides += (nx >= input[0].len() && ny >= input.len()) as usize;
            } else if nx >= input[0].len() && ny < input.len() {
                sides += (input[ny][x] != *c) as usize;
            } else if nx < input[0].len() && ny >= input.len() {
                sides += (input[y][nx] != *c) as usize;
            }
        } else {
            sides += (input[ny][x] != *c && input[y][nx] != *c
                || input[ny][x] == *c && input[y][nx] == *c && input[ny][nx] != *c)
                as usize;
        }
    }

    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        let (nx, ny) = (x.wrapping_add_signed(*dx), y.wrapping_add_signed(*dy));
        let (a, s) = get_area_corners(input, visited, nx, ny, c);
        area += a;
        sides += s;
    }
    (area, sides)
}

#[aoc(day12, part2)]
fn part2(input: &[Vec<char>]) -> usize {
    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    let mut total = 0;
    for (y, r) in input.iter().enumerate() {
        for (x, c) in r.iter().enumerate() {
            if !visited[y][x] {
                let (area, sides) = get_area_corners(input, &mut visited, x, y, c);
                total += area * sides;
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
