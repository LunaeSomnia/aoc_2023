use clap::Parser;

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

    pub mod day5;
    pub use day5::*;

    pub mod day6;
    pub use day6::*;
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
    println!("== Test Day {} ==", T::get_day_number());

    let data_part_one: &str = &T::get_input_data(Part::One, true);
    let data_part_two: &str = &T::get_input_data(Part::Two, true);

    println!("> Part 1: {}", T::solve_part_one(data_part_one));
    println!("> Part 2: {}", T::solve_part_two(data_part_two));
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: u8,
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => solve_day::<Day1>(),
        2 => solve_day::<Day2>(),
        3 => solve_day::<Day3>(),
        4 => solve_day::<Day4>(),
        5 => solve_day::<Day5>(),
        6 => solve_day::<Day6>(),
        _ => (),
    }
}

pub mod tests {
    #[allow(unused)]
    use crate::*;

    test_day!(Day6);
    test_day!(Day5);
    test_day!(Day4);
    test_day!(Day3);
    test_day!(Day2);
    test_day!(Day1);
}

#[macro_export]
macro_rules! test_day {
    ($day: ident) => {
        #[test]
        fn $day() {
            test_day::<$day>();
            solve_day::<$day>();
        }
    };
}
