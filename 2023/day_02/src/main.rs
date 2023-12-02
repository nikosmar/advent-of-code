use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn solve_game(game: &Vec<Vec<(u32, &str)>>) -> (bool, u32) {
    let mut part_a_possible = true;
    let (mut red, mut green, mut blue) = (0, 0, 0);

    for round in game {
        for (amount, color) in round {
            match *color {
                "red" => {
                    if *amount > MAX_RED {
                        part_a_possible = false;
                    }
                    if *amount > red {
                        red = *amount;
                    }
                }
                "green" => {
                    if *amount > MAX_GREEN {
                        part_a_possible = false;
                    }
                    if *amount > green {
                        green = *amount;
                    }
                }
                "blue" => {
                    if *amount > MAX_BLUE {
                        part_a_possible = false;
                    }
                    if *amount > blue {
                        blue = *amount;
                    }
                }
                _ => (),
            }
        }
    }

    (part_a_possible, red * green * blue)
}

fn main() {
    let file = File::open("../input_files/02_rgbcubes.txt").unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"([0-9]+) (red|green|blue)").unwrap();
    let mut ids_sum = 0;
    let mut power_sum = 0;

    for line in reader.lines() {
        let line_str = line.unwrap();
        let game_str: Vec<_> = line_str.split(':').collect();

        let game_id: u32 = game_str[0].split(' ').collect::<Vec<_>>()[1]
            .parse::<u32>()
            .unwrap();
        let game_rounds_str: Vec<_> = game_str[1].split(';').collect();

        let mut game: Vec<_> = vec![];
        for round in game_rounds_str {
            let mut rounds: Vec<(u32, &str)> = vec![];
            for (_, [f1, f2]) in re.captures_iter(round).map(|caps| caps.extract()) {
                rounds.push((f1.parse::<u32>().unwrap(), f2));
            }
            game.push(rounds);
        }

        let (part_a, part_b) = solve_game(&game);
        if part_a {
            ids_sum += game_id;
        }
        power_sum += part_b;
    }

    println!("Part A: {}", ids_sum);
    println!("Part B: {}", power_sum);
}
