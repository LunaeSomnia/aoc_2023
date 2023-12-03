use crate::{daily_challenge::DailyChallenge, PartResult};

pub struct Day2;

impl Default for Day2 {
    fn default() -> Self {
        Self {}
    }
}

struct Game {
    id: usize,
    games: Vec<Vec<(usize, CubeColor)>>,
}

enum CubeColor {
    Red,
    Green,
    Blue,
}

fn parse_line(line: &str) -> Game {
    let line = line
        .replace("Game ", "")
        .replace(" red", "r")
        .replace(" green", "g")
        .replace(" blue", "b")
        .replace(" ", "");

    let mut split_collon = line.split(":");
    let game_number: usize = split_collon.next().unwrap().parse().unwrap();
    let games_unparsed: &str = split_collon.next().unwrap();

    let mut games = Vec::new();

    for game in games_unparsed.split(";") {
        let mut revealed = Vec::new();
        for cube in game.split(",") {
            let cube_type = match &cube[cube.len() - 1..] {
                "b" => CubeColor::Blue,
                "g" => CubeColor::Green,
                "r" => CubeColor::Red,
                _ => panic!("Cube color unprocessed"),
            };
            let number: usize = cube[0..cube.len() - 1].parse().unwrap();
            revealed.push((number, cube_type));
        }
        games.push(revealed);
    }

    Game {
        id: game_number,
        games,
    }
}

impl DailyChallenge for Day2 {
    fn get_day_number() -> usize {
        2
    }

    fn solve_part_one(input: &str) -> PartResult {
        const RED_CUBES: usize = 12;
        const GREEN_CUBES: usize = 13;
        const BLUE_CUBES: usize = 14;

        let games: Vec<Game> = input.lines().map(|v| parse_line(v)).collect();

        let mut sum = 0;

        for top_game in games {
            let mut valid = true;
            for game in top_game.games {
                if game.iter().any(|v| match v.1 {
                    CubeColor::Red => v.0 > RED_CUBES,
                    CubeColor::Green => v.0 > GREEN_CUBES,
                    CubeColor::Blue => v.0 > BLUE_CUBES,
                }) {
                    valid = false;
                    break;
                }
            }

            if valid {
                sum += top_game.id;
            }
        }

        PartResult::new(sum)
    }

    fn solve_part_two(input: &str) -> PartResult {
        let games: Vec<Game> = input.lines().map(|v| parse_line(v)).collect();

        let mut powers = Vec::new();

        for game in games {
            let mut min_red = usize::MIN;
            let mut min_green = usize::MIN;
            let mut min_blue = usize::MIN;

            for throw in game.games {
                for (n, color) in throw {
                    let var = match color {
                        CubeColor::Red => &mut min_red,
                        CubeColor::Green => &mut min_green,
                        CubeColor::Blue => &mut min_blue,
                    };

                    if n > *var {
                        *var = n;
                    }
                }
            }

            powers.push(min_red * min_green * min_blue);
        }

        PartResult::new(powers.iter().sum::<usize>())
    }
}
