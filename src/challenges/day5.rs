use std::ops::Range;

use regex::Regex;

use crate::{daily_challenge::DailyChallenge, PartResult};

#[derive(Default)]
pub struct Day5;

#[derive(Clone, Debug)]
struct SingleMap {
    destination_range_start: u64,
    source_range_start: u64,
    source_range: Range<u64>,
    range_length: u64,
}

#[derive(Clone, Debug)]
struct Map {
    source_name: String,
    destination_name: String,
    maps: Vec<SingleMap>,
}

struct Input {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

fn parse_input(input: &str) -> Input {
    let input = input.replace("seeds: ", "");
    let mut lines = input.lines();

    let seeds_str = lines.next().unwrap();
    let seeds = seeds_str
        .split(" ")
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut maps = Vec::new();

    const MAP_REGEX: &str = r"(.+?)-to-(.+?) map:\n((\d+? \d+? \d+?\n)+)";
    let map_regex = Regex::new(MAP_REGEX).unwrap();
    for map_capture in map_regex.captures_iter(&input) {
        let source_name = map_capture.get(1).unwrap();
        let destination_name = map_capture.get(2).unwrap();

        let mut single_maps = Vec::new();

        for line in map_capture.get(3).unwrap().as_str().lines() {
            let mut splitted = line.split(" ");
            let destination_range_start = splitted.next().unwrap().parse::<u64>().unwrap();
            let source_range_start = splitted.next().unwrap().parse::<u64>().unwrap();
            let range_length = splitted.next().unwrap().parse::<u64>().unwrap();

            single_maps.push(SingleMap::new(
                destination_range_start,
                source_range_start,
                range_length,
            ));
        }

        maps.push(Map {
            source_name: source_name.as_str().to_string(),
            destination_name: destination_name.as_str().to_string(),
            maps: single_maps,
        });
    }

    Input { maps, seeds }
}

impl DailyChallenge for Day5 {
    fn get_day_number() -> usize {
        5
    }

    fn solve_part_one(input: &str) -> PartResult {
        let input = parse_input(input);

        let mut final_seeds = input.seeds.clone();

        for seed in final_seeds.iter_mut() {
            for map in input.maps.iter() {
                *seed = map.get_mapped_value(*seed);
            }
        }

        let lowest = final_seeds.iter().min().unwrap();
        PartResult::new(*lowest)
    }

    fn solve_part_two(input: &str) -> PartResult {
        let input = parse_input(input);

        let final_seeds = input.seeds.clone();

        let mut pairs: Vec<(u64, u64)> = Vec::new();

        for i in 0..final_seeds.len() / 2 {
            pairs.push((final_seeds[i * 2], final_seeds[i * 2 + 1]))
        }

        let mut total_seeds = Vec::new();
        for pair in pairs {
            total_seeds.append(&mut (pair.0..pair.0 + pair.1).collect::<Vec<u64>>());
        }

        for seed in total_seeds.iter_mut() {
            for map in input.maps.iter() {
                *seed = map.get_mapped_value(*seed);
            }
        }

        let lowest = total_seeds.iter().min().unwrap();
        PartResult::new(*lowest)
    }
}

impl SingleMap {
    pub fn new(destination_range_start: u64, source_range_start: u64, range_length: u64) -> Self {
        Self {
            destination_range_start,
            source_range_start,
            range_length,
            source_range: source_range_start..(source_range_start + range_length),
        }
    }
}

impl Map {
    pub fn get_mapped_value(&self, input: u64) -> u64 {
        for map in self.maps.iter() {
            if map.source_range.contains(&input) {
                let delta = input - map.source_range_start;
                return map.destination_range_start + delta;
            }
        }

        return input;
    }
}
