pub fn day_three() {
    let input = 325489;
    let mut base = 1;
    let mut square;

    loop {
        square = base * base;
        if square > input {
            println!("The length of the side of the grid is {}.", base);
            println!(
                "The number at the bottom right corner of the grid is {}.",
                square
            );
            break;
        }

        base += 2;
    }

    println!(
        "Our number is at {} steps up from the number at the bottom corner.",
        square - input
    );
    println!("Therefore our number is on the right edge of the grid.");
    println!("To get to 1, we need first to move down or up until the midsection of the edge.");

    let half_side = (base as f32 / 2.0).ceil() as i32;
    println!(
        "The midsection is at number #{} on the grid's edge.",
        half_side
    );
    let move_down = (square - input) - half_side;
    println!("We need {} movements to get there.", move_down);

    println!("Now to get to 1, we need to move left for half of the square grid's edge.");
    println!("So the total number of steps is {}.", move_down + half_side);
}

#[derive(Copy, Clone)]
enum Direction {
    RIGHT,
    UP,
    LEFT,
    DOWN,
}

struct Grid {
    grid: Vec<Vec<i32>>,
    direction: Direction,
    x: usize,
    y: usize,
}

impl Grid {
    // Constructor takes the size of the grid, the initial direction of movement, and the
    // coordinates of the cell we start from.
    fn new(edge: usize, direction: Direction, x: usize, y: usize) -> Grid {
        let mut new_grid = Grid {
            grid: Vec::with_capacity(edge),
            direction,
            x,
            y,
        };

        // Make the grid an edgexedge "matrix" filled with zeroes.
        for _ in 0..edge {
            let mut row = Vec::with_capacity(edge);

            for _ in 0..edge {
                row.push(0);
            }

            new_grid.grid.push(row);
        }

        new_grid
    }

    // Returns the coordinates of the next cell provided we move in the current direction.
    fn get_coords_to_move(&self) -> (usize, usize) {
        let mut new_x = self.x;
        let mut new_y = self.y;

        match self.direction {
            Direction::RIGHT => new_y += 1,
            Direction::UP => new_x -= 1,
            Direction::LEFT => new_y -= 1,
            Direction::DOWN => new_x += 1,
        }

        (new_x, new_y)
    }

    // Actually move forward in the current direction.
    fn move_forward(&mut self) {
        let (new_x, new_y) = self.get_coords_to_move();
        self.x = new_x;
        self.y = new_y;
    }

    // Returns true if the current position is outside the bounds of the grid.
    fn is_outside_bounds(&self) -> bool {
        self.x >= self.grid.len() || self.y >= self.grid.len()
    }

    // Compute the sum of all the cells around the current one. This is okay for the application
    // because all the cells that have not been filled yet are set to 0 anyway.
    fn sum_all_around(&self) -> i32 {
        let mut sum = 0;

        if self.x > 0 {
            sum += self.grid[self.x - 1][self.y];

            if self.y > 0 {
                sum += self.grid[self.x - 1][self.y - 1];
                sum += self.grid[self.x][self.y - 1];
            }

            if self.y < self.grid.len() - 1 {
                sum += self.grid[self.x - 1][self.y + 1];
                sum += self.grid[self.x][self.y + 1];
            }
        }

        if self.x < self.grid.len() - 1 {
            sum += self.grid[self.x + 1][self.y];

            if self.y > 0 {
                sum += self.grid[self.x + 1][self.y - 1];
            }

            if self.y < self.grid.len() - 1 {
                sum += self.grid[self.x + 1][self.y + 1];
            }
        }

        return sum;
    }

    fn set_cell(&mut self, value: i32) {
        self.grid[self.x][self.y] = value;
    }

    // Decides whether we should change direction for the next movement.
    fn maybe_change_direction(&mut self) {
        let next_direction: Direction;

        match self.direction {
            Direction::RIGHT => next_direction = Direction::UP,
            Direction::UP => next_direction = Direction::LEFT,
            Direction::LEFT => next_direction = Direction::DOWN,
            Direction::DOWN => next_direction = Direction::RIGHT,
        }

        // Try to move in the new direction; if we encounter a cell set to 0, then
        // that is where we should go next and we change the direction. Otherwise
        // we revert to the previous one.
        let old_direction = self.direction;
        self.direction = next_direction;
        let (new_x, new_y) = self.get_coords_to_move();

        if self.grid[new_x][new_y] != 0 {
            self.direction = old_direction;
        }
    }
}

// For part two I build each item in the grid until I find the result.
pub fn day_three_part_two() {
    let input = 325489;
    let edge: usize = 10;

    let x: usize = (edge as f32 / 2.0).ceil() as usize;
    let mut grid = Grid::new(edge, Direction::RIGHT, x, x);

    grid.set_cell(1);

    loop {
        grid.move_forward();

        if grid.is_outside_bounds() {
            println!("We have completed the grid without finding the result.");
            break;
        }

        let sum = grid.sum_all_around();
        if sum > input {
            println!("The result is {}.", sum);
            break;
        }

        grid.set_cell(sum);
        grid.maybe_change_direction();
    }
}
