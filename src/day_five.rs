use utils::read_input;

pub fn day_five() {
    let mut lines : Vec<String> = Vec::new();

    for l in read_input("data/day_five.txt").split('\n') {
        lines.push(l.to_owned());
    }

    let mut current_index : i32 = 0;
    let mut steps = 0;

    loop {
        if lines[current_index as usize].len() == 0 {
            break;
        }

        let num : i32 = lines[current_index as usize].parse().expect("Not a number");
        let next_index : i32 = num + current_index;
        steps += 1;

        if next_index < 0 || next_index >= lines.len() as i32 {
            break;
        }

        lines[current_index as usize] = (num + 1).to_string();
        current_index = next_index;
    }

    println!("Day 5. It takes {} steps to leave the memory.", steps);
}

pub fn day_five_part_two() {
    let mut lines : Vec<String> = Vec::new();

    for l in read_input("data/day_five.txt").split('\n') {
        lines.push(l.to_owned());
    }

    let mut current_index : i32 = 0;
    let mut steps = 0;

    loop {
        if lines[current_index as usize].len() == 0 {
            break;
        }

        let num : i32 = lines[current_index as usize].parse().expect("Not a number");
        let next_index : i32 = num + current_index;
        println!("Current index is {} and num is {}, so I jump to {}", current_index, num, next_index);
        steps += 1;

        if next_index < 0 || next_index >= lines.len() as i32 {
            break;
        }

        let mut offset : i32 = 1;
        if num >= 3 {
            offset = -1;
        }

        println!("{} becomes {}", lines[current_index as usize], (num+offset));
        lines[current_index as usize] = (num + offset).to_string();
        current_index = next_index;
    }

    println!("Day 5 part 2. It takes {} steps to leave the memory.", steps);
}
