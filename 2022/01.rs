use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn main() {
    let file = File::open("input_files/01_calories.txt").unwrap();
    let reader = BufReader::new(file);

    let mut max_elf_calories: [u32; 3] = [0, 0, 0];
    let mut current_elf_calories: u32 = 0;

    for line in reader.lines() {
        let tmp = line.unwrap();

        if tmp.is_empty() {
            if current_elf_calories > max_elf_calories[0] {
                max_elf_calories[0] = current_elf_calories;
                // Use max-heap instead of array to find up to N elements without sorting
                max_elf_calories.sort();
            }

            current_elf_calories = 0;
        } else {
            current_elf_calories += tmp.parse::<u32>().unwrap();
        }
    }

    println!("Part A: {}", max_elf_calories[2]);
    println!("Part B: {}", max_elf_calories.iter().sum::<u32>());
}
