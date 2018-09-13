extern crate advent_of_code_2015_day_3;

use advent_of_code_2015_day_3::*;

#[test]
fn part_1_example_1() {
    let input = ">";
    assert_eq!(solve_puzzle_part_1(input), 2);
}

#[test]
fn part_1_example_2() {
    let input = "^>v<";
    assert_eq!(solve_puzzle_part_1(input), 4);
}

#[test]
fn part_1_example_3() {
    let input = "^v^v^v^v^v";
    assert_eq!(solve_puzzle_part_1(input), 2);
}

#[test]
fn part_2_example_1() {
    let input = "^>";
    assert_eq!(solve_puzzle_part_2(input), 3);
}
#[test]
fn part_2_example_2() {
    let input = "^>v<";
    assert_eq!(solve_puzzle_part_2(input), 3);
}
#[test]
fn part_2_example_3() {
    let input = "^v^v^v^v^v";
    assert_eq!(solve_puzzle_part_2(input), 11);
}
