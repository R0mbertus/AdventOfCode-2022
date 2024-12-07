use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
fn parse1(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|l| {
            let (result, numbers) = l.split_once(": ").expect("Malformed input!");

            (
                result.parse().unwrap(),
                numbers.split(" ").map(|n| n.parse().unwrap()).collect(),
            )
        })
        .collect()
}

fn solve(concat: bool, target: &usize, result: usize, list: &[usize]) -> bool {
    if list.len() > 0 {
        let solve_pre = |r, l| solve(concat, target, r, l);
        solve_pre(result + list[0], &list[1..])
            || solve_pre(result * list[0], &list[1..])
            || (concat
                && solve_pre(
                    (result.to_string() + &list[0].to_string()).parse().unwrap(),
                    &list[1..],
                ))
    } else {
        target == &result
    }
}

#[aoc(day7, part1)]
fn part1(input: &[(usize, Vec<usize>)]) -> usize {
    input
        .iter()
        .filter_map(|(e, l)| solve(false, e, l[0], &l[1..]).then(|| e))
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[(usize, Vec<usize>)]) -> usize {
    input
        .iter()
        .filter_map(|(e, l)| solve(true, e, l[0], &l[1..]).then(|| e))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&&parse1(INPUT)), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&&parse1(INPUT)), 11387);
    }
}
