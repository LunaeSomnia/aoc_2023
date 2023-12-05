use std::{cmp::Ordering, rc::Rc};

use crate::{daily_challenge::DailyChallenge, PartResult};

#[derive(Default)]
pub struct Day3;

struct Schematic {
    numbers: Vec<Rc<Box<SchematicNumber>>>,
    symbols: Vec<SchematicSymbol>,
}

#[derive(Clone, Debug)]
struct SchematicSymbol {
    x: usize,
    y: usize,
    symbol: char,
    linked_numbers: Vec<Rc<Box<SchematicNumber>>>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct SchematicNumber {
    x: usize,
    y: usize,
    str: String,
    valid: bool,
}

fn parse_input(input: &str) -> Schematic {
    let mut schematic = Schematic {
        numbers: Vec::new(),
        symbols: Vec::new(),
    };

    for (y, line) in input.lines().enumerate() {
        let mut number_x = usize::MAX;
        let mut number = String::new();
        for (x, char) in line.chars().enumerate() {
            match char {
                '0'..='9' => {
                    number.push(char);
                    if number_x == usize::MAX {
                        number_x = x;
                    }
                }
                '.' => {}
                _ => schematic.symbols.push(SchematicSymbol {
                    x: x,
                    y: y,
                    symbol: char,
                    linked_numbers: Vec::new(),
                }),
            }

            if (!('0'..='9').contains(&char) || x == line.len() - 1) && !number.is_empty() {
                schematic.numbers.push(Rc::new(Box::new(SchematicNumber {
                    x: number_x,
                    y,
                    str: number.clone(),
                    valid: false,
                })));
                number_x = usize::MAX;
                number = String::new();
            }
        }
    }

    schematic
}

impl DailyChallenge for Day3 {
    fn get_day_number() -> usize {
        3
    }

    fn solve_part_one(input: &str) -> PartResult {
        let mut schematic = parse_input(input);

        let lines = input.lines().collect::<Vec<&str>>();

        for (i, number) in schematic.numbers.iter().enumerate() {
            let range_len = number.str.len() + 1;
            let range_start = (number.x as i32 - 1).clamp(0, i32::MAX) as usize;
            let range_end = (number.x as i32 - 1 + range_len as i32).clamp(0, i32::MAX) as usize;

            let line_range = ((number.y as i32 - 1).clamp(0, i32::MAX) as usize)
                ..=(number.y + 1).clamp(0, lines.len() - 1);

            let mut symbols_in_range = schematic
                .symbols
                .iter_mut()
                .filter(|v| line_range.contains(&v.y))
                .collect::<Vec<&mut SchematicSymbol>>();

            if let Some(mut symbol) = symbols_in_range
                .iter_mut()
                .find(|v| (range_start..=range_end).contains(&v.x))
            {
                symbol.linked_numbers.push(number.clone())
            }
        }

        let mut valid: Vec<SchematicNumber> = Vec::new();

        for symbol in schematic.symbols.iter_mut().filter(|v| v.symbol == '*') {
            symbol
                .linked_numbers
                .iter_mut()
                .for_each(|v| valid.push(*v.as_ref().clone()));
        }

        PartResult::new(
            valid
                .iter()
                .map(|v| v.str.parse::<usize>().unwrap())
                .sum::<usize>(),
        )
    }

    fn solve_part_two(input: &str) -> PartResult {
        let mut schematic = parse_input(input);

        let mut valid_vec = Vec::new();
        let mut invalid_vec = Vec::new();

        let lines = input.lines().collect::<Vec<&str>>();

        for (i, number) in schematic.numbers.iter().enumerate() {
            let range_len = number.str.len() + 1;
            let range_start = (number.x as i32 - 1).clamp(0, i32::MAX) as usize;
            let range_end = (number.x as i32 - 1 + range_len as i32).clamp(0, i32::MAX) as usize;

            let line_range = ((number.y as i32 - 1).clamp(0, i32::MAX) as usize)
                ..=(number.y + 1).clamp(0, lines.len() - 1);

            let mut symbols_in_range = schematic
                .symbols
                .iter_mut()
                .filter(|v| line_range.contains(&v.y))
                .collect::<Vec<&mut SchematicSymbol>>();

            if let Some(mut symbol) = symbols_in_range
                .iter_mut()
                .find(|v| (range_start..=range_end).contains(&v.x))
            {
                valid_vec.push(number.str.parse::<usize>().unwrap());

                symbol.linked_numbers.push(number.clone())
            } else {
                invalid_vec.push(number.str.parse::<usize>().unwrap());
            }
        }

        let mut sum: usize = 0;

        for symbol in schematic.symbols.iter_mut().filter(|v| v.symbol == '*') {
            symbol.linked_numbers.sort_by(|v1, v2| {
                if v1.y < v2.y {
                    Ordering::Less
                } else if v1.y == v2.y {
                    v1.x.cmp(&v2.x)
                } else {
                    Ordering::Greater
                }
            });

            if symbol.linked_numbers.len() == 2 {
                let v1 = symbol.linked_numbers[0].str.parse::<usize>().unwrap();
                let v2 = symbol.linked_numbers[1].str.parse::<usize>().unwrap();
                let gear_ratio = v1 * v2;

                sum += gear_ratio;

                symbol.linked_numbers.iter_mut().for_each(|v| {
                    Rc::make_mut(v).valid = false;
                });
            }
        }

        PartResult::new(sum)
    }
}
