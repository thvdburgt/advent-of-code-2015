#[macro_use]
extern crate lazy_static;

mod instruction;

use crate::instruction::Instruction;

type GridPart1 = [[bool; 1000]; 1000];
type GridPart2 = [[u32; 1000]; 1000];

pub fn solve_puzzle_part_1(input: &str) -> usize {
    let mut grid: GridPart1 = [[false; 1000]; 1000];

    input
        .lines()
        .map(|line| line.parse().unwrap())
        .for_each(|i: Instruction| i.follow_part_1(&mut grid));

    grid.iter()
        .map(|&row| row.iter().filter(|&&is_on| is_on).count())
        .sum()
}

pub fn solve_puzzle_part_2(input: &str) -> usize {
    let mut grid: GridPart2 = [[0; 1000]; 1000];

    input
        .lines()
        .map(|line| line.parse().unwrap())
        .for_each(|i: Instruction| i.follow_part_2(&mut grid));

    grid.iter()
        .map(|&row| row.iter().sum::<u32>() as usize)
        .sum()
}
