use utils::read_input;

pub fn day_nine() {
    let contents = read_input("data/day_nine.txt");

    let mut total_score = 0;
    let mut nested_score = 0;
    let mut count_garbage = 0;

    let mut ignore_next = false;
    let mut is_garbage = false;

    for ch in contents.chars() {
        if ignore_next {
            ignore_next = false;
            continue;
        }

        if is_garbage {
            count_garbage += 1;
        }

        match ch {
            '!' => {
                ignore_next = true;
                if is_garbage {
                    count_garbage -= 1;
                }
            }
            '<' => is_garbage = true,
            '>' => {
                is_garbage = false;
                count_garbage -= 1;
            }
            '{' => {
                if !is_garbage {
                    nested_score += 1;
                    total_score += nested_score;
                }
            }
            '}' => {
                if !is_garbage {
                    nested_score -= 1;
                }
            }
            _ => (),
        }
    }

    println!("Day 9 part 1. Total score is {}.", total_score);
    println!(
        "Day 9 part 2. There are {} non-cancelled garbage characters.",
        count_garbage
    );
}
