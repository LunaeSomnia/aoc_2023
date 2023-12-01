pub mod daily_challenge;
pub use daily_challenge::*;

pub mod challenges {
    pub mod day_one;
    pub use day_one::*;
}
pub use challenges::*;

pub fn solve_day<T: DailyChallenge>(challenge: T) {
    println!("== Day {} ==", T::get_day_number());
    println!("> Part 1: {}", T::solve_day_one());
    println!("> Part 2: {}", T::solve_day_two());
}

fn main() {
    solve_day(DayOne::default());
}
