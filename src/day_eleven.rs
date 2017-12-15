use std::cmp::max;
use std::cmp::min;
use utils::read_input;

struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn new(row: i32, col: i32) -> Position {
        Position { row, col }
    }
}

fn min_distance(target_pos: &Position) -> i32 {
    let mut pos = Position::new(target_pos.row.abs(), target_pos.col.abs());
    let m = min(pos.row, pos.col);
    pos.row -= m;
    pos.col -= m;

    m + pos.col + (pos.row as f32 / 2.0).floor() as i32
}

pub fn day_eleven() {
    let contents = read_input("data/day_eleven.txt");
    let directions: Vec<&str> = contents.split(',').collect();

    let mut child_position = Position::new(0, 0);
    let mut max_distance = 0;

    for direction in directions {
        match direction {
            "se" => {
                child_position.row += 1;
                child_position.col += 1;
            }
            "se\n" => {
                child_position.row += 1;
                child_position.col += 1;
            }
            "s" => child_position.row += 2,
            "sw" => {
                child_position.row += 1;
                child_position.col -= 1;
            }
            "nw" => {
                child_position.row -= 1;
                child_position.col -= 1;
            }
            "n" => child_position.row -= 2,
            "ne" => {
                child_position.row -= 1;
                child_position.col += 1;
            }
            _ => panic!("Unrecognized direction {}.", direction),
        }

        let distance = min_distance(&child_position);
        if distance > max_distance {
            max_distance = distance;
        }
    }

    let final_distance = min_distance(&child_position);
    println!("Day 11 part 1. Distance is {}.", final_distance);
    println!("Day 11 part 2. Max distance reached is {}.", max_distance);
}
