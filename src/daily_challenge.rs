use std::{fmt::Display, fs::read_to_string};

const BASE_DATA_PATH: &str = "data/day";
const TEST_PART_ONE_PATH: &str = "/test_part1.txt";
const PART_ONE_PATH: &str = "/input_part1.txt";
const TEST_PART_TWO_PATH: &str = "/test_part2.txt";
const PART_TWO_PATH: &str = "/input_part2.txt";

pub enum Part {
    One,
    Two,
}

pub struct PartResult {
    result: String,
}

pub trait DailyChallenge: Default {
    fn get_input_data(part: Part, test: bool) -> String {
        let data_path = format!(
            "{}{}{}",
            BASE_DATA_PATH,
            Self::get_day_number(),
            match (part, test) {
                (Part::One, true) => TEST_PART_ONE_PATH,
                (Part::One, false) => PART_ONE_PATH,
                (Part::Two, true) => TEST_PART_TWO_PATH,
                (Part::Two, false) => PART_TWO_PATH,
            }
        );

        read_to_string(data_path).unwrap()
    }
    fn get_day_number() -> usize;
    fn solve_part_one(input: &str) -> PartResult;
    fn solve_part_two(input: &str) -> PartResult;
}

impl PartResult {
    pub fn new<'a>(value: impl Into<String>) -> Self {
        PartResult {
            result: value.into(),
        }
    }
}

impl Display for PartResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.result)
    }
}
