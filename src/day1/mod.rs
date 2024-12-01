use anyhow::{bail, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::multiunzip;
#[allow(unused)]
use itertools::Itertools;

type Output = i64;
type Input = (Vec<i64>, Vec<i64>);

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Result<Input> {
    Ok(multiunzip(input
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
        .collect::<Result<Vec<(i64, i64)>>>()?
        ))
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Input) -> Result<Output> {
    Ok(input.0
        .iter()
        .sorted()
        .zip(input.1.iter().sorted())
        .map(|(a, b)| (a - b).abs())
        .sum())
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Input) -> Result<Output> {
    Ok(input.0
        .iter()
        .map(|num| input.1.iter().filter(|n| num == *n).count() as i64 * num)
        .sum())
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
        assert_eq!(11, solve_part1(&input_generator(sample()).unwrap()).unwrap());
    }

    #[test]
    fn samples_part2() {
        assert_eq!(11, solve_part1(&input_generator(sample()).unwrap()).unwrap());
    }
}
