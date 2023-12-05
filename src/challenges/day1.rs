use crate::{daily_challenge::DailyChallenge, PartResult};

#[derive(Default)]
pub struct Day1;

impl DailyChallenge for Day1 {
    fn get_day_number() -> usize {
        1
    }

    fn solve_part_one(input: &str) -> PartResult {
        let mut sum: usize = 0;

        for line in input.lines() {
            let mut number_line = String::new();
            for char in line.chars() {
                if char.is_numeric() {
                    number_line.push(char);
                }
            }
            sum += (format!(
                "{}{}",
                number_line.chars().nth(0).unwrap(),
                number_line.chars().nth(number_line.len() - 1).unwrap()
            ))
            .parse::<usize>()
            .unwrap();
        }

        PartResult::new(sum)
    }

    fn solve_part_two(input: &str) -> PartResult {
        let mut sum: usize = 0;

        for line in input.lines() {
            let mut number_line = String::new();

            let len = line.len();
            let mut i = 0;
            while i < len {
                let substr = &line[i..];

                if let Some(digit) = DIGITS.iter().find(|v| substr.starts_with(v.0)) {
                    number_line.push_str(&digit.1.to_string());
                    i += digit.0.len() - 1;
                } else {
                    let char = substr.chars().nth(0).unwrap();
                    if char.is_numeric() {
                        number_line.push(char);
                    }
                    i += 1;
                }
            }

            sum += (format!(
                "{}{}",
                number_line.chars().nth(0).unwrap(),
                number_line.chars().nth(number_line.len() - 1).unwrap()
            ))
            .parse::<usize>()
            .unwrap();
        }

        PartResult::new(sum)
    }
}

const DIGITS: &[Digit] = &[
    Digit("one", 1),
    Digit("two", 2),
    Digit("three", 3),
    Digit("four", 4),
    Digit("five", 5),
    Digit("six", 6),
    Digit("seven", 7),
    Digit("eight", 8),
    Digit("nine", 9),
];

/// A digit in string and usize form
struct Digit<'a>(&'a str, usize);
