use std::usize;

use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;
use pathfinding::{grid::Grid, prelude::dijkstra};

type Output = usize;
type Output2 = String;
type Input = (Vec<(usize, usize)>, (usize, usize), usize);

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Input {
    (
        input
            .lines()
            .map(|line| line.split_once(',').unwrap())
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .collect(),
        (70, 70),
        1024,
    )
}

fn create_ram(bytes: &Vec<(usize, usize)>, end: (usize, usize), fallen: usize) -> Grid {
    (0..=end.0)
        .cartesian_product(0..=end.1)
        .filter(|pos| !&bytes[..fallen].contains(pos))
        .collect::<Grid>()
}

fn neighbours(pos: &(usize, usize), ram: &Grid) -> Vec<((usize, usize), usize)> {
    ram.neighbours(*pos)
        .iter()
        .cloned()
        .map(|pos| (pos, 1))
        .collect::<Vec<((usize, usize), usize)>>()
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &Input) -> Output {
    let (bytes, end, fallen) = input;
    let start = (0, 0);
    let ram = create_ram(bytes, *end, *fallen);
    let path = dijkstra(&start, |pos| neighbours(pos, &ram), |pos| pos == end)
        .unwrap()
        .0;
    path.len() - 1
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &Input) -> Output2 {
    let (bytes, end, fallen) = input;
    let start = (0, 0);
    let mut ram = create_ram(bytes, *end, *fallen);
    let mut idx = *fallen;
    while dijkstra(&start, |pos| neighbours(pos, &ram), |pos| pos == end).is_some() {
        ram.remove_vertex(bytes[idx]);
        idx += 1
    }
    format!("{},{}", bytes[idx - 1].0, bytes[idx - 1].1)
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
        "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
    }

    #[test]
    fn samples_part1() {
        assert_eq!(22, solve_part1(&(input_generator(sample()).0, (6, 6), 12)));
    }

    #[test]
    fn samples_part2() {
        assert_eq!(
            "6,1",
            solve_part2(&(input_generator(sample()).0, (6, 6), 12))
        );
    }
}
