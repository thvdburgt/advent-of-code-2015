use std::collections::HashSet;

fn visit(c: char, x: &mut i32, y: &mut i32, visited_locations: &mut HashSet<(i32, i32)>) {
    match c {
        '>' => *x += 1,
        '<' => *x -= 1,
        '^' => *y += 1,
        'v' => *y -= 1,
        _ => panic!("Unknown move: {}", c),
    };
    visited_locations.insert((*x, *y));
}

pub fn solve_puzzle_part_1(input: &str) -> usize {
    let mut visited_locations = HashSet::new();
    visited_locations.insert((0, 0));
    let (mut x, mut y) = (0, 0);

    for c in input.chars() {
        visit(c, &mut x, &mut y, &mut visited_locations);
    }

    visited_locations.len()
}

pub fn solve_puzzle_part_2(input: &str) -> usize {
    let mut visited_locations: HashSet<(i32, i32)> = HashSet::new();
    visited_locations.insert((0, 0));
    let (mut santa_x, mut santa_y) = (0, 0);
    let (mut robo_x, mut robo_y) = (0, 0);

    for (i, c) in input.char_indices() {
        if i % 2 == 0 {
            visit(c, &mut santa_x, &mut santa_y, &mut visited_locations);
        } else {
            visit(c, &mut robo_x, &mut robo_y, &mut visited_locations);
        }
    }

    visited_locations.len()
}
