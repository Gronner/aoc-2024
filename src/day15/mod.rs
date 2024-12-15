use std::{collections::HashMap, fmt::Display};

use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;

use crate::utils::point::Point;

type Output = isize;
type Map = HashMap<Point, Position>;
type Commands = Vec<Direction>;
type Input = (Map, Commands);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Position {
    Empty,
    Wall,
    Cargo,
    Robot,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Position::Empty => '.',
            Position::Wall => '#',
            Position::Cargo => 'O',
            Position::Robot => '@',
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
           '^' => Self::Up,
           '>' => Self::Right,
           'v' => Self::Down,
           '<' => Self::Left,
           d => panic!("Unkown direction: {}", d),
        }
    }
}

impl From<&Direction> for Point {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Up => Point::from((0, -1)),
            Direction::Right => Point::from((1, 0)),
            Direction::Down => Point::from((0, 1)),
            Direction::Left => Point::from((-1, 0)),
        }
    }
}

impl From<char> for Position {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'O' => Self::Cargo,
            '@' => Self::Robot,
            o => panic!("Unkown space occupant: {}", o),
        }
    }
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Input {
    let (map, commands) = input.split_once("\n\n").unwrap();
    let map = map.lines()
        .enumerate()
        .flat_map(|(y, row)| row.chars().enumerate().map(move |(x, c)| (Point::from((x as isize, y as isize)), Position::from(c))))
        .collect::<HashMap<Point, Position>>();

    let commands = commands.chars().filter(|c| *c != '\n').map(Direction::from).collect::<Vec<Direction>>();

    (map, commands)
}

pub fn shift(pos: Point, dir: &Direction, map: &mut Map) -> Option<Point> {
    let new_pos = pos + Point::from(dir);
    if map[&new_pos] == Position::Wall {
        return None;
    }
    if map[&new_pos] == Position::Cargo {
        if let Some(nnp) = shift(new_pos, dir, map) {
            *map.get_mut(&nnp).unwrap() = Position::Cargo;
            *map.get_mut(&new_pos).unwrap() = Position::Empty;
        } else {
            return None;
        }
    }
    Some(new_pos)
}

pub fn compute_gps(pos: &Point) -> Output {
    pos['y'] * 100 + pos['x']
}

pub fn print_map(map: &Map) {
    let x_max = map.keys().map(|pos| pos['x']).max().unwrap();
    let y_max = map.keys().map(|pos| pos['y']).max().unwrap();

    for y in 0..=y_max {
        for x in 0..=x_max {
            print!("{}", map[&Point::from((x, y))]);
        }
        println!();
    }
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &Input) -> Output {
    let (map, commands) = input;
    let mut map = map.clone();

    let mut pos = *map.iter()
        .find(|(_, occ)| **occ == Position::Robot)
        .unwrap().0;


    for command in commands {
        let tmp = map.clone();
        if let Some(new_pos) = shift(pos, command, &mut map) {
            pos = new_pos;
            *map.get_mut(&new_pos).unwrap() = Position::Robot;
            *map.get_mut(&pos).unwrap() = Position::Empty;
        } else {
            map = tmp;
        }
    }

    map.iter()
        .filter(|(_, occ)| **occ == Position::Cargo)
        .map(|(pos, _)| compute_gps(pos))
        .sum()
}

#[aoc(day15, part2)]
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
        "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
    }

    #[test]
    fn samples_part1() {
        assert_eq!(10092, solve_part1(&input_generator(sample())));
    }

    #[test]
    fn samples_part2() {
        assert_eq!(31, solve_part2(&input_generator(sample())));
    }
}
