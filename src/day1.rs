use std::fs;

const WORD_DIGIT_PAIRS: [(&str, u8); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() {
    // Read the input file
    let input = fs::read_to_string("input.txt").expect("Error reading the file");

    let lines = input.lines().collect::<Vec<_>>();

    let total = part_two(&lines);
    println!("total -> {}", total);
}

pub fn part_two(lines: &[&str]) -> u32 {
    lines
        .iter()
        .map(|line| {
            let left_digit = (0..line.len()).find_map(|i| {
                let s = &line[i..];
                if let Some(b) = s.bytes().next().filter(u8::is_ascii_digit) {
                    return Some(b - b'0');
                }
                for &(word, digit) in &WORD_DIGIT_PAIRS {
                    if s.starts_with(word) {
                        return Some(digit);
                    }
                }
                None
            });
            let right_digit = (0..line.len()).rev().find_map(|i| {
                let s = &line[..=i];
                if let Some(b) = s.bytes().next_back().filter(u8::is_ascii_digit) {
                    return Some(b - b'0');
                }
                for &(word, digit) in &WORD_DIGIT_PAIRS {
                    if s.ends_with(word) {
                        return Some(digit);
                    }
                }
                None
            });
            (10 * left_digit.unwrap() + right_digit.unwrap()) as u32
        })
        .sum()
}
