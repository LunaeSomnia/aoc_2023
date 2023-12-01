use std::fmt::Display;

pub struct PartResult {
    result: String,
}

pub trait DailyChallenge: Default {
    fn get_day_number() -> usize;
    fn solve_day_one() -> PartResult;
    fn solve_day_two() -> PartResult;
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
