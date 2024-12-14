use std::{cmp::Ordering, str::FromStr};

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;

use crate::utils::point::Point;

type Output = i64;
type Input = (Vec<Robot>, Point);

const X_LIM: isize = 101;
const Y_LIM: isize = 103;

#[derive(Clone, Copy, Debug)]
pub struct Robot {
    position: Point,
    speed: Point,
}

impl FromStr for Robot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let re = regex!(r"p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)");
        let captured = re.captures(s).context("Regex not capturing")?;
        Ok(Self {
            position: Point::from((
                captured
                    .name("px")
                    .context("Group not captured")?
                    .as_str()
                    .parse()
                    .context("Not a number")?,
                captured
                    .name("py")
                    .context("Group not captured")?
                    .as_str()
                    .parse()
                    .context("Not a number")?,
            )),
            speed: Point::from((
                captured
                    .name("vx")
                    .context("Group not captured")?
                    .as_str()
                    .parse()
                    .context("Not a number")?,
                captured
                    .name("vy")
                    .context("Group not captured")?
                    .as_str()
                    .parse()
                    .context("Not a number")?,
            )),
        })
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Result<Input> {
    Ok((
        input
            .lines()
            .map(Robot::from_str)
            .collect::<Result<Vec<Robot>>>()?,
        Point::from((X_LIM, Y_LIM)),
    ))
}

impl Robot {
    pub fn drive(&self, time: isize, limits: Point) -> Self {
        Self {
            position: self.position.wrapping_add(self.speed * time, limits),
            speed: self.speed,
        }
    }

    pub fn get_quadrant(&self, limits: Point) -> Option<usize> {
        let half_point_x = limits['x'] / 2;
        let half_point_y = limits['y'] / 2;
        match (
            self.position['x'].cmp(&half_point_x),
            self.position['y'].cmp(&half_point_y),
        ) {
            (Ordering::Less, Ordering::Less) => Some(0),
            (Ordering::Less, Ordering::Greater) => Some(1),
            (Ordering::Greater, Ordering::Less) => Some(2),
            (Ordering::Greater, Ordering::Greater) => Some(3),
            _ => None,
        }
    }
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &Input) -> Output {
    let (map, limits) = input;
    let new_map = map
        .iter()
        .map(|robot| robot.drive(100, *limits))
        .collect::<Vec<Robot>>();

    let mut quadrants = [0; 4];
    for robot in new_map.iter() {
        if let Some(quadrant) = robot.get_quadrant(*limits) {
            quadrants[quadrant] += 1;
        }
    }
    quadrants.iter().product()
}

fn print_robots(robots: &[Robot], limits: Point) {
    for y in 0..limits['y'] {
        for x in 0..limits['x'] {
            if robots
                .iter()
                .map(|r| r.position)
                .contains(&Point::from((x, y)))
            {
                print!("x");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &Input) -> Output {
    let (map, limits) = input;
    let mut map = map.clone();
    let robots = map.len();
    let mut seconds = 0;
    loop {
        map = map
            .iter()
            .map(|robot| robot.drive(1, *limits))
            .collect::<Vec<Robot>>();
        seconds += 1;
        if map.iter().map(|robot| robot.position).unique().count() == robots {
            print_robots(&map, *limits);
            break;
        }
    }

    seconds
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(&input_generator(input).unwrap())
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(&input_generator(input).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
    }

    #[test]
    fn samples_part1() {
        let (map, _) = input_generator(&sample()).unwrap();
        let limits = Point::from((11, 7));
        assert_eq!(12, solve_part1(&(map, limits)));
    }

    #[test]
    fn samples_part2() {
        assert!(true);
    }
}
