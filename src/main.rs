pub mod daily_challenge;
pub use daily_challenge::*;

pub mod challenges {
    pub mod day1;
    pub use day1::*;

    pub mod day2;
    pub use day2::*;

    pub mod day3;
    pub use day3::*;

    pub mod day4;
    pub use day4::*;
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
    solve_day::<Day1>();
    solve_day::<Day2>();
    solve_day::<Day3>();
    solve_day::<Day4>();
}

pub mod tests {
    #[allow(unused)]
    use crate::*;

    #[test]
    fn test_day1() {
        test_day::<Day1>();
        solve_day::<Day1>();
    }

    #[test]
    fn test_day2() {
        test_day::<Day2>();
        solve_day::<Day2>();
    }

    #[test]
    fn test_day3() {
        test_day::<Day3>();
        solve_day::<Day3>();
    }

    #[test]
    fn test_day4() {
        test_day::<Day4>();
        solve_day::<Day4>();
    }
}
