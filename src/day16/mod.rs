use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;
use pathfinding::{directed::dijkstra, grid::Grid, prelude::astar_bag};

type Output = usize;
type Cost = usize;
type Pos = (isize, isize);
type Input = (Grid, Pos, Pos);

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Input {
    let start_index = input
        .chars()
        .filter(|c| *c != '\n')
        .position(|c| c == 'S')
        .unwrap();
    let end_index = input
        .chars()
        .filter(|c| *c != '\n')
        .position(|c| c == 'E')
        .unwrap();

    let x_size = input.lines().next().unwrap().len();

    let start = (
        (start_index % x_size) as isize - 1,
        (start_index / x_size) as isize - 1,
    );

    let end = (
        (end_index % x_size) as isize - 1,
        (end_index / x_size) as isize - 1,
    );

    let grid = Grid::from_coordinates(
        &input
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '#')
                    .map(move |(x, _)| (x as isize, y as isize))
            })
            .collect::<Vec<_>>(),
    )
    .unwrap();

    (grid, start, end)
}

fn turns(curr_dir: Pos, new_dir: Pos) -> usize {
    // No turn
    if curr_dir == new_dir {
        return 0;
    }
    // Turn counterclockwise
    if (-curr_dir.1, curr_dir.0) == new_dir {
        return 1;
    }
    // Turn clockwise
    if (curr_dir.1, -curr_dir.0) == new_dir {
        return 1;
    }
    // Turn 180 deg
    2
}

fn cost(curr_dir: Pos, new_dir: Pos) -> Cost {
    match turns(curr_dir, new_dir) {
        0 => 0,
        1 => 1000,
        2 => 2000, // Should never be taken as it would mean steping back, but generalizes the
          // problem
        t => panic!("Unexpected number of turns: {t}"),
    }
}

fn neighbours(pos: Pos, dir: Pos, grid: &Grid) -> Vec<((Pos, Pos), Cost)> {
    grid
        .neighbours((pos.0 as usize, pos.1 as usize))
        .iter()
        .map(|neigh| {
            let next_dir = (neigh.0 as isize - pos.0, neigh.1 as isize - pos.1);
            let cost = cost(dir, next_dir) + 1;
            (((neigh.0 as isize, neigh.1 as isize), next_dir), cost)
        })
        .collect::<Vec<_>>()
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Input) -> Output {
    let (grid, start, end) = input;

    let path = dijkstra::dijkstra(
        &(*start, (1_isize, 0_isize)),
        |(pos, dir)| neighbours(*pos, *dir, grid),
        |(pos, _)| *pos == *end,
    )
    .unwrap();
    path.1
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Input) -> Output {
    let (grid, start, end) = input;

    astar_bag(
        &(*start, (1_isize, 0_isize)),
        |(pos, dir)| neighbours(*pos, *dir, grid),
        |_| 1, // Does not really matter, just improves performance and must be lower than actual
               // cost
        |(pos, _)| *pos == *end,
    )
    .unwrap().0
        .flatten()
        .map(|(pos, _)| (pos.0, pos.1))
        .unique()
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

    fn sample1() -> &'static str {
        "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
    }

    fn sample2() -> &'static str {
        "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
    }

    #[test]
    fn samples1_part1() {
        assert_eq!(7036, solve_part1(&input_generator(sample1())));
    }

    #[test]
    fn samples2_part1() {
        assert_eq!(11048, solve_part1(&input_generator(sample2())));
    }

    #[test]
    fn samples1_part2() {
        assert_eq!(45, solve_part2(&input_generator(sample1())));
    }

    #[test]
    fn samples2_part2() {
        assert_eq!(64, solve_part2(&input_generator(sample2())));
    }
}
