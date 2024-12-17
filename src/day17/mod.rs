use std::{fmt::Display, str::FromStr};

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;

type Num = isize;
type Output = String;
type Output2 = isize;
type Input = BitComp;

#[derive(Clone, Debug)]
pub struct BitComp {
    reg_a: Num,
    reg_b: Num,
    reg_c: Num,
    out_reg: Vec<Num>,
    pc: usize,
    memory: Vec<Num>,
}

impl BitComp {
    pub fn run(&mut self) -> String {
        while let Some(opcode) = self.memory.get(self.pc)  {
            match opcode {
                0 => self.adv(self.pc + 1),
                1 => self.bxl(self.pc + 1),
                2 => self.bst(self.pc + 1),
                3 => self.jnz(self.pc + 1),
                4 => self.bxc(self.pc + 1),
                5 => self.out(self.pc + 1),
                6 => self.bdv(self.pc + 1),
                7 => self.cdv(self.pc + 1),
                op => panic!("Unkown opcode: {op}")
            }
        }
        self.out_reg.iter().map(|n| n.to_string()).join(",")
    }

    pub fn set_a(&mut self, new_val: Num) {
        self.reg_a = new_val;
    }

    pub fn get_mem(&self) -> String {
        self.memory.iter().map(|n| n.to_string()).join(",")
    }

    fn combo_operand(&self, addr: usize) -> Num {
        match self.memory[addr] {
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            n if (0..=3).contains(&n) => n,
            ilgn => panic!("Unkown combo operand: {ilgn}"),
        }
    }

    fn adv(&mut self, addr: usize) {
        self.reg_a = self.reg_a / 2_isize.pow(self.combo_operand(addr).try_into().expect("Overflow in exponent"));
        self.pc += 2;
    }

    fn bxl(&mut self, addr: usize) {
        self.reg_b ^= self.memory[addr];
        self.pc += 2;
    }

    fn bst(&mut self, addr: usize) {
        self.reg_b = self.combo_operand(addr) % 8;
        self.pc += 2;
    }

    fn jnz(&mut self, addr: usize) {
        if self.reg_a != 0 {
            self.pc = self.memory[addr] as usize;
        } else {
            self.pc += 2;
        }
    }

    fn bxc(&mut self, _: usize) {
        self.reg_b ^= self.reg_c;
        self.pc += 2;
    }

    fn out(&mut self, addr: usize) {
        self.out_reg.push(self.combo_operand(addr) % 8);
        self.pc += 2;
    }

    fn bdv(&mut self, addr: usize) {
        self.reg_b = self.reg_a / 2_isize.pow(self.combo_operand(addr).try_into().expect("Overflow in exponent"));
        self.pc += 2;
    }

    fn cdv(&mut self, addr: usize) {
        self.reg_c = self.reg_a / 2_isize.pow(self.combo_operand(addr).try_into().expect("Overflow in exponent"));
        self.pc += 2;
    }
}

impl Display for BitComp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "A: {}
B: {}
C: {}
Mem: {:?}", self.reg_a, self.reg_b, self.reg_c, self.memory)
    }
}

impl FromStr for BitComp {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"Register A: (?<reg_a>\d+)\nRegister B: (?<reg_b>\d+)\nRegister C: (?<reg_c>\d+)\n\nProgram: (?<prog>(:?\d+,?)*)");
        let captured = re.captures(s).context("Regex not capturing")?;
        Ok(Self {
            reg_a: captured.name("reg_a").context("Group reg_a not captured")?.as_str().parse().context("Not a number")?,
            reg_b: captured.name("reg_b").context("Group reg_b not captured")?.as_str().parse().context("Not a number")?,
            reg_c: captured.name("reg_c").context("Group reg_c not captured")?.as_str().parse().context("Not a number")?,
            out_reg: vec![],
            pc: 0,
            memory: captured.name("prog").context("Group prog not captured")?.as_str().split(',').map(|n| n.parse::<Num>().context("Not a number")).collect::<Result<Vec<Num>>>()?,
        })

    }
}


#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Result<Input> {
    BitComp::from_str(input)
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &Input) -> Output {
    let mut computer = input.clone();
    computer.run()
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &Input) -> Output2 {
    0
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
        "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
    }

    #[test]
    fn samples_part1() {
        assert_eq!("4,6,3,5,6,3,5,2,1,0", solve_part1(&input_generator(sample()).unwrap()));
    }

    fn sample2() -> &'static str {
        "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
    }

    #[test]
    fn samples_part2() {
        assert_eq!(117440, solve_part2(&input_generator(sample2()).unwrap()));
    }
}
