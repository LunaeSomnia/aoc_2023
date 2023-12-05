use regex::Regex;

use crate::{daily_challenge::DailyChallenge, PartResult};

#[derive(Default)]
pub struct Day4;

struct Card {
    number: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

impl DailyChallenge for Day4 {
    fn get_day_number() -> usize {
        4
    }

    fn solve_part_one(input: &str) -> PartResult {
        let mut sum = 0;

        for line in input.lines() {
            let card = parse_line(line);

            let have: Vec<&usize> = card
                .numbers
                .iter()
                .filter(|v| card.winning_numbers.contains(&v))
                .collect();

            let points = if have.len() != 0 {
                2usize.pow(have.len() as u32 - 1)
            } else {
                0
            };

            sum += points;
        }

        PartResult::new(sum)
    }

    fn solve_part_two(input: &str) -> PartResult {
        let cards = input.lines().map(|v| parse_line(v)).collect::<Vec<Card>>();
        let mut instances = vec![1usize; cards.len()];

        let mut i = 0;
        while i < instances.len() {
            let card = &cards[i];
            let have: Vec<&usize> = card
                .numbers
                .iter()
                .filter(|v| card.winning_numbers.contains(&v))
                .collect();
            let number = have.len();

            for c in (i + 1)..=(i + number).clamp(0, instances.len() - 1) {
                instances[c] += instances[i];
            }

            i += 1;
        }

        PartResult::new(instances.iter().sum::<usize>())
    }
}

const MULT_SPACES: &str = r" +";

fn parse_line(line: &str) -> Card {
    let mult_spaces_regex = Regex::new(MULT_SPACES).unwrap();
    let line = mult_spaces_regex.replace_all(line, " ");
    let line = line.replace("Card ", "");

    let mut collon_split = line.split(":");
    let card_number: usize = collon_split.next().unwrap().parse().unwrap();
    let collon_rest = collon_split.next().unwrap();

    let mut vertical_bar_split = collon_rest.split("|");
    let left_side = vertical_bar_split.next().unwrap();
    let right_side = vertical_bar_split.next().unwrap();

    let winning_numbers: Vec<usize> = left_side
        .split(" ")
        .filter(|v| !v.is_empty())
        .map(|v| v.parse().unwrap())
        .collect();
    let numbers: Vec<usize> = right_side
        .split(" ")
        .filter(|v| !v.is_empty())
        .map(|v| v.parse().unwrap())
        .collect();

    Card {
        number: card_number,
        winning_numbers,
        numbers,
    }
}
