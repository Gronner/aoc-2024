use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;
use pathfinding::matrix::Matrix;

type Num = usize;
type Output = Num;
type Input = Matrix<char>;
type Pos = (usize, usize);

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Input {
    Matrix::from_rows(input.lines().map(|line| line.chars().collect::<Vec<_>>())).unwrap()
}

fn perimeter(region: &HashSet<Pos>, garden: &Input) -> Num {
    region
        .iter()
        .map(|pos| {
            let neighs = garden
                .neighbours(*pos, false)
                .collect::<Vec<_>>();
            neighs.iter()
                .filter(|neigh| !region.contains(neigh))
                .count() + 4 - neighs.len()
        })
        .sum()
}

fn area(region: &HashSet<Pos>) -> Num {
    region.len()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &Input) -> Output {
    let mut regions = input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, _)| ((x, y), HashSet::from_iter(vec![(x, y)])))
        })
        .collect::<HashMap<Pos, HashSet<Pos>>>();

    for plot in input.keys() {
        input
            .neighbours(plot, false)
            .filter(|neigh| input.get(*neigh).unwrap() == input.get(plot).unwrap())
            .for_each(|neigh| {
                *regions.get_mut(&plot).unwrap() =
                    regions[&plot].union(&regions[&neigh]).cloned().collect();
                for other_plot in &regions[&plot].clone() {
                    *regions.get_mut(other_plot).unwrap() = regions[&plot].clone();
                }
            });
    }
    let mut unique_regions = Vec::new();
    // Hashing collections in Rust are not hashable themselves, unique and other things therefor do
    // not work
    for region in regions.values() {
        if !unique_regions.contains(&region) {
            unique_regions.push(region);
        }
    }

    let dim = 10;

    for r in 0..dim {
        for c in 0 .. dim {
            print!("{:x}", unique_regions.iter().position(|reg| reg.contains(&(r, c))).unwrap());
        }
        println!("");
    }

    unique_regions
        .iter()
        .inspect(|r| print!("{r:?}: "))
        .map(|region| {
            let area = area(region);
            let perimeter = perimeter(region, input);
            println!("{area} - {perimeter}");
            area * perimeter
        })
        .inspect(|r| println!("{r:?}"))
        .sum()
}

#[aoc(day12, part2)]
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
        "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
    }

    #[test]
    fn samples_part1() {
        assert_eq!(1930, solve_part1(&input_generator(sample())));
    }

    #[test]
    fn samples_part2() {
        /*let sample = "RRRRRRRRRR
RRRRRRRRRR
RRRRRRRRRR
RRRRRRRRRR
RRRRRRRRRR
RRIRRRRRRR
RRIIIRRRRR
RIIIIIRRRR
RIIIRIRRRR
RRRIRRRRRR
";*/
        assert_eq!(1206, solve_part2(&input_generator(sample())));
    }
}
