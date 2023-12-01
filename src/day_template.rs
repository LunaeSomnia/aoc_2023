use crate::daily_challenge::DailyChallenge;

pub struct DayX;

impl Default for DayX {
    fn default() -> Self {
        Self {}
    }
}

impl DailyChallenge for DayX {
    fn get_day_number() -> usize {
        0
    }

    fn solve_day_one() -> Result<String, String> {
        todo!();
    }

    fn solve_day_two() -> Result<String, String> {
        todo!();
    }
}
