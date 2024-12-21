use std::{collections::HashMap, iter::once};

use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;
use pathfinding::{grid::Grid, prelude::dijkstra};

type Output = usize;
type Pos = (usize, usize);
type Input = Vec<String>;

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Input {
    input.lines().map(|l| l.to_string()).collect()
}

fn compute_dir(pos_a: Pos, pos_b: Pos) -> char {
    let dir_key: HashMap<(isize, isize), char> = HashMap::from_iter(vec![
        ((1, 0), '>'),
        ((0, 1), 'v'),
        ((-1, 0), '<'),
        ((0, -1), '^'),
    ]);
    dir_key[&(
        pos_b.0 as isize - pos_a.0 as isize,
        pos_b.1 as isize - pos_a.1 as isize,
    )]
}

fn precompute_shortest_path(
    mapping: &HashMap<char, Pos>,
    grid: &Grid,
) -> HashMap<(char, char), Vec<char>> {
    mapping
        .iter()
        .cartesian_product(mapping.iter())
        .map(|(s, e)| {
            (
                (*s.0, *e.0),
                dijkstra(
                    &(*s.1, '*'),
                    |(pos, dir)| {
                        grid.neighbours(*pos)
                            .iter()
                            .map(|neigh| {
                                let next_dir = compute_dir(*pos, *neigh);
                                ((*neigh, next_dir), if *dir == next_dir { 0 } else { 1 })
                            })
                            .collect::<Vec<_>>()
                    },
                    |(pos, _)| pos == e.1,
                )
                .unwrap()
                .0
                .windows(2)
                .map(|window| compute_dir(window[0].0, window[1].0))
                .collect::<Vec<_>>(),
            )
        })
        .collect()
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &Input) -> Output {
    let numerics: HashMap<char, Pos> = HashMap::from_iter(vec![
        ('A', (2, 3)),
        ('0', (1, 3)),
        ('3', (2, 2)),
        ('2', (1, 2)),
        ('1', (0, 2)),
        ('6', (2, 1)),
        ('5', (1, 1)),
        ('4', (0, 1)),
        ('9', (2, 0)),
        ('8', (1, 0)),
        ('7', (0, 0)),
    ]);
    let directions: HashMap<char, Pos> = HashMap::from_iter(vec![
        ('A', (2, 0)),
        ('^', (1, 0)),
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
    ]);

    let numeric_keypad =
        Grid::from_coordinates(numerics.values().cloned().collect::<Vec<_>>().as_slice()).unwrap();
    let directional_keypad =
        Grid::from_coordinates(directions.values().cloned().collect::<Vec<_>>().as_slice())
            .unwrap();

    let numeric_paths = precompute_shortest_path(&numerics, &numeric_keypad);
    let directional_paths = precompute_shortest_path(&directions, &directional_keypad);

    let paths = input.iter() // 029A Full Codes
        .inspect(|code| println!("{code}"))
        .map(|code| once('A').chain(code.chars()) // Digits starting at 'A'
            .tuple_windows()
            .fold(vec![], |mut sequence, (digit_a, digit_b)| {
                sequence.extend(numeric_paths[&(digit_a, digit_b)].iter().cloned().chain(once('A')));
                sequence
                })
            )
        .inspect(|movements| println!("Numeric Movements: {}, Sample: <A^A>^^AvvvA", movements.iter().join("")))
        .map(|movements_1| once('A').chain(movements_1) // Movements for board 1 (Radiation)
                                                                    // starting a 'A'
            .tuple_windows()
            .fold(vec![], |mut sequence, (mov_a, mov_b)| {
                sequence.extend(directional_paths[&(mov_a, mov_b)].iter().cloned().chain(once('A')));
                sequence
                })
            )
        .inspect(|movements| println!("Direction 1 Movements: {}, Sample: v<<A>>^A<A>AvA<^AA>A<vAAA>^A", movements.iter().join("")))
        .map(|movements_2| once('A').chain(movements_2) // Movements for board 2 (Cryo)
                                                                    // starting a 'A'
            .tuple_windows()
            .fold(vec![], |mut sequence, (mov_a, mov_b)| {
                sequence.extend(directional_paths[&(mov_a, mov_b)].iter().cloned().chain(once('A')));
                sequence
                })
            )
        .inspect(|movements| println!("Direction 2 Movements: {}: {}, Sample: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A", movements.iter().join(""), movements.len()))
        // Movements for board 3 (Human)
        .collect::<Vec<_>>();
    let mut sum = 0;
    for (i, code) in input.iter().enumerate() {
        sum += code[..code.len() - 1].parse::<usize>().unwrap() * paths[i].len()
    }
    sum
}

#[aoc(day21, part2)]
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
        "029A
980A
179A
456A
379A"
    }

    #[test]
    fn samples_part1() {
        assert_eq!(126384, solve_part1(&input_generator(sample())));
    }

    #[test]
    fn samples_part2() {
        assert_eq!(45, solve_part2(&input_generator(sample())));
    }
}
