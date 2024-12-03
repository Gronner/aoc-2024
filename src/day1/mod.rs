use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap;
use itertools::multiunzip;
#[allow(unused)]
use itertools::Itertools;
use rayon::prelude::*;

type Output = i64;
type Input = (Vec<i64>, Vec<i64>);

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Input {
    multiunzip(
        input
            .lines()
            .flat_map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .tuple_windows()
            })
            .collect::<Vec<(i64, i64)>>(),
    )
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Input) -> Output {
    let mut left = input.0.clone();
    let mut right = input.1.clone();
    left.par_sort_unstable();
    right.par_sort_unstable();
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Input) -> Output {
    let num_count = input.1.iter().fold(FxHashMap::default(), |mut map, num| {
        map.entry(num)
            .and_modify(|frequency| *frequency += 1)
            .or_insert(1);
        map
    });
    input
        .0
        .iter()
        .map(|num| num * num_count.get(num).unwrap_or(&0))
        .sum()
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
        "3   4
4   3
2   5
1   3
3   9
3   3
"
    }

    #[test]
    fn samples_part1() {
        assert_eq!(11, solve_part1(&input_generator(sample())));
    }

    #[test]
    fn samples_part2() {
        assert_eq!(31, solve_part2(&input_generator(sample())));
    }
}
