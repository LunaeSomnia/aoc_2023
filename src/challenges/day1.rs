use crate::{daily_challenge::DailyChallenge, PartResult};

pub struct DayOne;

impl Default for DayOne {
    fn default() -> Self {
        Self {}
    }
}

impl DailyChallenge for DayOne {
    fn get_day_number() -> usize {
        1
    }

    fn solve_part_one(input: &str) -> PartResult {
        todo!()
    }

    fn solve_part_two(input: &str) -> PartResult {
        todo!()
    }
}
