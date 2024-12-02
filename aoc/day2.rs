use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse1(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|i| i.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|v| {
            let comp = if v[0] < v[1] { u32::lt } else { u32::gt };
            v.windows(2)
                .all(|w| comp(&w[0], &w[1]) && (w[0].abs_diff(w[1]) < 4)) as u32
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|v| {
            v.iter().enumerate().any(|(i, _)| {
                let (mut l, mut r) = (vec![0; i], vec![0; v.len() - (i + 1)]);
                l.clone_from_slice(&v[..i]);
                r.clone_from_slice(&v[(i + 1)..]);
                l.append(&mut r);

                let comp = if l[0] < l[1] { u32::lt } else { u32::gt };
                l.windows(2)
                    .all(|w| (comp(&w[0], &w[1]) && w[0].abs_diff(w[1]) < 4))
            }) as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&&parse1(INPUT)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&&parse1(INPUT)), 4);
    }
}
