use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Clone, Debug, Default)]
struct Wire {
    on: bool,
    value: u32,
}

struct Connection {
    in1: String,
    in2: String,
    to: String,
    op: String,
}

#[aoc_generator(day24)]
fn parse1(input: &str) -> (HashMap<String, Wire>, Vec<Connection>) {
    let (wires_str, connections_str) = input.split_once("\n\n").unwrap();

    let mut wires: HashMap<String, Wire> = wires_str.lines().fold(HashMap::new(), |mut acc, l| {
        let (name, value) = l.split_once(": ").unwrap();
        let value = value.parse().unwrap();
        acc.insert(name.to_string(), Wire { on: true, value });
        acc
    });

    let connections: Vec<Connection> = connections_str
        .lines()
        .map(|l| {
            let (from, to) = l.split_once(" -> ").unwrap();
            let (in1, op_str, in2) = from.split_whitespace().collect_tuple().unwrap();

            wires.entry(to.to_string()).or_default();
            wires.entry(in1.to_string()).or_default();
            wires.entry(in2.to_string()).or_default();

            Connection {
                in1: in1.to_string(),
                in2: in2.to_string(),
                to: to.to_string(),
                op: op_str.to_string(),
            }
        })
        .collect();

    (wires, connections)
}

fn get_z_wires(wires: &HashMap<String, Wire>) -> Vec<Wire> {
    wires
        .iter()
        .filter(|(key, _)| key.starts_with("z"))
        .sorted_by_key(|(key, _)| key[1..].parse::<usize>().unwrap())
        .map(|(_, wire)| wire.clone())
        .collect()
}

#[aoc(day24, part1)]
fn part1(input: &(HashMap<String, Wire>, Vec<Connection>)) -> usize {
    let mut wires = input.0.clone();

    loop {
        for conn in input.1.iter() {
            let in1 = wires.get(&conn.in1).unwrap();
            let in2 = wires.get(&conn.in2).unwrap();

            if !in1.on || !in2.on {
                continue;
            }

            wires.insert(
                conn.to.clone(),
                Wire {
                    on: true,
                    value: match conn.op.as_str() {
                        "AND" => in1.value & in2.value,
                        "OR" => in1.value | in2.value,
                        "XOR" => in1.value ^ in2.value,
                        _ => unreachable!(),
                    },
                },
            );
        }

        if get_z_wires(&wires).iter().all(|w| w.on) {
            break;
        }
    }

    let z_wires: String = get_z_wires(&wires)
        .into_iter()
        .map(|wire| char::from_digit(wire.value, 10).unwrap())
        .rev()
        .collect();

    usize::from_str_radix(&z_wires, 2).unwrap()
}

#[aoc(day24, part2)]
fn part2(input: &(HashMap<String, Wire>, Vec<Connection>)) -> String {
    let mut wrong_pairs = vec![];
    let is_xyz = |c: char| c == 'x' || c == 'y' || c == 'z';

    for conn in input.1.iter() {
        if (conn.to.starts_with('z') && conn.op != "XOR" && conn.to != "z45")
            || (conn.op == "XOR"
                && !is_xyz(conn.in1.chars().next().unwrap())
                && !is_xyz(conn.in2.chars().next().unwrap()))
                && !is_xyz(conn.to.chars().next().unwrap())
        {
            wrong_pairs.push(conn.to.clone());
        }

        if conn.op == "XOR" || (conn.op == "AND" && conn.in1 != "x00" && conn.in2 != "x00") {
            for conn_riple in input.1.iter() {
                if (conn.to == conn_riple.in1 || conn.to == conn_riple.in2)
                    && ((conn_riple.op == "OR" && conn.op == "XOR")
                        || (conn_riple.op != "OR" && conn.op == "AND"))
                {
                    wrong_pairs.push(conn.to.clone());
                }
            }
        }
    }

    wrong_pairs
        .into_iter()
        .collect::<HashSet<String>>()
        .iter()
        .sorted()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const INPUT2: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&&parse1(INPUT1)), 4);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&&parse1(INPUT2)), 2024);
    }
}
