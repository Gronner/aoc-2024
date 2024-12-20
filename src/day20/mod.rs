use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;
use pathfinding::{grid::Grid, prelude::dijkstra_all};

type Output = usize;
type Pos = (usize, usize);
type Input = (Grid, Pos, usize);

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Input {
    let start_index = input
        .chars()
        .filter(|c| *c != '\n')
        .position(|c| c == 'S')
        .unwrap();

    let x_size = input.lines().next().unwrap().len();

    let start = ((start_index % x_size) - 1, (start_index / x_size) - 1);

    let min_cheat = 100;

    let grid = Grid::from_coordinates(
        &input
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '#')
                    .map(move |(x, _)| (x, y))
            })
            .collect::<Vec<_>>(),
    )
    .unwrap();

    (grid, start, min_cheat)
}

#[inline]
fn manhatten_distance(pos_a: &Pos, pos_b: &Pos) -> usize {
    pos_a.0.abs_diff(pos_b.0) + pos_a.1.abs_diff(pos_b.1)
}

fn count_cheats(grid: &Grid, start: &Pos, min_cheat: usize, max_distance: usize) -> Output {
    let mut nodes = dijkstra_all::<_, usize, _, _>(start, |pos| {
        grid.neighbours(*pos)
            .iter()
            .map(|pos| (*pos, 1))
            .collect::<Vec<_>>()
    });
    nodes.insert(*start, (*start, 0));
    // There is only one possible path, so the furthest away node must be the end node, so we
    // don't really care where it is. Also as there are no diagonals using a cheat is the walking
    // the manhatten distance.
    nodes
        .keys()
        .tuple_combinations()
        .map(|(pos_a, pos_b)| (pos_a, pos_b, manhatten_distance(pos_a, pos_b)))
        .filter(|(_, _, mhd)| *mhd <= max_distance)
        .map(|(pos_a, pos_b, mhd)| nodes[pos_b].1.abs_diff(nodes[pos_a].1) - mhd)
        .filter(|shortcut| *shortcut >= min_cheat)
        .count()
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &Input) -> Output {
    let (grid, start, min_cheat) = input;
    count_cheats(grid, start, *min_cheat, 2)
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &Input) -> Output {
    let (grid, start, min_cheat) = input;
    count_cheats(grid, start, *min_cheat, 20)
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
        "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"
    }

    #[test]
    fn samples_part1() {
        let (grid, start, _) = &input_generator(sample());
        assert_eq!(44, solve_part1(&(grid.clone(), *start, 2)));
    }

    #[test]
    fn samples_part2() {
        let (grid, start, _) = &input_generator(sample());
        assert_eq!(285, solve_part2(&(grid.clone(), *start, 50)));
    }
}
