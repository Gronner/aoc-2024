use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::all;
#[allow(unused)]
use itertools::Itertools;

type Output = usize;
type Input = Vec<Vec<i64>>;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Result<Input> {
    Ok(input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|n| {
                    n.parse::<i64>()
                        .with_context(|| format!("Not a number: {}", n))
                })
                .collect()
        })
        .collect::<Result<Vec<Vec<i64>>>>()?)
}

pub fn is_safe(report: &[i64]) -> bool {
    let safe_pairs = report
        .windows(2)
        .map(|pair| pair[0] - pair[1])
        .filter(|delta| delta.abs() != 0 && delta.abs() <= 3)
        .collect::<Vec<i64>>();

    (all(&safe_pairs, |level| *level < 0) || all(&safe_pairs, |level| *level > 0))
        && safe_pairs.len() == report.len() - 1
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Input) -> Result<Output> {
    Ok(input
        .iter()
        .filter(|report| is_safe(*report))
        .count())
}

pub fn is_safe_dampened(report: &[i64]) -> bool {
    for i in 0..report.len() {
        let mut report = report.to_vec();
        report.remove(i);
        if is_safe(&report) {
            return true;
        }
    }
    false
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Input) -> Result<Output> {
    Ok(input
        .iter()
        .filter(|report| is_safe_dampened(*report))
        .count())
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(&input_generator(input).unwrap()).unwrap()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(&input_generator(input).unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"
    }

    #[test]
    fn samples_part1() {
        assert_eq!(2, solve_part1(&input_generator(sample()).unwrap()).unwrap());
    }

    #[test]
    fn samples_part2() {
        assert_eq!(4, solve_part2(&input_generator(sample()).unwrap()).unwrap());
    }
}
