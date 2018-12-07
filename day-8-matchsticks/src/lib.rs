// number of characters of code for string literals
fn string_literal_size(s: &str) -> usize {
    s.len()
}

// number of characters in memory for the values of the strings
fn string_memory_size(s: &str) -> usize {
    // \\
    // \"
    // \x00
    let mut count = 0;

    let mut chars = s.chars().peekable();
    while chars.peek().is_some() {
        let c = chars.next().unwrap();

        if c == '"' {
            continue;
        } else if c == '\\' {
            let next = chars.next().unwrap();
            match next {
                '\\' => {}
                '"' => {}
                'x' => {
                    assert!(chars.next().unwrap().is_ascii_hexdigit());
                    assert!(chars.next().unwrap().is_ascii_hexdigit());
                }
                _ => panic!("invalid escape sequence"),
            }
        }

        count += 1;
    }

    count
}

fn encode_string(s: &str) -> String {
    let mut encoded = s.chars().fold('"'.to_string(), |mut encoded, c| {
        match c {
            '"' => encoded.push_str(r#"\""#),
            '\\' => encoded.push_str(r#"\\"#),
            c => encoded.push(c),
        }
        encoded
    });

    encoded.push('"');
    encoded
}

pub fn solve_puzzle_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|s| string_literal_size(s) - string_memory_size(s))
        .sum()
}

pub fn solve_puzzle_part_2(input: &str) -> usize {
    input
        .lines()
        .map(|s| encode_string(s).len() - string_literal_size(s))
        .sum()
}

#[test]
fn part_1_samples() {
    let sample1 = r#""""#;
    let sample2 = r#""abc""#;
    let sample3 = r#""aaa\"aaa""#;
    let sample4 = r#""\x27""#;

    assert_eq!(string_literal_size(sample1), 2);
    assert_eq!(string_memory_size(sample1), 0);

    assert_eq!(string_literal_size(sample2), 5);
    assert_eq!(string_memory_size(sample2), 3);

    assert_eq!(string_literal_size(sample3), 10);
    assert_eq!(string_memory_size(sample3), 7);

    assert_eq!(string_literal_size(sample4), 6);
    assert_eq!(string_memory_size(sample4), 1);
}

#[test]
fn part_2_samples() {
    let sample1 = r#""""#;
    let sample2 = r#""abc""#;
    let sample3 = r#""aaa\"aaa""#;
    let sample4 = r#""\x27""#;
    println!("1: {}", encode_string(sample1));
    println!("2: {}", encode_string(sample2));
    println!("3: {}", encode_string(sample3));
    println!("4: {}", encode_string(sample4));

    assert_eq!(encode_string(sample1).len(), 6);
    assert_eq!(encode_string(sample2).len(), 9);
    assert_eq!(encode_string(sample3).len(), 16);
    assert_eq!(encode_string(sample4).len(), 11);
}
