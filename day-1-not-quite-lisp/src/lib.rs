pub fn solve_puzzle_part_1(input: &str) -> i32 {
    let floor = input.chars().fold(0, |floor, c| {
        match c {
            '(' => floor + 1,
            ')' => floor - 1,
            _   => panic!(),
        }
    });

    floor
}

pub fn solve_puzzle_part_2(input: &str) -> usize {
    input.chars().scan(0, |floor, c| {
        *floor = match c {
            '(' => *floor + 1,
            ')' => *floor - 1,
            _   => panic!(),
        };
        Some(*floor)
    })
    .enumerate()
    .find(|&(_, floor)| floor < 0).unwrap().0 + 1
}

