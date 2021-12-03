use std::fs::File;
use std::io::{BufRead, BufReader};

const BITS: usize = 12;

fn rating(
    report: &Vec<usize>,
    indices: (u32, u32), // indices.0 = starting index, indices.1 = current index
    msb_pattern: Option<usize>,
    candidates: usize,
    rating_selector: usize, // 0: majority (oxygen rating), 1: minority (co2 rating)
) -> u32 {
    let mut ones: usize = 0;
    let mut new_pattern: usize;
    let mut desired_rating= 0;

    match msb_pattern {
        None => {
            for i in report.iter() {
                ones += (*i >> indices.1) & 1;
            }

            new_pattern = 0;
        }
        Some(current_pattern) => {
            for i in report.iter() {
                if *i >> (1 + indices.1) == current_pattern {
                    ones += (*i >> indices.1) & 1;
                    desired_rating = *i;
                }
            }

            new_pattern = current_pattern << 1;
        }
    }

    let remaining_candidates: usize;
    if ones >= candidates - ones {
        remaining_candidates = if rating_selector == 0 { ones } else { candidates - ones };
        new_pattern |= 1 ^ rating_selector;
    } else {
        remaining_candidates = if rating_selector == 1 { ones } else { candidates - ones };
        new_pattern |= 0 ^ rating_selector;
    }

    match (candidates, indices.1) {
        (2, 0) => new_pattern as u32,
        (1, _) => desired_rating as u32,
        (_, 0) => rating(
            report,
            (indices.0 - 1, indices.0 - 1),
            None,
            report.len(),
            rating_selector,
        ),
        (_, _) => rating(
            report,
            (indices.0, indices.1 - 1),
            Some(new_pattern),
            remaining_candidates,
            rating_selector,
        ),
    }
}

fn main() {
    let file = File::open("report.txt").unwrap();
    let reader = BufReader::new(file);
    let mut diagnostic_report = vec![];

    for line in reader.lines() {
        let bin = usize::from_str_radix(&line.unwrap(), 2).unwrap();
        diagnostic_report.push(bin);
    }

    let starting_index: u32 = BITS as u32 - 1;
    let oxygen = rating(
        &diagnostic_report,
        (starting_index, starting_index),
        None,
        diagnostic_report.len(),
        0
    );
    let co2 = rating(
        &diagnostic_report,
        (starting_index, starting_index),
        None,
        diagnostic_report.len(),
        1
    );

    println!("{}", oxygen * co2);
}
