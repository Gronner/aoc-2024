use std::str::FromStr;

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;

type Output = usize;
type Book = Vec<usize>;
type Input = (Vec<Rule>, Vec<Book>);

#[derive(Clone, Copy, Debug)]
pub struct Rule {
    before: usize,
    after: usize,
}

impl Rule {
    fn matches(&self, vec: &Book) -> bool {
        if let (Some(b), Some(a)) = (
            vec.iter().position(|n| *n == self.before),
            vec.iter().position(|n| *n == self.after),
        ) {
            a > b
        } else {
            true
        }
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .split('|')
            .map(|n| n.parse::<usize>().context("Failed to parse number"))
            .collect::<Result<Vec<usize>, Self::Err>>()?;
        Ok(Rule {
            before: numbers[0],
            after: numbers[1],
        })
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Result<Input> {
    let input = input.split("\n\n").collect::<Vec<&str>>();

    let rules = input[0]
        .lines()
        .map(Rule::from_str)
        .collect::<Result<Vec<Rule>>>()?;

    let books = input[1]
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<usize>().context("Failed to parse number"))
                .collect::<Result<Vec<usize>>>()
        })
        .collect::<Result<Vec<Vec<usize>>>>()?;

    Ok((rules, books))
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Input) -> Output {
    let (rules, books) = input;

    books
        .iter()
        .filter(|book| rules.iter().all(|rule| rule.matches(book)))
        .map(|book| book[book.len() / 2])
        .sum()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Input) -> Output {
    let (rules, books) = input;

    books
        .iter()
        .filter(|book| !rules.iter().all(|rule| rule.matches(book)))
        .map(|book| {
            book.iter()
                .sorted_by(|a, b| order(**a, **b, rules))
                .collect::<Vec<_>>()
        })
        .map(|book| book[book.len() / 2])
        .sum()
}

fn order(a: usize, b: usize, rules: &[Rule]) -> std::cmp::Ordering {
    if rules.iter().any(|rule| rule.before == b && rule.after == a) {
        std::cmp::Ordering::Greater
    } else if rules.iter().any(|rule| rule.before == a && rule.after == b) {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Equal
    }
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(&input_generator(input).unwrap())
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(&input_generator(input).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"
    }

    #[test]
    fn samples_part1() {
        assert_eq!(143, solve_part1(&input_generator(sample()).unwrap()));
    }

    #[test]
    fn samples_part2() {
        assert_eq!(123, solve_part2(&input_generator(sample()).unwrap()));
    }
}
