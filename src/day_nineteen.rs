use utils::read_input;

struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Position {
        Position { row, col }
    }
}

enum Direction {
    Down,
    Right,
    Up,
    Left,
}

fn get_start_position(path: &Vec<&str>) -> Position {
    let mut pos = Position::new(0, 0);
    let mut index = 0;

    for ch in path[0].chars() {
        if ch == '|' {
            pos.col = index;
        }

        index += 1;
    }

    pos
}

fn get_char(path: &Vec<&str>, pos: &Position) -> char {
    return path[pos.row].chars().nth(pos.col).unwrap();
}

fn get_right_char(path: &Vec<&str>, pos: &Position) -> char {
    if pos.col == path[0].len() - 1 {
        return ' ';
    }

    let new_pos = Position::new(pos.row, pos.col + 1);
    get_char(path, &new_pos)
}

fn get_left_char(path: &Vec<&str>, pos: &Position) -> char {
    if pos.col == 0 {
        return ' ';
    }

    let new_pos = Position::new(pos.row, pos.col - 1);
    get_char(path, &new_pos)
}

fn get_up_char(path: &Vec<&str>, pos: &Position) -> char {
    if pos.row == 0 {
        return ' ';
    }

    let new_pos = Position::new(pos.row - 1, pos.col);
    get_char(path, &new_pos)
}

fn get_down_char(path: &Vec<&str>, pos: &Position) -> char {
    if pos.row == path.len() - 1 {
        return ' ';
    }

    let new_pos = Position::new(pos.row + 1, pos.col);
    get_char(path, &new_pos)
}

// This assumes we already know that we can move in that direction without
// going out of bound of the path matrix.
fn update_position(direction: &Direction, pos: &Position) -> Position {
    let mut new_pos = Position::new(pos.row, pos.col);

    match *direction {
        Direction::Down => new_pos.row += 1,
        Direction::Up => new_pos.row -= 1,
        Direction::Right => new_pos.col += 1,
        Direction::Left => new_pos.col -= 1,
    };

    new_pos
}

pub fn day_nineteen() {
    let contents = read_input("data/day_nineteen.txt");
    let path: Vec<&str> = contents.split('\n').collect();

    let mut position = get_start_position(&path);
    let mut direction = Direction::Down;
    let mut letters = String::new();
    let mut steps = 0;

    loop {
        let ch = get_char(&path, &position);

        // We completed the path.
        if ch.is_whitespace() {
            break;
        }

        steps += 1; // Count a new step for the second part.
        if ch.is_uppercase() {
            letters.push(get_char(&path, &position));
        } else if ch == '+' {
            match direction {
                Direction::Down | Direction::Up => {
                    if get_right_char(&path, &position) == '-' {
                        direction = Direction::Right;
                    } else if get_left_char(&path, &position) == '-' {
                        direction = Direction::Left;
                    }
                }
                Direction::Left | Direction::Right => {
                    if get_up_char(&path, &position) == '|' {
                        direction = Direction::Up;
                    } else if get_down_char(&path, &position) == '|' {
                        direction = Direction::Down;
                    }
                }
            }
        }

        position = update_position(&direction, &position);
    }

    println!("Day 19 part 1. Solution is {}.", letters);
    println!("Day 19 part 2. Number of steps is {}.", steps);
}
