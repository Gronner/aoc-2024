use std::cmp::max_by_key;

use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::{FxHashMap, FxHashSet};
#[allow(unused)]
use itertools::Itertools;

type Output = usize;
type Output2 = String;
type Input = FxHashMap<String, FxHashSet<String>>;

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.split_once("-").expect("No `-` delimiter"))
        .fold(FxHashMap::default(), |mut graph, (pc_a, pc_b)| {
            graph
                .entry(pc_a.to_string())
                .and_modify(|connected| {
                    connected.insert(pc_b.to_string());
                })
                .or_insert(FxHashSet::from_iter(vec![pc_b.to_string()]));
            graph
                .entry(pc_b.to_string())
                .and_modify(|connected| {
                    connected.insert(pc_a.to_string());
                })
                .or_insert(FxHashSet::from_iter(vec![pc_a.to_string()]));
            graph
        })
}

/// Finds the maximum cliques in a graph
fn bron_kerbosch(
    r: FxHashSet<String>,
    mut p: FxHashSet<String>,
    mut x: FxHashSet<String>,
    n: &FxHashMap<String, FxHashSet<String>>,
) -> FxHashSet<String> {
    if p.is_empty() && x.is_empty() {
        return r;
    }
    let mut max_r = FxHashSet::default();
    for vertex in p.clone() {
        let mut r_v = FxHashSet::default();
        r_v.insert(vertex.clone());
        let new_max_r = bron_kerbosch(
            r.union(&r_v).cloned().collect(),
            p.intersection(&n[&vertex]).cloned().collect(),
            x.intersection(&n[&vertex]).cloned().collect(),
            n,
        );
        p = p
            .difference(&FxHashSet::from_iter(vec![vertex.clone()]))
            .cloned()
            .collect();
        x = x
            .union(&FxHashSet::from_iter(vec![vertex.clone()]))
            .cloned()
            .collect();
        max_r = max_by_key(max_r, new_max_r, |set| set.len());
    }
    max_r
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &Input) -> Output {
    input
        .keys()
        .combinations(3)
        .filter(|pcs| pcs.iter().any(|pc| pc.starts_with('t')))
        .filter(|pcs| {
            input[pcs[0]].contains(pcs[1])
                && input[pcs[1]].contains(pcs[2])
                && input[pcs[2]].contains(pcs[0])
        })
        .count()
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &Input) -> Output2 {
    bron_kerbosch(
        FxHashSet::default(),
        FxHashSet::from_iter(input.keys().cloned()),
        FxHashSet::default(),
        input,
    )
    .iter()
    .sorted()
    .join(",")
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(&input_generator(input))
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(&input_generator(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        "kh-tc
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
td-yn"
    }

    #[test]
    fn samples_part1() {
        assert_eq!(7, solve_part1(&input_generator(sample())));
    }

    #[test]
    fn samples_part2() {
        assert_eq!("co,de,ka,ta", solve_part2(&input_generator(sample())));
    }
}
