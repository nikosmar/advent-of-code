fn main() {
    let target_x = (230, 283);
    let target_y = (-107, -57);
    let max_y = -target_y.0 - 1;
    let mut viable_ways = 0;

    for x in 0..=target_x.1 {
        for y in target_y.0..=max_y {
            let mut current_x = x;
            let mut current_y = y;
            let mut position = (0, 0);

            while position.1 >= target_y.0 {
                position.0 += current_x;
                position.1 += current_y;

                if current_x > 0 {
                    current_x -= 1;
                }
                current_y -= 1;

                if target_y.1 >= position.1
                    && position.1 >= target_y.0
                    && target_x.1 >= position.0
                    && position.0 >= target_x.0
                {
                    viable_ways += 1;
                    break;
                }
            }
        }
    }

    let max_height = max_y * (max_y + 1) / 2;
    println!("Part One: {}\nPart Two: {}", max_height, viable_ways);
}
