use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct CPU {
    commands: Vec<Vec<String>>,
    pc: usize,
    cycle: isize,
    accumulator: isize,
    fetch_on: isize,
    current_command: Vec<String>,
}

impl CPU {
    fn execute(&mut self) {
        let mut part_a = 0;
        let mut part_b = [[' '; 40]; 6];

        while self.pc < self.commands.len() {
            self.cycle += 1;

            if self.fetch_on == self.cycle {
                if &self.current_command[0] == "addx" {
                    self.accumulator += self.current_command[1].parse::<isize>().unwrap();
                }

                self.current_command = self.commands[self.pc].clone();
                self.fetch_on += if &self.current_command[0] == "noop" { 1 } else { 2 };
                self.pc += 1;
            }

            let pixel_row = (self.cycle - 1) / 40;
            let pixel_col = (self.cycle - 1) % 40;
            if pixel_col.abs_diff(self.accumulator) < 2 {
                part_b[(pixel_row) as usize][(pixel_col) as usize] = '#';
            }

            if (self.cycle + 20) % 40 == 0 {
                part_a += self.cycle * self.accumulator;
            }
        }

        println!("Part A: {}\nPart B:", part_a);
        for row in part_b {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

fn main() {
    let file = File::open("../input_files/10_crt.txt").unwrap();
    let reader = BufReader::new(file);

    let mut input = vec![];
    for line in reader.lines() {
        let tmp_line = line.unwrap();
        let command: Vec<String> = tmp_line.split(' ').map(|x| x.to_string()).collect();
        input.push(command);
    }

    let mut cpu = CPU {
        commands: input,
        pc: 0,
        cycle: 0,
        accumulator: 1,
        fetch_on: 1,
        current_command: vec!["".to_string()],
    };
    cpu.execute();
}
