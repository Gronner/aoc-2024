use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::{FxHashMap, FxHashSet};
#[allow(unused)]
use itertools::Itertools;

type Num = i64;
type Output = Num;
type Input = Vec<Num>;

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.parse::<Num>().unwrap())
        .collect::<Vec<Num>>()
}

fn generate(mut secret: Num) -> Num {
    secret ^= secret << 6; // secret * 64 (aka 2^6)
    secret &= 0xFFFFFF; // % 16777216
    secret ^= secret >> 5; // secret / / 32 (aka 2^5)
    secret &= 0xFFFFFF; // % 16777216
    secret ^= secret << 11; // * 2048
    secret &= 0xFFFFFF; // % 16777216
    secret
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &Input) -> Output {
    input
        .iter()
        .map(|n| (0..2000).fold(*n, |sec, _| generate(sec)))
        .sum()
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &Input) -> Output {
    let changes = input
        .iter()
        .map(|n| {
            (0..2000).fold(vec![*n], |mut store, _| {
                store.push(generate(*store.last().unwrap()));
                store
            })
        })
        .map(|sec_nums| {
            sec_nums
                .iter()
                .map(|sec| sec % 10)
                .tuple_windows()
                .map(|(prev, cur)| (cur, cur - prev))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut buy_options = FxHashMap::default();
    for change in changes {
        let mut previous_pats = FxHashSet::default();
        for sequence in change.windows(4) {
            let pattern = (sequence[0].1, sequence[1].1, sequence[2].1, sequence[3].1);
            if !previous_pats.contains(&pattern) {
                buy_options
                    .entry(pattern)
                    .and_modify(|v| *v += sequence[3].0)
                    .or_insert(sequence[3].0);
                previous_pats.insert(pattern);
            }
        }
    }
    *buy_options.values().max().unwrap()
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

    fn sample1() -> &'static str {
        "1
10
100
2024
"
    }

    #[test]
    fn samples_part1() {
        assert_eq!(37327623, solve_part1(&input_generator(sample1())));
    }

    fn sample2() -> &'static str {
        "1
2
3
2024"
    }

    #[test]
    fn samples_part2() {
        assert_eq!(23, solve_part2(&input_generator(sample2())));
    }
}
