use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;
use pathfinding::{
    matrix::Matrix,
    prelude::{bfs_reach, yen},
};

type Num = usize;
type Output = Num;
type Input = Matrix<Num>;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Input {
    Matrix::from_rows(
        input
            .lines()
            .map(|line| line.chars().map(|n| n.to_digit(10).unwrap() as Num)),
    )
    .unwrap()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Input) -> Output {
    input
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, n)| **n == 0)
                .map(move |(c, _)| (r, c))
                .collect::<Vec<(usize, usize)>>()
        })
        .map(|trailhead| {
            bfs_reach(trailhead, |cur| {
                input
                    .neighbours(*cur, false)
                    .filter(|neigh| *input.get(*neigh).unwrap() == *input.get(*cur).unwrap() + 1)
                    .collect::<Vec<_>>()
            })
            .filter(|step| *input.get(*step).unwrap() == 9)
            .count()
        })
        .sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Input) -> Output {
    input
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, n)| **n == 0)
                .map(move |(c, _)| (r, c))
                .collect::<Vec<(usize, usize)>>()
        })
        .map(|trailhead| {
            yen(
                &trailhead,
                |cur| {
                    input
                        .neighbours(*cur, false)
                        .filter(|neigh| {
                            *input.get(*neigh).unwrap() == *input.get(*cur).unwrap() + 1
                        })
                        .map(|neigh| (neigh, 1))
                        .collect::<Vec<_>>()
                },
                |step| *input.get(*step).unwrap() == 9,
                1000,
            )
            .len()
        })
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
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
    }

    #[test]
    fn samples_part1() {
        assert_eq!(36, solve_part1(&input_generator(sample())));
    }

    #[test]
    fn samples_part2() {
        assert_eq!(81, solve_part2(&input_generator(sample())));
    }
}
