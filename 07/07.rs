use std::fs::File;
use std::io::{BufRead, BufReader};

fn median(number_vector: &mut Vec<i32>) -> f32 {
    number_vector.sort_unstable();

    let length = number_vector.len() as f32;
    let left_median = number_vector[(length / 2.0).floor() as usize] as f32;
    let right_median = number_vector[(length / 2.0).ceil() as usize] as f32;

    if left_median == right_median {
        left_median
    } else {
        (left_median + right_median) / 2.0
    }
}

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

    let amount_of_crabs = crabs_positions.len() as i32;
    let median = median(&mut crabs_positions) as i32;

    // integer division done on purpose
    let mut mean = crabs_sum / amount_of_crabs;
    // by now crabs_positions is sorted, therefore we can binary search it
    match crabs_positions.binary_search(&mean) {
        Ok(..) => mean += 1,
        Err(..) => (),
    }

    let mut fuel_sum = (0, 0);
    for crab in crabs_positions {
        let dist = (mean - crab).abs();

        fuel_sum.0 += (median - crab).abs();
        fuel_sum.1 += (dist * (dist + 1)) / 2;
    }

    println!("Part One: {}\nPart Two: {}", fuel_sum.0, fuel_sum.1);
}
