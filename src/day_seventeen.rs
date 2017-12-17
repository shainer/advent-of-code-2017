pub fn day_seventeen() {
    let steps: usize = 356;
    let mut current_position: usize = 0;
    let mut circular_buffer: Vec<u32> = Vec::new();
    circular_buffer.push(0);

    for i in 1..2018 {
        current_position = (current_position + steps) % circular_buffer.len();
        current_position += 1;
        circular_buffer.insert(current_position, i);
    }

    println!(
        "Day 17 part 1. The number after 2017 is {}.",
        circular_buffer[(current_position + 1) % circular_buffer.len()]
    );

    // Part 2. The number we are looking for is the last one that will be inserted at position 1,
    // since 0 remains at position 0. To speed up things, we avoid inserting into the buffer at
    // all, and we simply simulate the update of the current position by keeping track of what
    // the length of the buffer would be. This allows the solution to be found in a few seconds
    // rather than several hours.
    current_position = 0;
    let mut fake_len = 1;

    let ten: u32 = 10;
    let iterations = ten.pow(6) * 50;
    let mut solution = 0;

    for i in 1..iterations + 1 {
        current_position = (current_position + steps) % fake_len;
        current_position += 1;
        fake_len += 1;

        if current_position == 1 {
            solution = i;
        }
    }

    println!("Day 17 part 2. Solution is {}.", solution);
}
