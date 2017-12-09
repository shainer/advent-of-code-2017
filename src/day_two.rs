use utils::read_input;
use std;

pub fn day_two() {
    let contents = read_input("data/day_two.txt");
    let mut checksum = 0;

    for line in contents.split('\n') {
        let mut row_max: i32 = 0;
        let mut row_min: i32 = std::i32::MAX;
        let mut finished = false;

        for s in line.split('\t') {
            if s.len() == 0 {
                finished = true;
                break;
            }

            let num: i32 = s.parse().expect(
                "I encountered something that was not a number.",
            );
            if num > row_max {
                row_max = num;
            }
            if num < row_min {
                row_min = num;
            }
        }

        if !finished {
            checksum += row_max - row_min;
        }
    }

    println!("Day two. Checksum is {}", checksum);
}

pub fn day_two_part_two() {
    let contents = read_input("data/day_two.txt");
    let mut checksum = 0;

    for line in contents.split('\n') {
        let items: Vec<&str> = line.split('\t').collect();

        for i in 0..items.len() {
            let s1 = &items[i];
            if s1.len() == 0 {
                break;
            }
            let num1: i32 = s1.parse().expect(
                "I encountered something that was not a number.",
            );

            for j in i + 1..items.len() {
                let s2 = &items[j];
                let num2: i32 = s2.parse().expect(
                    "I encountered something that was not a number.",
                );

                if num2 % num1 == 0 {
                    checksum += num2 / num1;
                } else if num1 % num2 == 0 {
                    checksum += num1 / num2;
                }
            }
        }
    }

    println!("Day two part two. Checksum is {}.", checksum);
}
