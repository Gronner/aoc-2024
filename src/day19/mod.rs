use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;

use cached::proc_macro::cached;
use cached::SizedCache;

type Output = usize;
type Input = (Vec<String>, Vec<String>);

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Input {
    let (patterns, designs) = input.split_once("\n\n").unwrap();

    let patterns = patterns
        .split(", ")
        .map(|pat| pat.to_string())
        .collect::<Vec<_>>();
    let designs = designs
        .lines()
        .map(|des| des.to_string())
        .collect::<Vec<_>>();
    (patterns, designs)
}

#[cached(
    ty = "SizedCache<String, Option<usize>>",
    create = "{ SizedCache::with_size(100) }",
    convert = r#"{ format!("{}", design) }"#
)]
fn count_feasible(design: &str, patterns: &[String]) -> Option<usize> {
    if design.is_empty() {
        return Some(1);
    }
    let count = patterns
        .iter()
        .filter_map(|pat| design.strip_prefix(pat))
        .filter_map(|stripped| count_feasible(stripped, patterns))
        .sum();
    if count != 0 {
        Some(count)
    } else {
        None
    }
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &Input) -> Output {
    let (patterns, designs) = input;

    designs
        .iter()
        .filter(|des| count_feasible(des, patterns).is_some())
        .count()
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &Input) -> Output {
    let (patterns, designs) = input;

    designs
        .iter()
        .filter_map(|des| count_feasible(des, patterns))
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
        "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
    }

    #[test]
    fn samples_part1() {
        assert_eq!(6, solve_part1(&input_generator(sample())));
    }

    #[test]
    fn samples_part2() {
        assert_eq!(16, solve_part2(&input_generator(sample())));
    }
}
