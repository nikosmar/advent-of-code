use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("lanternfish.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total_fish: u128 = 0;
    let mut fish_per_day = [0u128; 9];

    let initial_state = reader.lines().next().unwrap().unwrap();
    for fish in initial_state.split(',') {
        fish_per_day[fish.parse::<usize>().unwrap()] += 1;
    }

    for _day in 0..256 {
        let zeros = fish_per_day[0];
        for i in 0..8 {
            fish_per_day[i] = fish_per_day[i + 1];
        }
        fish_per_day[6] += zeros;
        fish_per_day[8] = zeros;
    }

    for fish in fish_per_day {
        total_fish += fish;
    }

    println!("{}", total_fish);
}
