use std::cmp::max_by_key;

use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::{FxHashMap, FxHashSet};
#[allow(unused)]
use itertools::Itertools;

type Output = usize;
type Output2 = String;
type Input = Vec<(String, String)>;

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.split_once("-").expect("No `-` delimiter"))
        .map(|(a, b)| (a.min(b), a.max(b)))
        .sorted_by(|(aa, ab), (ba, bb)| aa.cmp(ba).then(ab.cmp(bb)))
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect()
}

/// Finds the maximum cliques in a graph
fn bron_kerbosch<'a>(
    r: FxHashSet<&'a str>,
    mut p: FxHashSet<&'a str>,
    mut x: FxHashSet<&'a str>,
    n: &FxHashMap<&'a str, FxHashSet<&'a str>>,
) -> FxHashSet<&'a str> {
    if p.is_empty() && x.is_empty() {
        return r;
    }
    let mut max_r = FxHashSet::default();
    for vertex in p.clone() {
        let mut r_v = FxHashSet::default();
        r_v.insert(vertex);
        let new_max_r = bron_kerbosch(
            r.union(&r_v).cloned().collect(),
            p.intersection(&n[vertex]).cloned().collect(),
            x.intersection(&n[vertex]).cloned().collect(),
            n,
        );
        p = p
            .difference(&FxHashSet::from_iter(vec![vertex]))
            .cloned()
            .collect();
        x = x
            .union(&FxHashSet::from_iter(vec![vertex]))
            .cloned()
            .collect();
        max_r = max_by_key(max_r, new_max_r, |set| set.len());
    }
    max_r
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &Input) -> Output {
    let input: Vec<(&str, &str)> = input
        .iter()
        .map(|(a, b)| (a.as_str(), b.as_str()))
        .collect();
    input[..input.len() - 1]
        .iter()
        .cartesian_product(input[1..].iter())
        .filter_map(|(conn_a, conn_b)| {
            if conn_a.0 == conn_b.0 && input.contains(&(conn_a.1, conn_b.1)) {
                Some((conn_a.0, conn_a.1, conn_b.1))
            } else {
                None
            }
        })
        .filter(|pcs| [pcs.0, pcs.1, pcs.2].iter().any(|pc| pc.starts_with('t')))
        .count()
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &Input) -> Output2 {
    let map = input
        .iter()
        .fold(FxHashMap::default(), |mut graph, (pc_a, pc_b)| {
            graph
                .entry(pc_a.as_str())
                .and_modify(|connected: &mut FxHashSet<_>| {
                    connected.insert(pc_b.as_str());
                })
                .or_insert({
                    let mut map = FxHashSet::default();
                    map.insert(pc_b.as_str());
                    map
                });
            graph
                .entry(pc_b.as_str())
                .and_modify(|connected| {
                    connected.insert(pc_a.as_str());
                })
                .or_insert(FxHashSet::from_iter(vec![pc_a.as_str()]));
            graph
        });
    bron_kerbosch(
        FxHashSet::default(),
        FxHashSet::from_iter(map.keys().copied()),
        FxHashSet::default(),
        &map,
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
