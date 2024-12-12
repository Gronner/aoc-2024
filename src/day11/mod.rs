use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;

type Num = usize;
type Output = Num;
type Input = HashMap<Num, Num>;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Input {
    input.split_whitespace().fold(HashMap::new(), |mut map, n| {
        map.entry(n.parse().unwrap())
            .and_modify(|count| *count += 1)
            .or_insert(1);
        map
    })
}

fn transform_stone(num: Num) -> (Num, Option<Num>) {
    match num {
        0 => (1, None),
        n if (num.ilog10() + 1) % 2 == 0 => {
            let digits = num.ilog10() + 1; // As num must not be 0 it can only be precomputed here
            (
                n / 10_usize.pow(digits / 2),
                Some(n - (n / 10_usize.pow(digits / 2) * 10_usize.pow(digits / 2))),
            )
        }
        n => (n * 2024, None),
    }
}

fn blink(map: Input) -> Input {
    map.iter()
        .fold(HashMap::new(), |mut new_map, (num, count)| {
            let next = transform_stone(*num);
            new_map
                .entry(next.0)
                .and_modify(|old_count| *old_count += *count)
                .or_insert(*count);
            if let Some(next2) = next.1 {
                new_map
                    .entry(next2)
                    .and_modify(|old_count| *old_count += *count)
                    .or_insert(*count);
            }
            new_map
        })
}

fn observ(map: &Input, time: Num) -> Output {
    let mut map = map.clone();
    for _ in 0..time {
        map = blink(map)
    }
    map.values().sum()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Input) -> Output {
    observ(input, 25)
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Input) -> Output {
    observ(input, 75)
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
        "125 17"
    }

    #[test]
    fn samples_part1() {
        assert_eq!(55312, solve_part1(&input_generator(sample())));
    }

    #[test]
    fn samples_part2() {
        assert_eq!(65601038650482, solve_part2(&input_generator(sample())));
    }
}
