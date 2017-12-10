fn run_one_cycle(
    sequence: &mut Vec<i32>,
    lengths: &Vec<usize>,
    position: &mut usize,
    skip_size: &mut usize,
) {
    //let mut sequence = s.clone();
    //let mut position = pos;
    //let mut skip_size = ss;

    for length in lengths.iter() {
        let end_index: usize = (*position + *length - 1) % sequence.len();

        let mut i = *position;
        let mut j = end_index;

        for _ in 0..length / 2 {
            let tmp = sequence[i];
            sequence[i] = sequence[j];
            sequence[j] = tmp;

            i = (i + 1) % sequence.len();
            if j == 0 {
                j = sequence.len() - 1;
            } else {
                j -= 1;
            }
        }

        *position = (*position + *length + *skip_size) % sequence.len();
        *skip_size += 1;
    }
}

pub fn day_ten() {
    let mut sequence: Vec<i32> = (0..256).collect();
    let lengths: Vec<usize> = [
        63,
        144,
        180,
        149,
        1,
        255,
        167,
        84,
        125,
        65,
        188,
        0,
        2,
        254,
        229,
        24,
    ].to_vec();

    let mut position: usize = 0;
    let mut skip_size: usize = 0;

    run_one_cycle(&mut sequence, &lengths, &mut position, &mut skip_size);

    println!("Day 10 part 1. Result is {}.", sequence[0] * sequence[1]);
}

pub fn day_ten_part_2() {
    let mut sequence: Vec<i32> = (0..256).collect();

    let input = "63,144,180,149,1,255,167,84,125,65,188,0,2,254,229,24"
        .as_bytes()
        .to_vec();
    let mut lengths: Vec<usize> = input.iter().map(|x| *x as usize).collect();
    lengths.push(17);
    lengths.push(31);
    lengths.push(73);
    lengths.push(47);
    lengths.push(23);

    let mut position: usize = 0;
    let mut skip_size: usize = 0;

    for _ in 0..64 {
        run_one_cycle(&mut sequence, &lengths, &mut position, &mut skip_size);
    }

    let mut dense_hash: String = String::new();
    let mut block_start = 0;

    while block_start < sequence.len() {
        let mut dense_hash_item: i32 = sequence[block_start];

        for i in block_start + 1..block_start + 16 {
            dense_hash_item = dense_hash_item ^ sequence[i];
        }

        let mut tmp = format!("{:x}", dense_hash_item);
        if tmp.len() == 1 {
            tmp.insert(0, '0');
        }

        dense_hash.push_str(&tmp);
        block_start += 16;
    }

    println!("Day 10 part 2. Result is {}.", dense_hash);
}
