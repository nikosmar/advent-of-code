fn inequality_roots(b: f64, c: f64) -> (f64, f64) {
    let delta = b * b + 4.0 * c;
    let sq_delta = delta.sqrt();
    let root_1 = (b - sq_delta) / 2.0;
    let root_2 = (b + sq_delta) / 2.0;

    (root_1, root_2)
}

fn ways_to_beat_record(root: f64, duration: u64) -> u64 {
    let min_integer_root = root as u64 + 1;

    2 * (duration.div_ceil(2) - min_integer_root) + 1 - (duration & 1)
}

fn main() {
    let puzzle_input = [(53, 333), (83, 1635), (72, 1289), (88, 1532)];
    let mut part_a = 1;

    for (duration, record) in puzzle_input {
        let (root_1, _) = inequality_roots(duration as f64, -record as f64);
        part_a *= ways_to_beat_record(root_1, duration);
    }

    let (part_b_root, _) = inequality_roots(53_83_72_88.0, -333_1635_1289_1532.0);

    println!("Part A: {}", part_a);
    println!("Part B: {}", ways_to_beat_record(part_b_root, 53_83_72_88));
}
