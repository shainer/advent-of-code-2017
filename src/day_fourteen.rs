fn hex_to_bin(hex: char) -> String {
    let result: String = String::from(match hex {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'a' => "1010",
        'b' => "1011",
        'c' => "1100",
        'd' => "1101",
        'e' => "1110",
        'f' => "1111",
        _ => panic!("Invalid hexadecimal character {}", hex),
    });

    result
}

fn run_one_cycle(
    sequence: &mut Vec<i32>,
    lengths: &Vec<usize>,
    position: &mut usize,
    skip_size: &mut usize,
) {
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

fn compute_knot_hash(input: &String) -> String {
    let mut sequence: Vec<i32> = (0..256).collect();

    let new_input = input.as_bytes().to_vec();
    let mut lengths: Vec<usize> = new_input.iter().map(|x| *x as usize).collect();
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

    dense_hash
}

fn get_cell_neighbours(
    disk_region_map: &Vec<Vec<u32>>,
    square: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();

    if square.0 > 0 {
        result.push((square.0 - 1, square.1));
    }
    if square.0 < disk_region_map.len() - 1 {
        result.push((square.0 + 1, square.1));
    }

    if square.1 > 0 {
        result.push((square.0, square.1 - 1));
    }

    if square.1 < disk_region_map[square.0].len() - 1 {
        result.push((square.0, square.1 + 1));
    }

    result
}

fn count_regions(disk_map: Vec<String>) -> u32 {
    let mut region_count: u32 = 2;
    let mut disk_region_map: Vec<Vec<u32>> = Vec::new();

    for row in disk_map {
        let mut new_row: Vec<u32> = Vec::new();

        for ch in row.chars() {
            if ch == '1' {
                new_row.push(1);
            } else {
                new_row.push(0);
            }
        }

        disk_region_map.push(new_row);
    }

    for row_index in 0..disk_region_map.len() {
        for col_index in 0..disk_region_map[row_index].len() {
            if disk_region_map[row_index][col_index] == 1 {
                let mut queue: Vec<(usize, usize)> = Vec::new();
                queue.push((row_index, col_index));

                while !queue.is_empty() {
                    let square = queue.pop().expect("Impossible");

                    disk_region_map[square.0][square.1] = region_count;
                    let neighbours = get_cell_neighbours(&disk_region_map, square);

                    for n in neighbours {
                        if disk_region_map[n.0][n.1] == 1 {
                            queue.push(n);
                        }
                    }
                }

                region_count += 1;
            }
        }
    }

    region_count - 2
}

pub fn day_fourteen() {
    let input = String::from("oundnydw-");
    let mut used_count = 0;
    let mut disk_map: Vec<String> = Vec::new();

    for row in 0..128 {
        let new_input: String = input.clone() + &row.to_string();
        let hash = compute_knot_hash(&new_input);
        let mut bin_hash = String::new();

        for ch in hash.chars() {
            bin_hash += &hex_to_bin(ch);
        }

        for ch in bin_hash.chars() {
            if ch == '1' {
                used_count += 1;
            }
        }

        disk_map.push(bin_hash);
    }

    let regions = count_regions(disk_map);

    println!("Day 14 part 1. Number of used squares is {}.", used_count);
    println!("Day 14 part 2. Number of regions is {}.", regions);
}
