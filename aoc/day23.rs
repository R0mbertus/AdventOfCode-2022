use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[aoc_generator(day23)]
fn parse1(input: &str) -> HashMap<String, HashSet<String>> {
    input
        .lines()
        .map(|l| {
            let line = l.to_string();
            let (c1, c2) = line.split_once("-").unwrap();
            (c1.to_string(), c2.to_string())
        })
        .fold(HashMap::new(), |mut acc, (c1, c2)| {
            acc.entry(c1.to_string())
                .or_insert_with(HashSet::new)
                .insert(c2.to_string());
            acc.entry(c2.to_string())
                .or_insert_with(HashSet::new)
                .insert(c1.to_string());
            acc
        })
}

#[aoc(day23, part1)]
fn part1(input: &HashMap<String, HashSet<String>>) -> usize {
    let mut interconnected = HashSet::new();
    for (a, connections) in input.iter() {
        if a.chars().nth(0).unwrap() != 't' {
            continue;
        }

        for b in connections.iter() {
            for c in connections.intersection(&input[b]) {
                let (x, y, z) = [a, b, c].into_iter().sorted().next_tuple().unwrap();

                interconnected.insert((x, y, z));
            }
        }
    }

    interconnected.len()
}

fn bron_kerbosch1(
    r: HashSet<String>,
    mut p: HashSet<String>,
    mut x: HashSet<String>,
    graph: &HashMap<String, HashSet<String>>,
    clique: &mut HashSet<String>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > clique.len() {
            *clique = r;
        }
        return;
    }

    for s in p.clone() {
        bron_kerbosch1(
            r.union(&vec![s.clone()].into_iter().collect())
                .cloned()
                .collect(),
            p.intersection(&graph[&s]).cloned().collect(),
            x.intersection(&graph[&s]).cloned().collect(),
            graph,
            clique,
        );

        p.remove(&s);
        x.insert(s.clone());
    }
}

#[aoc(day23, part2)]
fn part2(input: &HashMap<String, HashSet<String>>) -> String {
    let r: HashSet<String> = HashSet::new();
    let p: HashSet<String> = input.keys().cloned().collect();
    let x: HashSet<String> = HashSet::new();
    let mut clique: HashSet<String> = HashSet::new();

    bron_kerbosch1(r, p, x, &input.clone(), &mut clique);

    clique.iter().sorted().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&&parse1(INPUT1)), 7);
    }

    const INPUT2: &str = "ka-co
ta-co
de-co
ta-ka
de-ta
ka-de";

    #[test]
    fn part1_example2() {
        assert_eq!(part2(&&parse1(INPUT2)), "co,de,ka,ta");
    }
}
