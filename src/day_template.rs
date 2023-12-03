use crate::{daily_challenge::DailyChallenge, PartResult};

pub struct DayX;

impl Default for DayX {
    fn default() -> Self {
        Self {}
    }
}

impl DailyChallenge for DayX {
    fn get_day_number() -> usize {
        2
    }

    fn solve_part_one(input: &str) -> PartResult {
        PartResult::new(0)
    }

    fn solve_part_two(input: &str) -> PartResult {
        PartResult::new(0)
    }
}
