use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;

type Id = usize;
type Output = usize;
type Input = (Vec<Block>, usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Block {
    Empty,
    File(Id),
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Input {
    let mut id = 0;
    (
        input
            .chars()
            .enumerate()
            .flat_map(|(idx, c)| {
                let size = c.to_digit(10).expect("Value in file map not a digit") as usize;
                if idx % 2 == 0 {
                    let new_blocks = std::iter::repeat(Block::File(id)).take(size);
                    id += 1;
                    new_blocks
                } else {
                    std::iter::repeat(Block::Empty).take(size)
                }
            })
            .collect::<Vec<Block>>(),
        id - 1,
    )
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Input) -> Output {
    let (input, _) = input;
    let mut defragmented = input.clone();
    let mut empty = defragmented
        .iter()
        .position(|block| *block == Block::Empty)
        .expect("No empty Blocks in filesystem");
    for (pos, block) in input.iter().enumerate().rev() {
        if *block == Block::Empty {
            continue;
        }
        defragmented[empty] = *block;
        defragmented[pos] = Block::Empty;
        empty = defragmented
            .iter()
            .position(|block| *block == Block::Empty)
            .expect("No empty Blocks in filesystem");
        let last_full = defragmented
            .iter()
            .rposition(|block| *block != Block::Empty)
            .expect("No IDs in filesystem");
        if empty >= last_full {
            break;
        }
    }
    hash(&defragmented)
}

fn hash(filesystem: &[Block]) -> usize {
    filesystem
        .iter()
        .enumerate()
        .filter(|(_, block)| **block != Block::Empty)
        .map(|(pos, id)| match id {
            Block::Empty => unreachable!(),
            Block::File(id) => pos * id,
        })
        .sum()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Input) -> Output {
    let (input, highest_id) = input;
    let mut defragmented = input.clone();
    for id in (0..=*highest_id).rev() {
        let filesize = input
            .iter()
            .filter(|block| **block == Block::File(id))
            .count();
        let old_pos = input
            .iter()
            .position(|block| *block == Block::File(id))
            .expect("File disappeared from file system");
        if let Some(new_pos) = defragmented
            .windows(filesize)
            .position(|blocks| blocks == vec![Block::Empty; filesize])
        {
            if old_pos <= new_pos {
                continue;
            }
            for i in 0..filesize {
                defragmented[new_pos + i] = Block::File(id);
                defragmented[old_pos + i] = Block::Empty;
            }
        }
    }
    hash(&defragmented)
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
        "2333133121414131402"
    }

    #[test]
    fn samples_part1() {
        assert_eq!(1928, solve_part1(&input_generator(sample())));
    }

    #[test]
    fn samples_part2() {
        assert_eq!(2858, solve_part2(&input_generator(sample())));
    }
}
