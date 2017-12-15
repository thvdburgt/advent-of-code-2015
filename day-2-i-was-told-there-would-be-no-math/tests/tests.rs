extern crate advent_of_code_2015_day_2;

use advent_of_code_2015_day_2::*;

#[test]
fn part_1_example() {
    assert_eq!(solve_puzzle_part_1("2x3x4"), 52+6);
    assert_eq!(solve_puzzle_part_1("1x1x10"), 42+1);
}

#[test]
fn part_2_example() {
    assert_eq!(solve_puzzle_part_2("2x3x4"), 10+24);
    assert_eq!(solve_puzzle_part_2("1x1x10"), 4+10);
}
