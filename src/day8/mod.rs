use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;

use crate::utils::point::Point;

type Output = usize;
type Input = (Vec<((Point, Point), Point)>, isize, isize);

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Input {
    let y_dim = input.lines().count() as isize;
    let x_dim = input.lines().next().unwrap().len() as isize;
    let distances = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| (Point::from((x as isize, y as isize)), c))
        })
        .fold(HashMap::new(), |mut map, (pos, freq)| {
            map.entry(freq)
                .and_modify(|positions: &mut Vec<Point>| positions.push(pos))
                .or_insert(vec![pos]);
            map
        })
        .values()
        .flat_map(|positions| {
            positions
                .iter()
                .tuple_combinations()
                .filter(|(ant_a, ant_b)| ant_a != ant_b)
                .map(|(ant_a, ant_b)| ((*ant_a, *ant_b), ant_a - ant_b))
        })
        .collect();

    (distances, x_dim, y_dim)
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Input) -> Output {
    let (distances, x_dim, y_dim) = input;
    let mut antinodes = HashSet::new();
    for ((antenna_a, antenna_b), distance) in distances {
        let antinode_a = *antenna_a - distance * 2;
        if antinode_a.in_map(*x_dim, *y_dim) {
            antinodes.insert(antinode_a);
        }
        let antinode_b = *antenna_b + distance * 2;

        if antinode_b.in_map(*x_dim, *y_dim) {
            antinodes.insert(antinode_b);
        }
    }
    antinodes.len()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Input) -> Output {
    let (distances, x_dim, y_dim) = input;
    let mut antinodes = HashSet::new();
    for ((antenna_a, antenna_b), distance) in distances {
        let mut i = 0;
        loop {
            let antinode_a = *antenna_a - distance * i;
            if antinode_a.in_map(*x_dim, *y_dim) {
                antinodes.insert(antinode_a);
            } else {
                break;
            }
            i += 1;
        }

        i = 0;
        loop {
            let antinode_b = *antenna_b + distance * i;
            if antinode_b.in_map(*x_dim, *y_dim) {
                antinodes.insert(antinode_b);
            } else {
                break;
            }
            i += 1;
        }
    }
    antinodes.len()
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
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
    }

    #[test]
    fn samples_part1() {
        assert_eq!(14, solve_part1(&input_generator(sample())));
    }

    #[test]
    fn samples_part2() {
        assert_eq!(34, solve_part2(&input_generator(sample())));
    }
}
