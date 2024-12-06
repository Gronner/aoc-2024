use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;

type Output = usize;
type Input = HashMap<Point, char>;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, c)| (Point::from((x as isize, y as isize)), c))
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq)]
enum Outcome {
    Loop,
    Escape(HashSet<(Point, Dir)>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn turn_right(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: isize,
    y: isize,
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<Point> for (isize, isize) {
    fn from(value: Point) -> Self {
        (value.x, value.y)
    }
}

impl From<(isize, isize)> for Point {
    fn from(value: (isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<Dir> for Point {
    fn from(value: Dir) -> Self {
        Self::from(match value {
            Dir::Up => (0, -1),
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
        })
    }
}

fn walk(map: &HashMap<Point, char>) -> Outcome {
    let mut visited = HashSet::new();
    let (start, _) = map.iter().find(|(_, c)| **c == '^').unwrap();
    let mut pos = *start;
    let mut dir = Dir::Up;
    visited.insert((pos, dir));

    loop {
        let new_pos = pos + dir.into();
        if !map.contains_key(&new_pos) {
            return Outcome::Escape(visited);
        }
        if *map.get(&new_pos).expect("Path contained in map not in map") == '#' {
            dir = dir.turn_right();
        } else {
            pos = new_pos;
        }
        if !visited.insert((pos, dir)) {
            return Outcome::Loop;
        }
    }
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Input) -> Output {
    match walk(input) {
        Outcome::Loop => unreachable!(),
        Outcome::Escape(steps) => steps.iter().map(|(pos, _)| pos).unique().count(),
    }
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Input) -> Output {
    let path = match walk(input) {
        Outcome::Loop => unreachable!(),
        Outcome::Escape(steps) => steps
            .iter()
            .map(|(pos, _)| *pos)
            .unique()
            .collect::<Vec<Point>>(),
    };
    path.iter()
        .filter(|pos| *input.get(pos).expect("Path not in map") == '.')
        .filter(|pos| {
            let mut new_map = input.clone();
            *new_map.get_mut(pos).expect("Path not in map") = '#';
            walk(&new_map) == Outcome::Loop
        })
        .count()
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
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"
    }

    #[test]
    fn samples_part1() {
        assert_eq!(41, solve_part1(&input_generator(sample())));
    }

    #[test]
    fn samples_part2() {
        assert_eq!(6, solve_part2(&input_generator(sample())));
    }
}
