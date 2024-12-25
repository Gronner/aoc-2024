use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;

type Num = i64;
type Output = usize;
type Input = Vec<Schematic>;

#[derive(Debug)]
pub enum Schematic {
    Key(Vec<Num>),
    Lock(Vec<Num>),
}

fn count_pin_len(pin_map: &str) -> Schematic {
    let mut pinout = vec![-1; 5];
    pin_map.lines().for_each(|line| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .for_each(|(col, _)| pinout[col] += 1)
    });
    if pin_map.lines().next().unwrap().chars().all(|c| c == '.') {
        Schematic::Key(pinout)
    } else {
        Schematic::Lock(pinout)
    }
}

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|pinning| count_pin_len(pinning))
        .collect()
}

#[aoc(day25, part1)]
pub fn solve_part1(input: &Input) -> Output {
    let locks: Vec<Vec<Num>> = input
        .iter()
        .filter_map(|pin_map| {
            if let Schematic::Lock(pinning) = pin_map {
                Some(pinning.clone())
            } else {
                None
            }
        })
        .collect();
    let keys: Vec<Vec<Num>> = input
        .iter()
        .filter_map(|pin_map| {
            if let Schematic::Key(pinning) = pin_map {
                Some(pinning.clone())
            } else {
                None
            }
        })
        .collect();

    locks
        .iter()
        .cartesian_product(keys.iter())
        .filter(|(lock, key)| {
            lock.iter()
                .interleave(key.iter())
                .array_chunks()
                .all(|pair: [&i64; 2]| pair[0] + pair[1] <= 5)
        })
        .count()
}

#[aoc(day25, part2)]
pub fn solve_part2(input: &Input) -> Output {
    0
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
        "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"
    }

    #[test]
    fn samples_part1() {
        assert_eq!(3, solve_part1(&input_generator(sample())));
    }

    #[test]
    fn samples_part2() {
        assert_eq!(31, solve_part2(&input_generator(sample())));
    }
}
