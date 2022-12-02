use std::{
    fs::File,
    io::{BufRead, BufReader}
};

fn parse_literal_value(packet_body: &str) -> (u64, usize) {
    let mut group_index = 0;
    let mut current_group = &packet_body[group_index..group_index + 5];
    let mut literal_value_string = String::from("");

    while current_group.chars().nth(0).unwrap() == '1' {
        literal_value_string += &current_group[1..];
        group_index += 5;
        current_group = &packet_body[group_index..group_index + 5]
    }

    (
        u64::from_str_radix((literal_value_string + &current_group[1..]).as_str(), 2).unwrap(),
        group_index + 5,
    )
}

fn parse_package(binary_transmission: String) -> (u64, usize, u64) {
    let mut sum_of_versions = u64::from_str_radix(&binary_transmission[..3], 2).unwrap();
    let outer_packet_type = u64::from_str_radix(&binary_transmission[3..6], 2).unwrap();
    let mut values = vec![];
    let next_packet;

    if outer_packet_type == 4 {
        let (value, body_bits_read) = parse_literal_value(&binary_transmission[6..]);
        values.push(value);
        next_packet = 6 + body_bits_read;
    } else if binary_transmission.chars().nth(6).unwrap() == '0' {
        let packet_body_length = usize::from_str_radix(&binary_transmission[7..22], 2).unwrap();

        let (versions, mut body_bits_read, value) = parse_package(String::from(
            &binary_transmission[22..22 + packet_body_length],
        ));
        sum_of_versions += versions;
        values.push(value);

        while body_bits_read < packet_body_length {
            let (versions, b, value) = parse_package(String::from(
                &binary_transmission[22 + body_bits_read..22 + packet_body_length],
            ));
            sum_of_versions += versions;
            body_bits_read += b;
            values.push(value);
        }

        next_packet = 22 + packet_body_length;
    } else {
        let number_of_sub_packets = usize::from_str_radix(&binary_transmission[7..18], 2).unwrap();
        let mut body_bits_read = 0;

        for _ in 0..number_of_sub_packets {
            let (versions, b, value) =
                parse_package(String::from(&binary_transmission[18 + body_bits_read..]));
            sum_of_versions += versions;
            body_bits_read += b;
            values.push(value);
        }

        next_packet = 18 + body_bits_read;
    }

    let result;
    match outer_packet_type {
        0 => result = values.iter().fold(0, |acc, x| acc + x),
        1 => result = values.iter().fold(1, |acc, x| acc * x),
        2 => {
            result = *values
                .iter()
                .reduce(|accum, item| if accum < item { accum } else { item })
                .unwrap()
        }
        3 => {
            result = *values
                .iter()
                .reduce(|accum, item| if accum > item { accum } else { item })
                .unwrap()
        }
        5 => result = if values[0] > values[1] { 1 } else { 0 },
        6 => result = if values[0] < values[1] { 1 } else { 0 },
        7 => result = if values[0] == values[1] { 1 } else { 0 },
        _ => result = values[0],
    }

    (sum_of_versions, next_packet, result)
}

pub fn solve() {
    let file = File::open("input_files/2021/16_transmission.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut binary_transmission = String::from("");
    let transmission = lines.next().unwrap().unwrap();

    for hex_char in transmission.chars() {
        match hex_char.to_digit(16) {
            Some(hex_number) => binary_transmission += format!("{:04b}", hex_number).as_str(),
            None => println!("Invalid input character: {}", hex_char),
        }
    }

    let (pt1, _, pt2) = parse_package(binary_transmission);
    println!("Part One: {}\nPart Two: {}", pt1, pt2);
}
