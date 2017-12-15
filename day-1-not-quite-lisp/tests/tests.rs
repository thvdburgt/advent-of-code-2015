extern crate advent_of_code_2015_day_1;

use advent_of_code_2015_day_1::*;

#[test]
fn part_1_example() {
    assert_eq!(solve_puzzle_part_1("(())"), 0);
    assert_eq!(solve_puzzle_part_1("()()"), 0);
    assert_eq!(solve_puzzle_part_1("((("), 3);
    assert_eq!(solve_puzzle_part_1("(()(()("), 3);
    assert_eq!(solve_puzzle_part_1("))((((("), 3);
    assert_eq!(solve_puzzle_part_1("())"), -1);
    assert_eq!(solve_puzzle_part_1("))("), -1);
    assert_eq!(solve_puzzle_part_1(")))"), -3);
    assert_eq!(solve_puzzle_part_1(")())())"), -3);
}

#[test]
fn part_2_example() {
    assert_eq!(solve_puzzle_part_2(")"), 1);
    assert_eq!(solve_puzzle_part_2("()())"), 5);
}
