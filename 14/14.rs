use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn element_difference(element_appearances: &HashMap<char, u128>) -> u128 {
    let mut lowest = u128::MAX;
    let mut highest: u128 = 0;

    for (_, appearances) in element_appearances {
        if *appearances < lowest {
            lowest = *appearances;
        }
        if *appearances > highest {
            highest = *appearances;
        }
    }

    highest - lowest
}

fn main() {
    let file = File::open("polymer_template.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let templ = lines.next().unwrap().unwrap();
    let mut template = templ.chars();

    lines.next();
    let mut rule_table: HashMap<String, char> = HashMap::new();
    let mut pairs_appearances: HashMap<String, u128> = HashMap::new();

    for line in lines {
        let x = line.unwrap();
        let mut split = x.split(" -> ");
        let pair = split.next().unwrap();
        let target = split.next().unwrap();

        rule_table.insert(pair.to_string(), target.parse().unwrap());
    }

    let mut x = template.next().unwrap();
    let mut element_appearances: HashMap<char, u128> = HashMap::from([(x, 1)]);
    loop {
        match template.next() {
            Some(y) => {
                *pairs_appearances
                    .entry(format!("{}{}", x, y).to_string())
                    .or_insert(0) += 1;
                x = y;
                *element_appearances.entry(x).or_insert(0) += 1;
            }
            None => break,
        }
    }

    let mut next_pairs_appearances: HashMap<String, u128> = pairs_appearances.clone();

    for step in 0..40 {
        for (pair, appearances) in pairs_appearances {
            if appearances > 0 && rule_table.contains_key(&pair) {
                let new_el = rule_table[&pair];
                let mut el = (*pair).chars();
                let mut x = el.next().unwrap().to_string();
                x.push(new_el);
                let mut y = el.next().unwrap().to_string();
                y.insert(0, new_el);

                *next_pairs_appearances.entry(pair).or_insert(0) -= appearances;
                *next_pairs_appearances.entry(x).or_insert(0) += appearances;
                *next_pairs_appearances.entry(y).or_insert(0) += appearances;
                *element_appearances.entry(new_el).or_insert(0) += appearances;
            }
        }

        pairs_appearances = next_pairs_appearances.clone();

        if step == 9 {
            println!("Part One: {}", element_difference(&element_appearances));
        }
    }

    println!("Part Two: {}", element_difference(&element_appearances));
}
