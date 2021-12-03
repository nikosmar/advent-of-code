use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("report.txt").unwrap();
    let reader = BufReader::new(file);

    const BITS: usize = 12;

    let mut appearances: [u32; BITS] = [0; BITS];
    let mut line_counter = 0;
    for line in reader.lines() {
        line_counter += 1;
        let mut bin = usize::from_str_radix(&line.unwrap(), 2).unwrap();

        for i in (0..BITS).rev() {
            appearances[i] += (bin & 1) as u32;
            bin >>= 1;
        }
    }

    let mut gamma: u32 = 0;

    for i in 0..BITS {
        if 2 * appearances[BITS - 1 - i] > line_counter {
            gamma |= 1 << i;
        }
    }

    let xor_mask = 2_u32.pow(BITS as u32) - 1;
    println!("{}", gamma * (gamma ^ xor_mask));
}
