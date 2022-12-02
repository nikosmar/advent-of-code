use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("notes.txt").unwrap();
    let reader = BufReader::new(file);

    // ENCODING: 0 - UP - UP_L - UP_R - MID - DW_L - DW_R - DW
    let seven_seg_to_num = HashMap::from([
        (0b01110111, 0),
        (0b00010010, 1),
        (0b01011101, 2),
        (0b01011011, 3),
        (0b00111010, 4),
        (0b01101011, 5),
        (0b01101111, 6),
        (0b01010010, 7),
        (0b01111111, 8),
        (0b01111011, 9),
    ]);

    let mut results = (0, 0);

    for entry_str in reader.lines() {
        let tmp = entry_str.unwrap();
        let mut entry = tmp.split('|');
        let input = entry.next().unwrap();
        let output = entry.next().unwrap();

        let mut letter_frequencies = HashMap::from([
            ('a', 0),
            ('b', 0),
            ('c', 0),
            ('d', 0),
            ('e', 0),
            ('f', 0),
            ('g', 0),
        ]);
        let mut letter_to_signal = HashMap::from([
            ('a', 0),
            ('b', 0),
            ('c', 0),
            ('d', 0),
            ('e', 0),
            ('f', 0),
            ('g', 0),
        ]);
        let mut one: &str = "";
        let mut four: &str = "";

        for pattern in input.split_whitespace() {
            match pattern.len() {
                2 => one = pattern,
                4 => four = pattern,
                _ => (),
            }
            for letter in pattern.chars() {
                *letter_frequencies.entry(letter).or_insert(0) += 1;
            }
        }
        for (letter, frequency) in letter_frequencies.iter() {
            match frequency {
                4 => *letter_to_signal.entry(*letter).or_insert(0) += 4,
                6 => *letter_to_signal.entry(*letter).or_insert(0) += 32,
                7 => {
                    let val = if four.contains(*letter) { 8 } else { 1 };
                    *letter_to_signal.entry(*letter).or_insert(0) += val
                }
                8 => {
                    let val = if !one.contains(*letter) { 64 } else { 16 };
                    *letter_to_signal.entry(*letter).or_insert(0) += val
                }
                9 => *letter_to_signal.entry(*letter).or_insert(0) += 2,
                _ => (),
            }
        }

        let mut weight = 1000;
        for pattern in output.split_whitespace() {
            // Part One
            match pattern.len() {
                2 | 3 | 4 | 7 => results.0 += 1,
                _ => (),
            }
            // Part Two
            let mut displayed_signals = 0;
            for letter in pattern.chars() {
                displayed_signals += letter_to_signal[&letter];
            }
            results.1 += seven_seg_to_num[&displayed_signals] * weight;
            weight /= 10;
        }
    }

    println!("Part One: {}\nPart Two: {}", results.0, results.1);
}
