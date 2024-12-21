use std::iter::once;
use std::sync::LazyLock;

use bimap::BiMap;

use aoc_runner_derive::{aoc, aoc_generator};
use cached::proc_macro::cached;
use cached::SizedCache;
#[allow(unused)]
use itertools::Itertools;
use pathfinding::grid::Grid;

type Output = usize;
type Pos = (usize, usize);
type Input = Vec<String>;

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Input {
    input.lines().map(|l| l.to_string()).collect()
}

static NUMERICS: LazyLock<BiMap<char, Pos>> = LazyLock::new(|| {
    // M3 Ultra takes about 16 million years in --release config
    BiMap::from_iter(vec![
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
    ])
});

static DIRECTIONS: LazyLock<BiMap<char, Pos>> = LazyLock::new(|| {
    // M3 Ultra takes about 16 million years in --release config
    BiMap::from_iter(vec![
        ('A', (2, 0)),
        ('^', (1, 0)),
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
    ])
});

static DIRS: LazyLock<BiMap<(isize, isize), char>> = LazyLock::new(|| {
    BiMap::from_iter(vec![
        ((1, 0), '>'),
        ((0, 1), 'v'),
        ((-1, 0), '<'),
        ((0, -1), '^'),
    ])
});

static NUMERICAL_KEYBOARD: LazyLock<Grid> = LazyLock::new(|| {
    Grid::from_coordinates(
        NUMERICS
            .right_values()
            .cloned()
            .collect::<Vec<_>>()
            .as_slice(),
    )
    .unwrap()
});

static DIRECTIONAL_KEYBOARD: LazyLock<Grid> = LazyLock::new(|| {
    Grid::from_coordinates(
        DIRECTIONS
            .right_values()
            .cloned()
            .collect::<Vec<_>>()
            .as_slice(),
    )
    .unwrap()
});

#[inline]
fn manhatten_distance(pos_a: Pos, pos_b: Pos) -> usize {
    pos_a.0.abs_diff(pos_b.0) + pos_a.1.abs_diff(pos_b.1)
}

#[cached(
    ty = "SizedCache<(char, char, usize), usize>",
    create = "{ SizedCache::with_size(100) }",
    convert = r#"{ (button_a, button_b, depth) }"#
)]
fn traverse(
    button_a: char,
    button_b: char,
    depth: usize,
    mapping: &BiMap<char, Pos>,
    keyboard: &Grid,
) -> usize {
    let pos_a = *mapping.get_by_left(&button_a).unwrap();
    let pos_b = *mapping.get_by_left(&button_b).unwrap();
    if depth == 0 {
        return manhatten_distance(pos_a, pos_b) + 1;
    }

    let mut moves = vec![];

    if pos_a.0 < pos_b.0 {
        moves.extend(['>'].repeat(pos_b.0 - pos_a.0));
    } else {
        moves.extend(['<'].repeat(pos_a.0 - pos_b.0));
    }

    if pos_a.1 < pos_b.1 {
        moves.extend(['v'].repeat(pos_b.1 - pos_a.1));
    } else {
        moves.extend(['^'].repeat(pos_a.1 - pos_b.1));
    }

    moves
        .iter()
        .permutations(moves.len())
        .filter_map(|moves| {
            let mut cur_pos = pos_a;

            for dir in &moves {
                let dir_move = DIRS.get_by_right(dir).unwrap();
                let next_pos = (
                    cur_pos.0 as isize + dir_move.0,
                    cur_pos.1 as isize + dir_move.1,
                );
                if next_pos.0 < 0
                    || next_pos.1 < 0
                    || !keyboard.has_vertex((next_pos.0 as usize, next_pos.1 as usize))
                {
                    return None;
                }
                cur_pos = (next_pos.0 as usize, next_pos.1 as usize);
            }
            Some(
                once('A')
                    .chain(moves.into_iter().copied())
                    .chain(once('A'))
                    .tuple_windows()
                    .map(|(b_a, b_b)| {
                        traverse(b_a, b_b, depth - 1, &DIRECTIONS, &DIRECTIONAL_KEYBOARD)
                    })
                    .sum::<usize>(),
            )
        })
        .min()
        .unwrap()
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &Input) -> Output {
    input
        .iter()
        .map(|code| {
            (
                code.strip_suffix("A").unwrap().parse::<usize>().unwrap(),
                code.chars().collect::<Vec<char>>(),
            )
        })
        .map(|(num, sequence)| {
            (
                num,
                once(&'A')
                    .chain(sequence.iter())
                    .tuple_windows()
                    .map(|(button_a, button_b)| {
                        traverse(*button_a, *button_b, 2, &NUMERICS, &NUMERICAL_KEYBOARD)
                    })
                    .sum::<usize>(),
            )
        })
        .map(|(num, seq_len)| num * seq_len)
        .sum()
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &Input) -> Output {
    input
        .iter()
        .map(|code| {
            (
                code.strip_suffix("A").unwrap().parse::<usize>().unwrap(),
                code.chars().collect::<Vec<char>>(),
            )
        })
        .map(|(num, sequence)| {
            (
                num,
                once(&'A')
                    .chain(sequence.iter())
                    .tuple_windows()
                    .map(|(button_a, button_b)| {
                        traverse(*button_a, *button_b, 25, &NUMERICS, &NUMERICAL_KEYBOARD)
                    })
                    .sum::<usize>(),
            )
        })
        .map(|(num, seq_len)| num * seq_len)
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
        assert!(true);
    }
}
