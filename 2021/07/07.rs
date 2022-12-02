use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("crabs.txt").unwrap();
    let reader = BufReader::new(file);
    let first_line = reader.lines().next().unwrap().unwrap();

    let mut crabs_positions = vec![];
    let mut crabs_sum = 0;

    for crab in first_line.split(',') {
        let crab_pos = crab.parse::<i32>().unwrap();
        crabs_positions.push(crab_pos);
        crabs_sum += crab_pos;
    }

    crabs_positions.sort_unstable();
    let amount_of_crabs = crabs_positions.len() as f32;

    let median = crabs_positions[(amount_of_crabs / 2.0) as usize];
    let mean_floor = (crabs_sum as f32 / amount_of_crabs).floor() as i32;
    let mean_ceil = (crabs_sum as f32 / amount_of_crabs).ceil() as i32;

    let mut fuel_sum = (0, 0, 0);
    for crab in crabs_positions {
        fuel_sum.0 += (median - crab).abs();

        let dist_floor = (mean_floor - crab).abs();
        let dist_ceil = (mean_ceil - crab).abs();

        fuel_sum.1 += (dist_floor * (dist_floor + 1)) / 2;
        fuel_sum.2 += (dist_ceil * (dist_ceil + 1)) / 2;
    }

    if fuel_sum.2 < fuel_sum.1 {
        fuel_sum.1 = fuel_sum.2;
    }
    println!("Part One: {}\nPart Two: {}", fuel_sum.0, fuel_sum.1);
}
