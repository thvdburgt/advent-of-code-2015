const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn part_1_nice_string(s: &str) -> bool {
    let mut nr_of_vowels = 0;
    let mut double_letter = false;

    let mut prev = None;
    for c in s.chars() {
        if VOWELS.contains(&c) {
            nr_of_vowels += 1;
        }

        if let Some(p) = prev {
            if p == c {
                double_letter = true;
            }
            if (p == 'a' && c == 'b')
                || (p == 'c' && c == 'd')
                || (p == 'p' && c == 'q')
                || (p == 'x' && c == 'y')
            {
                return false;
            }
        }

        prev = Some(c);
    }

    nr_of_vowels >= 3 && double_letter
}

pub fn solve_puzzle_part_1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| part_1_nice_string(line))
        .count()
}

fn contains_repeating_pair(s: &str) -> bool {
    // check for pair of any two letters that appears at least twice
    let indices: Vec<_> = s.char_indices().map(|(i, _)| i).collect();

    for i in 0..=(indices.len() - 4) {
        let pair = &s[indices[i]..indices[i + 2]];
        let tail = &s[indices[i + 2]..];

        if tail.contains(pair) {
            return true;
        }
    }

    false
}

fn contains_repeating_char_one_apart(s: &str) -> bool {
    s.chars().zip(s.chars().skip(2)).any(|(a, b)| a == b)
}

fn part_2_nice_string(s: &str) -> bool {
    contains_repeating_pair(s) && contains_repeating_char_one_apart(s)
}

pub fn solve_puzzle_part_2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| part_2_nice_string(line))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_examples() {
        assert!(part_1_nice_string("ugknbfddgicrmopn"));
        assert!(part_1_nice_string("aaa"));
        assert!(!part_1_nice_string("jchzalrnumimnmhp"));
        assert!(!part_1_nice_string("haegwjzuvuyypxyu"));
        assert!(!part_1_nice_string("dvszwmarrgswjxmb"));
    }

    #[test]
    fn part_2_examples() {
        assert!(part_2_nice_string("qjhvhtzxzqqjkmpb"));
        assert!(part_2_nice_string("xxyxx"));
        assert!(!part_2_nice_string("uurcxstgmygtbstg"));
        assert!(!part_2_nice_string("ieodomkazucvgmuy"));
    }
}
