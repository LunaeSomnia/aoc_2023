use regex::Regex;

use crate::{daily_challenge::DailyChallenge, PartResult};

#[derive(Default)]
pub struct Day6;

#[derive(Default, Clone)]
struct Document {
    time: u64,     // miliseconds
    distance: u64, // milimeters
}

pub const REMOVE_DUPLICATE_SPACES: &str = r" +";

impl DailyChallenge for Day6 {
    fn get_day_number() -> usize {
        6
    }

    fn solve_part_one(input: &str) -> PartResult {
        let input = parse_input(input);

        let mut result = 1;

        for document in input {
            let mut ways = 0;

            let race_time = document.time;
            for wait_time in 1..race_time {
                let speed = wait_time; // milimeters / milisecond
                let distance_traveled = speed * (race_time - wait_time);

                if distance_traveled > document.distance {
                    ways += 1;
                }
            }

            result *= ways;
        }

        PartResult::new(result)
    }

    fn solve_part_two(input: &str) -> PartResult {
        let input = parse_input_part2(input);

        let mut ways = 0;

        let race_time = input.time;
        for wait_time in 1..race_time {
            let speed = wait_time; // milimeters / milisecond
            let distance_traveled = speed * (race_time - wait_time);

            if distance_traveled > input.distance {
                ways += 1;
            }
        }

        PartResult::new(ways)
    }
}

fn parse_input(input: &str) -> Vec<Document> {
    let remove_duplicate_spaces_regex = Regex::new(REMOVE_DUPLICATE_SPACES).unwrap();

    let input = remove_duplicate_spaces_regex
        .replace_all(input, " ")
        .to_string();

    let input = input.replace("Time: ", "");
    let input = input.replace("Distance: ", "");

    let mut lines = input.lines();

    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .split(" ")
        .map(|v| v.parse::<u64>().unwrap())
        .collect();

    let distances: Vec<u64> = lines
        .next()
        .unwrap()
        .split(" ")
        .map(|v| v.parse::<u64>().unwrap())
        .collect();

    let mut documents = Vec::new();

    for i in 0..times.len() {
        documents.push(Document {
            time: times[i],
            distance: distances[i],
        });
    }

    documents
}

fn parse_input_part2(input: &str) -> Document {
    let remove_duplicate_spaces_regex = Regex::new(REMOVE_DUPLICATE_SPACES).unwrap();

    let input = remove_duplicate_spaces_regex
        .replace_all(input, "")
        .to_string();

    let input = input.replace("Time:", "");
    let input = input.replace("Distance:", "");

    let mut lines = input.lines();

    let time: u64 = lines.next().unwrap().trim().parse().unwrap();
    let distance: u64 = lines.next().unwrap().trim().parse().unwrap();

    Document { time, distance }
}
