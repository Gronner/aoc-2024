use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;

use crate::utils::point::Point;

type Num = usize;
type Output = Num;
type Pos = Point;
type Input = Vec<HashSet<Point>>;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Input {
    let garden: HashMap<Point, char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Point::from((x as isize, y as isize)), c))
        })
        .collect();
    let mut regions: HashMap<Point, HashSet<Point>> = garden
        .keys()
        .map(|pos| (*pos, HashSet::from_iter(vec![*pos])))
        .collect::<HashMap<_, _>>();

    let directions = [
        Point::from((1, 0)),
        Point::from((0, 1)),
        Point::from((-1, 0)),
        Point::from((0, -1)),
    ];
    for plant in garden.keys() {
        for dir in directions {
            if garden.contains_key(&(plant + &&dir)) && garden[plant] == garden[&(plant + &&dir)] {
                *regions.get_mut(plant).unwrap() = regions[plant]
                    .union(&regions[&(plant + &&dir)])
                    .cloned()
                    .collect();
                for other_plants in &regions[plant].clone() {
                    *regions.get_mut(other_plants).unwrap() = regions[plant].clone();
                }
            }
        }
    }
    regions.values().fold(Vec::new(), |mut urs, region| {
        if !urs.contains(region) {
            urs.push(region.clone());
            urs
        } else {
            urs
        }
    })
}

fn area(region: &HashSet<Pos>) -> Num {
    region.len()
}

fn perimeter(region: &HashSet<Point>) -> Num {
    let directions = [
        Point::from((1, 0)),
        Point::from((0, 1)),
        Point::from((-1, 0)),
        Point::from((0, -1)),
    ];
    region
        .iter()
        .cartesian_product(directions.iter())
        .map(|(plant, dir)| plant + &dir)
        .filter(|neighbour| !region.contains(neighbour))
        .count()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &Input) -> Output {
    input
        .iter()
        .map(|region| {
            let area = area(region);
            let perimeter = perimeter(region);
            area * perimeter
        })
        .sum()
}

/// The amount of sides is equal to the amount of corners.
/// By "walking" the perimeter and checking wether a point is a corner *per direction* sides can be
/// counted. A corner is any point `a` with direction `a -> b` for which:
///
/// ```text
/// a b
/// c d
/// ```
///
/// * `b` and `c` are not part of `a`'s region:
///
/// ```text
/// A B
/// B ?
/// ```
///
/// * `b` is not part of of `a`'s region, but `c` and `d` are:
///
/// ```text
/// A B
/// A A
/// ```
fn corners(region: &HashSet<Point>) -> Num {
    let directions = [
        Point::from((1, 0)),
        Point::from((0, 1)),
        Point::from((-1, 0)),
        Point::from((0, -1)),
    ];
    region
        .iter()
        .cartesian_product(directions.iter())
        .filter(|(pos, dir)| {
            !region.contains(&(*dir + pos))
                && (!region.contains(&(**pos + dir.turn_clockwise()))
                    || (region.contains(&(**pos + **dir + dir.turn_clockwise()))))
        })
        .count()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Input) -> Output {
    input
        .iter()
        .map(|region| {
            let area = area(region);
            let sides = corners(region);
            area * sides
        })
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
        assert_eq!(1206, solve_part2(&input_generator(sample())));
    }
}
