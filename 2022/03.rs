use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    iter::FromIterator,
};

fn solve_part_a(input: &Vec<String>) -> Result<(), std::io::Error> {
    let mut priorities: u32 = 0;

    for line in input {
        let length = line.len() / 2;
        let mut rucksack = HashSet::with_capacity(length);
        let mut rucksack_items = line.chars();

        for _ in 0..length {
            rucksack.insert(rucksack_items.next().unwrap());
        }

        for item in rucksack_items {
            if rucksack.contains(&item) {
                if item.is_lowercase() {
                    priorities += item as u32 - 96;
                } else {
                    priorities += item as u32 - 38;
                }
                break;
            }
        }
    }

    println!("Part A: {}", priorities);

    Ok(())
}

fn solve_part_b(input: &Vec<String>) -> Result<(), std::io::Error> {
    let mut priorities: u32 = 0;
    let mut mode = 0;
    let mut rucksack: HashSet<char> = HashSet::new();

    for line in input {
        let current_item_set: HashSet<char> = HashSet::from_iter(line.chars());

        if mode == 0 {
            rucksack = current_item_set.clone();
        } else {
            rucksack.retain(|item| current_item_set.contains(item));
        }

        if mode == 2 {
            for item in &rucksack {
                priorities += if (*item).is_lowercase() {
                    *item as u32 - 96
                } else {
                    *item as u32 - 38
                };
            }
        }

        mode = (mode + 1) % 3;
    }

    println!("Part B: {}", priorities);

    Ok(())
}

fn main() {
    let mut input = Vec::new();

    let file = File::open("input_files/03_rucksack.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        input.push(line.unwrap());
    }

    solve_part_a(&input);
    solve_part_b(&input);
}
