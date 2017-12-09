use utils::read_input;

pub fn day_five() {
    let mut lines: Vec<i32> = Vec::new();

    for l in read_input("data/day_five.txt").split('\n') {
        if l.is_empty() {
            break;
        }

        lines.push(l.parse().expect("Not a number!"));
    }

    let mut current_index: i32 = 0;
    let mut steps = 0;

    loop {
        let num: i32 = lines[current_index as usize];
        let next_index: i32 = num + current_index;
        steps += 1;

        if next_index < 0 || next_index >= lines.len() as i32 {
            break;
        }

        lines[current_index as usize] = num + 1;
        current_index = next_index;
    }

    println!("Day 5. It takes {} steps to leave the memory.", steps);
}

pub fn day_five_part_two() {
    let mut lines: Vec<i32> = Vec::new();

    for l in read_input("data/day_five.txt").split('\n') {
        if l.is_empty() {
            continue;
        }

        lines.push(l.parse().expect("Not a number!"));
    }

    let mut current_index: i32 = 0;
    let mut steps = 0;

    loop {
        let num: i32 = lines[current_index as usize];
        let next_index: i32 = num + current_index;
        steps += 1;

        if next_index < 0 || next_index >= lines.len() as i32 {
            break;
        }

        let mut offset: i32 = 1;
        if num >= 3 {
            offset = -1;
        }

        lines[current_index as usize] = num + offset;
        current_index = next_index;
    }

    println!(
        "Day 5 part 2. It takes {} steps to leave the memory.",
        steps
    );
}
