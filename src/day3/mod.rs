use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;

type Output = i64;
type Input = String;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Input {
    input.to_owned()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Input) -> Output {
    let re = regex!(r"mul\((\d{1,3}),(\d{1,3})\)");
    re.captures_iter(input)
        .map(|cap| cap[1].parse::<i64>().unwrap() * cap[2].parse::<i64>().unwrap())
        .sum()
}

#[derive(Debug)]
enum ToDo {
    Do(i64),
    Dont(i64),
}

impl ToDo {
    fn inner(&self) -> i64 {
        match self {
            ToDo::Do(num) => *num,
            ToDo::Dont(num) => *num,
        }
    }
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Input) -> Output {
    let re = regex!(r"(?:mul\((\d{1,3}),(\d{1,3})\))|(don't\(\))|(do\(\))");
    re.captures_iter(input)
        .fold(ToDo::Do(0), |state, cap| match (state, &cap[0][..3]) {
            (ToDo::Do(sum), "mul") => {
                ToDo::Do(cap[1].parse::<i64>().unwrap() * cap[2].parse::<i64>().unwrap() + sum)
            }
            (ToDo::Do(sum), "don") => ToDo::Dont(sum),
            (ToDo::Dont(sum), "do(") => ToDo::Do(sum),
            (state, _) => state,
        })
        .inner()
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

    #[test]
    fn samples_part1() {
        assert_eq!(
            161,
            solve_part1(&input_generator(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            ))
        );
    }

    #[test]
    fn samples_part2() {
        assert_eq!(
            48,
            solve_part2(&input_generator(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ))
        );
    }
}
