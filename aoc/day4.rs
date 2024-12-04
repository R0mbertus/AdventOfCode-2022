use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse1(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.bytes().collect()).collect()
}

fn get_xmas_around(input: &[Vec<u8>], x: i32, y: i32) -> u32 {
    let mut total = 0;
    let xmas = "XMAS".as_bytes();
    'outer: for dir in [
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
    ] {
        for i in 0..4 {
            if let Some(r) = input.get((y + dir.0 * i) as usize) {
                if let Some(c) = r.get((x + dir.1 * i) as usize) {
                    if c != &xmas[i as usize] {
                        continue 'outer;
                    } else if i == 3 {
                        total += 1;
                    }
                }
            }
        }
    }
    total
}

fn get_mas_around(input: &[Vec<u8>], x: i32, y: i32) -> u32 {
    let (xu, yu) = (x as usize, y as usize);
    if xu == 0 || xu == (input[0].len() - 1) || yu == 0 || yu == (input.len() - 1) {
        return 0;
    }

    let tl = input[yu - 1][xu - 1];
    let tr = input[yu - 1][xu + 1];
    let bl = input[yu + 1][xu - 1];
    let br = input[yu + 1][xu + 1];

    if tl == tr && (tl == b'M' || tl == b'S') {
        let other = if tl == b'S' { b'M' } else { b'S' };
        (bl == br && bl == other) as u32
    } else if tl == bl && (tl == b'M' || tl == b'S') {
        let other = if tl == b'S' { b'M' } else { b'S' };
        (tr == br && tr == other) as u32
    } else {
        0
    }
}

fn get_occurence(input: &[Vec<u8>], comp: &u8, check_func: fn(&[Vec<u8>], i32, i32) -> u32) -> u32 {
    let mut total = 0;
    for (y, row) in input.iter().enumerate() {
        for (x, e) in row.iter().enumerate() {
            if e == comp {
                total += check_func(input, x as i32, y as i32);
            }
        }
    }
    total
}

#[aoc(day4, part1)]
fn part1(input: &[Vec<u8>]) -> u32 {
    get_occurence(input, &b'X', get_xmas_around)
}

#[aoc(day4, part2)]
fn part2(input: &[Vec<u8>]) -> u32 {
    get_occurence(input, &b'A', get_mas_around)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&&parse1(INPUT)), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&&parse1(INPUT)), 9);
    }
}
