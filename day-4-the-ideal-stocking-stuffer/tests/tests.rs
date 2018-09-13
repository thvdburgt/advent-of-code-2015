extern crate advent_of_code_2015_day_4;

use advent_of_code_2015_day_4::*;

#[test]
fn part_1_example_1() {
    assert_eq!(solve_puzzle_part_1("abcdef"), 609043);
}

#[test]
fn part_1_example_2() {
    assert_eq!(solve_puzzle_part_1("pqrstuv"), 1048970);
}
