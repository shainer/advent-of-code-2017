use utils::read_input;

#[derive(Default, PartialEq, Clone)]
struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn from_string(s: &str) -> Grid {
        let mut g: Vec<Vec<char>> = Vec::new();

        for row in s.split('/') {
            let mut v: Vec<char> = Vec::new();

            for ch in row.chars() {
                // This is needed to avoid whitespaces and other similar characters.
                if ch == '.' || ch == '#' {
                    v.push(ch);
                }
            }

            g.push(v);
        }

        Grid { grid: g }
    }

    fn new(g: Vec<Vec<char>>) -> Grid {
        Grid { grid: g }
    }

    // Divides the current grid in sizexsize squares, and returns them in a vector.
    fn iter_squares(&self, size: usize) -> Vec<Grid> {
        let mut squares: Vec<Grid> = Vec::new();
        let mut start_row: usize = 0;
        let mut start_col: usize = 0;

        while start_row <= self.grid.len() - size {
            while start_col <= self.grid.len() - size {
                let mut square: Vec<Vec<char>> = Vec::new();

                for row_index in start_row..start_row + size {
                    let mut row: Vec<char> = Vec::with_capacity(size);

                    for col_index in start_col..start_col + size {
                        row.push(self.grid[row_index][col_index])
                    }

                    square.push(row);
                }

                start_col += size;
                squares.push(Grid::new(square));
            }

            start_row += size;
            start_col = 0;
        }

        squares
    }

    // Appends a square to the row identified by row_number. This method assumes all squares
    // added to a grid have the same size.
    fn add_square(&mut self, square: &Grid, row_number: usize) {
        let start_row = row_number * square.grid.len();

        for row_i in start_row..start_row + square.grid.len() {
            if self.grid.len() <= row_i {
                self.grid.push(Vec::new());
            }

            for col_i in 0..square.grid.len() {
                self.grid[row_i].push(square.grid[row_i % square.grid.len()][col_i]);
            }
        }
    }

    fn count_on_pixels(&self) -> u32 {
        let mut count: u32 = 0;

        for row in &self.grid {
            for ch in row {
                if *ch == '#' {
                    count += 1;
                }
            }
        }

        count
    }
}

struct Rule {
    input: Grid,
    output: Grid,
}

impl Rule {
    fn new(input: Grid, output: Grid) -> Rule {
        Rule { input, output }
    }

    fn is_match(&self, grid: &Grid) -> bool {
        return self.input == *grid;
    }
}

fn parse_rules() -> Vec<Rule> {
    let mut rules: Vec<Rule> = Vec::new();
    let input = read_input("data/day_twenty_one.txt");

    for lines in input.split('\n') {
        if lines.is_empty() {
            continue;
        }

        let mut index = 0;
        let mut input_grid: Grid = Default::default();
        let mut output_grid: Grid = Default::default();

        for side in lines.split("=>") {
            side.trim();
            if index == 0 {
                input_grid = Grid::from_string(side);
            } else {
                output_grid = Grid::from_string(side);
            }

            index += 1;
        }

        rules.push(Rule::new(input_grid, output_grid));
    }

    rules
}

fn flip_grid_vertical(grid: &Grid) -> Grid {
    let mut new: Vec<Vec<char>> = Vec::with_capacity(grid.grid.len());

    for i in 0..grid.grid.len() {
        let mut row: Vec<char> = Vec::with_capacity(grid.grid.len());

        for j in 0..grid.grid.len() {
            row.push(grid.grid[i][grid.grid.len() - j - 1]);
        }

        new.push(row);
    }

    Grid::new(new)
}

fn flip_grid_horizontal(grid: &Grid) -> Grid {
    let mut new: Vec<Vec<char>> = Vec::with_capacity(grid.grid.len());

    for i in 0..grid.grid.len() {
        let mut row: Vec<char> = Vec::with_capacity(grid.grid.len());

        for j in 0..grid.grid.len() {
            row.push(grid.grid[grid.grid.len() - i - 1][j]);
        }

        new.push(row);
    }

    Grid::new(new)
}

// Rotates a grid by 90 degrees to the right.
fn rotate_grid(grid: &Grid) -> Grid {
    let mut new: Vec<Vec<char>> = Vec::with_capacity(grid.grid.len());

    for i in 0..grid.grid.len() {
        let mut row: Vec<char> = Vec::with_capacity(grid.grid.len());

        for j in 0..grid.grid.len() {
            row.push(grid.grid[grid.grid.len() - j - 1][i]);
        }

        new.push(row);
    }

    Grid::new(new)
}

fn find_rule(rules: &Vec<Rule>, square: &Grid) -> Option<Grid> {
    let flip_v = flip_grid_vertical(square);
    let flip_h = flip_grid_horizontal(square);
    let rotate_90 = rotate_grid(square);
    let rotate_180 = rotate_grid(&rotate_90);
    let rotate_270 = rotate_grid(&rotate_180);

    let rotate_flip_v90 = rotate_grid(&flip_v);
    let rotate_flip_v180 = rotate_grid(&rotate_flip_v90);
    let rotate_flip_v270 = rotate_grid(&rotate_flip_v180);

    // You could also include rotations after a horizontal flip, but it seems to generate
    // duplicates with regular no-flip rotations, and they are not required on my
    // input.
    for rule in rules {
        if rule.is_match(square) || rule.is_match(&flip_v) || rule.is_match(&flip_h) ||
            rule.is_match(&rotate_90) || rule.is_match(&rotate_180) ||
            rule.is_match(&rotate_270) ||
            rule.is_match(&rotate_flip_v90) || rule.is_match(&rotate_flip_v180) ||
            rule.is_match(&rotate_flip_v270)
        {
            return Some(rule.output.clone());
        }
    }

    None
}

pub fn day_twenty_one() {
    let rules: Vec<Rule> = parse_rules();
    let start_pattern = ".#./..#/###";
    let mut main_grid = Grid::from_string(&start_pattern);

    for iteration in 0..18 {
        let mut new_grid: Grid = Default::default();

        if main_grid.grid.len() % 2 == 0 {
            let mut square_index = 0;
            let mut row_number = 0;

            for square in main_grid.iter_squares(2) {
                // Finds the output square for this one.
                let new_square = find_rule(&rules, &square).expect("Could not find matching rule.");
                new_grid.add_square(&new_square, row_number);
                square_index += 1;

                if square_index % (main_grid.grid.len() / square.grid.len()) == 0 {
                    row_number += 1;
                }
            }
        } else if main_grid.grid.len() % 3 == 0 {
            let mut square_index = 0;
            let mut row_number = 0;

            for square in main_grid.iter_squares(3) {
                let new_square = find_rule(&rules, &square).expect("Could not find matching rule.");
                new_grid.add_square(&new_square, row_number);
                square_index += 1;

                if square_index % (main_grid.grid.len() / square.grid.len()) == 0 {
                    row_number += 1;
                }
            }
        } else {
            panic!(
                "Grid length has unexpected value of {}.",
                main_grid.grid.len()
            );
        }

        main_grid = new_grid;

        if iteration == 4 {
            println!(
                "Day 21 part 1. Number of \"on\" pixels is {}.",
                main_grid.count_on_pixels()
            );
        }
    }

    println!(
        "Day 21 part 2. Number of \"on\" pixels is {}.",
        main_grid.count_on_pixels()
    );
}
