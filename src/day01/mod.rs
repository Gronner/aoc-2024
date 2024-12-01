use anyhow::{bail, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::multiunzip;
#[allow(unused)]
use itertools::Itertools;

type Output = i64;
type Input = Vec<(i64, i64)>;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|line| line.split_once("   "))
        .map(|split| {
            if let Some((first, second)) = split {
                if let (Ok(first), Ok(second)) = (first.parse::<i64>(), second.parse::<i64>()) {
                    Ok((first, second))
                } else {
                    bail!("At least one not parsable to i64: {}, {}", first, second)
                }
            } else {
                bail!("No split found");
            }
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Input) -> Result<Output> {
    let input = input.clone();
    let (mut first, mut second): (Vec<_>, Vec<_>) = multiunzip(input);
    first.sort();
    second.sort();
    Ok(first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| (a - b).abs())
        .sum())
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Input) -> Result<Output> {
    let (first, second): (Vec<_>, Vec<_>) = multiunzip(input.clone());
    Ok(first
        .iter()
        .map(|num| second.iter().filter(|n| num == *n).count() as i64 * num)
        .sum())
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn samples_part1() {}

    #[test]
    fn samples_part2() {}
}
