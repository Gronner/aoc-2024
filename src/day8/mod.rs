use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;

use crate::utils::point::Point;

type Output = usize;
type Input = (HashMap<char, Vec<Point>>, isize, isize);

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Input {
    let y_dim = input.lines().count() as isize;
    let x_dim = input.lines().next().unwrap().len() as isize;
    let mut map = HashMap::new();
    input.lines().enumerate().for_each(|(y, row)| {
        row.chars()
            .enumerate()
            .filter(|(_, c)| *c != '.')
            .for_each(|(x, c)| {
                map.entry(c)
                    .and_modify(|antennas: &mut Vec<Point>| {
                        antennas.push(Point::from((x as isize, y as isize)))
                    })
                    .or_insert(vec![Point::from((x as isize, y as isize))]);
            })
    });
    (map, x_dim, y_dim)
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Input) -> Output {
    let (map, x_dim, y_dim) = input;
    let mut antinodes = HashSet::new();
    for antennas in map.values() {
        let pairs = antennas
            .iter()
            .combinations(2)
            .filter(|pairs| pairs[0] != pairs[1])
            .map(|pairs| ((*pairs[0], *pairs[1]), *pairs[0] - *pairs[1]))
            .collect::<Vec<((Point, Point), Point)>>();
        for ((antenna_a, antenna_b), distance) in pairs {
            let antinode_a = antenna_a - distance * 2;
            let antinode_b = antenna_b + distance * 2;
            if antinode_a.in_map(*x_dim, *y_dim) {
                antinodes.insert(antinode_a);
            }

            if antinode_b.in_map(*x_dim, *y_dim) {
                antinodes.insert(antinode_b);
            }
        }
    }
    antinodes.len()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Input) -> Output {
    let (map, x_dim, y_dim) = input;
    let mut antinodes = HashSet::new();
    for antennas in map.values() {
        let pairs = antennas
            .iter()
            .combinations(2)
            .filter(|pairs| pairs[0] != pairs[1])
            .map(|pairs| ((*pairs[0], *pairs[1]), *pairs[0] - *pairs[1]))
            .collect::<Vec<((Point, Point), Point)>>();
        for ((antenna_a, antenna_b), distance) in pairs {
            let mut i = 0;
            loop {
                let antinode_a = antenna_a - distance * i;
                if antinode_a.in_map(*x_dim, *y_dim) {
                    antinodes.insert(antinode_a);
                } else {
                    break;
                }
                i += 1;
            }

            i = 0;
            loop {
                let antinode_b = antenna_b + distance * i;
                if antinode_b.in_map(*x_dim, *y_dim) {
                    antinodes.insert(antinode_b);
                } else {
                    break;
                }
                i += 1;
            }
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
