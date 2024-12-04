use std::ops::Sub;

use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;
use pathfinding::matrix::Matrix;

type Output = usize;
type Input = Matrix<char>;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Input {
    Matrix::from_rows(
        input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>()),
    )
    .unwrap()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Input) -> Output {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == 'X')
                .map(move |(x, _)| (y, x))
        })
        .map(|pos| count_xmas(pos, input))
        .sum()
}

fn count_xmas(pos: (usize, usize), matrix: &Input) -> usize {
    matrix
        .neighbours(pos, true)
        .filter(|pos| *matrix.get(*pos).expect("Element in matrix not in matrix") == 'M')
        .map(|(y, x)| (y as isize - pos.0 as isize, x as isize - pos.1 as isize))
        .filter(|dir| {
            matrix
                .in_direction(pos, *dir)
                .map(|next_pos| {
                    *matrix
                        .get(next_pos)
                        .expect("Element in matrix not in matrix")
                })
                .take(3)
                .collect::<String>()
                == "MAS"
        })
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Input) -> Output {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == 'A')
                .map(move |(x, _)| (y, x))
        })
        .filter(|pos| is_x_mas(*pos, input))
        .inspect(|pos| println!("{:?}", pos))
        .count()
}

fn is_x_mas(pos: (usize, usize), matrix: &Input) -> bool {
    let neighbours = matrix
        .neighbours(pos, true)
        .map(|neigh| *matrix.get(neigh).unwrap())
        .collect::<Vec<char>>();
    if neighbours.len() != 8 {
        return false;
    }
    (neighbours[0] == 'M' && neighbours[2] == 'S' && neighbours[5] == 'M' && neighbours[7] == 'S')
        || (neighbours[0] == 'M'
            && neighbours[2] == 'M'
            && neighbours[5] == 'S'
            && neighbours[7] == 'S')
        || (neighbours[0] == 'S'
            && neighbours[2] == 'M'
            && neighbours[5] == 'S'
            && neighbours[7] == 'M')
        || (neighbours[0] == 'S'
            && neighbours[2] == 'S'
            && neighbours[5] == 'M'
            && neighbours[7] == 'M')
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
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
    }

    #[test]
    fn samples_part1() {
        assert_eq!(18, solve_part1(&input_generator(sample())));
    }

    #[test]
    fn samples_part2() {
        assert_eq!(9, solve_part2(&input_generator(sample())));
    }
}
