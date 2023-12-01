use fancy_regex::Regex;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn solve(input: &Vec<Vec<u32>>) -> u32 {
    let mut calibration_values_sum = 0;

    for line in input {
        calibration_values_sum += line.first().unwrap() * 10 + line.last().unwrap();
    }

    calibration_values_sum
}

fn main() {
    let str_to_num = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let re = Regex::new(r"(?=([0-9]|one|two|three|four|five|six|seven|eight|nine))").unwrap();

    let file = File::open("../input_files/01_trebuchet.txt").unwrap();
    let reader = BufReader::new(file);

    let mut part_a: Vec<Vec<u32>> = vec![];
    let mut part_b: Vec<Vec<u32>> = vec![];

    for line in reader.lines() {
        part_a.push(vec![]);
        part_b.push(vec![]);

        let line_str = line.unwrap();
        let string_matches = re.captures_iter(line_str.as_str());

        for string_match in string_matches {
            let regex_match = string_match.unwrap().get(1).unwrap().as_str();
            let num;

            if regex_match.len() == 1 {
                num = regex_match.parse::<u32>().unwrap();
                part_a.last_mut().unwrap().push(num);
            } else {
                num = str_to_num[regex_match];
            }

            part_b.last_mut().unwrap().push(num);
        }
    }

    println!("Part A: {}", solve(&part_a));
    println!("Part B: {}", solve(&part_b));
}
