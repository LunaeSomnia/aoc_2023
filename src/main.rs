pub mod daily_challenge;
pub use daily_challenge::*;

pub mod challenges {
    pub mod day1;
    pub use day1::*;
}
pub use challenges::*;

pub fn solve_day<T: DailyChallenge>() {
    println!("== Day {} ==", T::get_day_number());

    let data_part_one: &str = &T::get_input_data(Part::One, false);
    let data_part_two: &str = &T::get_input_data(Part::Two, false);

    println!("> Part 1: {}", T::solve_part_one(data_part_one));
    println!("> Part 2: {}", T::solve_part_two(data_part_two));
}

pub fn test_day<T: DailyChallenge>() {
    let data_part_one: &str = &T::get_input_data(Part::One, true);
    let data_part_two: &str = &T::get_input_data(Part::Two, true);

    println!("> Part 1: {}", T::solve_part_one(data_part_one));
    println!("> Part 2: {}", T::solve_part_two(data_part_two));
}

fn main() {
    solve_day::<DayOne>();
}

pub mod tests {
    #[allow(unused)]
    use crate::*;

    #[test]
    fn test_day1() {
        test_day::<DayOne>();
    }
}
