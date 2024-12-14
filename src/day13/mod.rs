use std::str::FromStr;

use anyhow::{Context, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;

use crate::utils::point::Point;

type Num = isize;
type Output = Num;
type Input = Vec<Machine>;

#[derive(Clone, Copy, Debug)]
pub struct Machine {
    a_move: Point,
    b_move: Point,
    price: Point,
}

impl Machine {
    /// Solving
    /// $$p_x = a * a_x + b * b_x$$
    /// $$p_y = a * a_y + b * b_y$$
    ///
    /// through
    ///
    /// $$b = \frac{a_y * p_x - a_x * p_y}{a_y * b_x - a_x * b_y} $$
    /// $$a = \frac{p_x - b * b_x}{a_x} $$
    ///
    /// requiring
    /// $$ a_x * b_y \neq{} a_y * b_x $$ and $$a_x \neq{} 0 $$
    pub fn solve(&self) -> Option<(isize, isize)> {
        if self.a_move['x'] == 0 {
            return None;
        }
        if self.a_move['x'] * self.b_move['y'] == self.a_move['y'] * self.b_move['x'] {
            return None;
        }
        let b = (self.a_move['y'] * self.price['x'] - self.a_move['x'] * self.price['y'])
            / (self.a_move['y'] * self.b_move['x'] - self.a_move['x'] * self.b_move['y']);

        let a = (self.price['x'] - b * self.b_move['x']) / self.a_move['x'];

        if self.price['x'] == a * self.a_move['x'] + b * self.b_move['x']
            && self.price['y'] == a * self.a_move['y'] + b * self.b_move['y']
        {
            Some((a, b))
        } else {
            None
        }
    }

    pub fn modify_price(&self, constant: Num) -> Self {
        Self {
            a_move: self.a_move,
            b_move: self.b_move,
            price: Point::from((self.price['x'] + constant, self.price['y'] + constant)),
        }
    }
}

impl FromStr for Machine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(
            r"Button A: X\+(?<ax>\d+), Y\+(?<ay>\d+)\nButton B: X\+(?<bx>\d+), Y\+(?<by>\d+)\nPrize: X=(?<px>\d+), Y=(?<py>\d+)"
        );
        let capured = re.captures(s).context("Regex not capturing")?;
        Ok(Self {
            a_move: Point::from((
                capured
                    .name("ax")
                    .context("Group not captured")?
                    .as_str()
                    .parse()
                    .context("Not a number")?,
                capured
                    .name("ay")
                    .context("Group not captured")?
                    .as_str()
                    .parse()
                    .context("Not a number")?,
            )),
            b_move: Point::from((
                capured
                    .name("bx")
                    .context("Group not captured")?
                    .as_str()
                    .parse()
                    .context("Not a number")?,
                capured
                    .name("by")
                    .context("Group not captured")?
                    .as_str()
                    .parse()
                    .context("Not a number")?,
            )),
            price: Point::from((
                capured
                    .name("px")
                    .context("Group not captured")?
                    .as_str()
                    .parse()
                    .context("Not a number")?,
                capured
                    .name("py")
                    .context("Group not captured")?
                    .as_str()
                    .parse()
                    .context("Not a number")?,
            )),
        })
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Result<Input> {
    input
        .split("\n\n")
        .map(Machine::from_str)
        .collect::<Result<Vec<Machine>>>()
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Input) -> Output {
    input
        .iter()
        .filter_map(|machine| machine.solve())
        .map(|(a, b)| a * 3 + b)
        .sum()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Input) -> Output {
    input
        .iter()
        .map(|machine| machine.modify_price(10000000000000))
        .filter_map(|machine| machine.solve())
        .map(|(a, b)| a * 3 + b)
        .sum()
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
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
    }

    #[test]
    fn samples_part1() {
        assert_eq!(480, solve_part1(&input_generator(sample()).unwrap()));
    }

    #[test]
    fn samples_part2() {
        assert_eq!(
            875318608908,
            solve_part2(&input_generator(sample()).unwrap())
        );
    }
}
