use utils::read_input;

struct Position {
    row: usize,
    col: usize,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

fn parse_nodes() -> Vec<Vec<NodeState>> {
    let contents = read_input("data/day_twenty_two.txt");
    let mut nodes: Vec<Vec<NodeState>> = Vec::new();

    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut row: Vec<NodeState> = Vec::new();

        for ch in line.chars() {
            match ch {
                '.' => row.push(NodeState::Clean),
                '#' => row.push(NodeState::Infected),
                _ => panic!("Unrecognized character {}.", ch),
            };
        }

        nodes.push(row);
    }

    nodes
}

fn turn_left(dir: &mut Direction) {
    *dir = match *dir {
        Direction::Up => Direction::Left,
        Direction::Right => Direction::Up,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
    }
}

fn turn_right(dir: &mut Direction) {
    *dir = match *dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn reverse_dir(dir: &mut Direction) {
    *dir = match *dir {
        Direction::Up => Direction::Down,
        Direction::Right => Direction::Left,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
    }
}

// Given that the real grid extends on the infinity, it is possible that a movement brings
// us outside the current matrix. If that is going to happen, we update the nodes to have
// a new row or column in the right position.
fn move_forward(dir: &Direction, pos: Position, nodes: &mut Vec<Vec<NodeState>>) -> Position {
    let mut new_pos = Position {
        row: pos.row,
        col: pos.col,
    };

    match *dir {
        Direction::Up => {
            // If we insert a new row (or column) at the beginning, it will have index 0, so we
            // should not update the position at all.
            if pos.row == 0 {
                let mut new_row: Vec<NodeState> = Vec::new();
                for _ in 0..nodes[0].len() {
                    new_row.push(NodeState::Clean);
                }

                nodes.insert(0, new_row);
            } else {
                new_pos.row -= 1;
            }
        }
        Direction::Right => {
            if pos.col == nodes[pos.row].len() - 1 {
                for i in 0..nodes.len() {
                    nodes[i].push(NodeState::Clean);
                }
            }

            new_pos.col += 1;
        }
        Direction::Down => {
            if pos.row == nodes.len() - 1 {
                let mut new_row: Vec<NodeState> = Vec::new();
                for _ in 0..nodes[0].len() {
                    new_row.push(NodeState::Clean);
                }

                nodes.push(new_row);
            }

            new_pos.row += 1;
        }
        Direction::Left => {
            if pos.col == 0 {
                for i in 0..nodes.len() {
                    nodes[i].insert(0, NodeState::Clean);
                }
            } else {
                new_pos.col -= 1;
            }
        }
    };

    new_pos
}

pub fn day_twenty_two_part_two() {
    let mut nodes: Vec<Vec<NodeState>> = parse_nodes();
    let mut pos: Position = Position {
        row: nodes.len() / 2,
        col: nodes.len() / 2,
    };
    let mut dir = Direction::Up;
    let mut infected_count = 0;

    for _ in 0..10000000 {
        match nodes[pos.row][pos.col] {
            NodeState::Clean => {
                turn_left(&mut dir);
                nodes[pos.row][pos.col] = NodeState::Weakened;
            }
            NodeState::Weakened => {
                nodes[pos.row][pos.col] = NodeState::Infected;
                infected_count += 1;
            }
            NodeState::Infected => {
                turn_right(&mut dir);
                nodes[pos.row][pos.col] = NodeState::Flagged;
            }
            NodeState::Flagged => {
                reverse_dir(&mut dir);
                nodes[pos.row][pos.col] = NodeState::Clean;
            }
        }

        pos = move_forward(&dir, pos, &mut nodes);
    }

    println!(
        "Day 22 part 2. {} bursts caused a node to become infected.",
        infected_count
    );
}

pub fn day_twenty_two() {
    let mut nodes: Vec<Vec<NodeState>> = parse_nodes();
    let mut pos: Position = Position {
        row: nodes.len() / 2,
        col: nodes.len() / 2,
    };
    let mut dir = Direction::Up;
    let mut infected_count = 0;

    for _ in 0..10000 {
        if nodes[pos.row][pos.col] == NodeState::Clean {
            turn_left(&mut dir);
            nodes[pos.row][pos.col] = NodeState::Infected;
            infected_count += 1;
        } else {
            turn_right(&mut dir);
            nodes[pos.row][pos.col] = NodeState::Clean;
        }

        pos = move_forward(&dir, pos, &mut nodes);
    }

    println!(
        "Day 22 part 1. {} bursts caused a node to become infected.",
        infected_count
    );
}
