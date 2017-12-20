use utils::read_input;
use std::collections::HashSet;

#[derive(Hash, Clone)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl Eq for Vector {}

impl Vector {
    // Initializes to 0 rather than taking values as input, because that is what we need
    // in the parsing function.
    fn new() -> Vector {
        Vector { x: 0, y: 0, z: 0 }
    }

    fn sum(&mut self, other: &Vector) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    fn distance(&self) -> u64 {
        (self.x.abs() + self.y.abs() + self.z.abs()) as u64
    }
}

struct Particle {
    pos: Vector,
    vel: Vector,
    acc: Vector,
}

impl Particle {
    // Initializes to empty vectors rather than taking values as input, because that is what
    // we need in the parsing function.
    fn new() -> Particle {
        Particle {
            pos: Vector::new(),
            vel: Vector::new(),
            acc: Vector::new(),
        }
    }

    fn update(&mut self) {
        self.vel.sum(&self.acc);
        self.pos.sum(&self.vel);
    }

    fn distance(&self) -> u64 {
        self.pos.distance()
    }
}

fn find_char_indices(line: &str, to_find: char) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    let mut i = 0;

    for ch in line.chars() {
        if ch == to_find {
            res.push(i);
        }

        i += 1;
    }

    res
}

fn parse_swarm(contents: &str) -> Vec<Particle> {
    let mut swarm: Vec<Particle> = Vec::new();

    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }

        let open_indices = find_char_indices(line, '<');
        let closed_indices = find_char_indices(line, '>');
        let mut particle = Particle::new();

        for i in 0..open_indices.len() {
            let open_index = open_indices[i];
            let closed_index = closed_indices[i];
            let mut vector = Vector::new();

            let vector_str: &str = &line[open_index + 1..closed_index];
            let mut num_index = 0;
            for num_str in vector_str.split(',') {
                if num_index == 0 {
                    vector.x = num_str.parse().expect("Number");
                } else if num_index == 1 {
                    vector.y = num_str.parse().expect("Number");
                } else if num_index == 2 {
                    vector.z = num_str.parse().expect("Number");
                }

                num_index += 1;
            }

            if i == 0 {
                particle.pos = vector;
            } else if i == 1 {
                particle.vel = vector;
            } else if i == 2 {
                particle.acc = vector;
            }
        }

        swarm.push(particle);
    }

    swarm
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<Vec<f64>> {
    let mut solutions: Vec<f64> = Vec::new();

    if a == 0.0 {
        if b == 0.0 {
            if c == 0.0 {
                use std::f64::NAN;
                solutions.push(NAN);
            } else {
                return None;
            }
        } else {
            solutions.push(-c / b);
        }
    } else {
        let square = b.powf(2.0) - 4.0 * a * c;

        if square == 0.0 {
            solutions.push(-b / (2.0 * a));
        } else if square < 0.0 {
            return None;
        } else {
            solutions.push((-b + square.sqrt()) / (2.0 * a));
            solutions.push((-b - square.sqrt()) / (2.0 * a));
        }
    }

    Some(solutions)
}

fn find_collision(p1: &Particle, p2: &Particle) -> Option<Vec<f64>> {
    // The position along any axis of a particle at time t is given by:
    //   r(t) = r(0) + t*v(0) + G(t)*a(0)
    // where G(t) is the Gaussian sum, i.e. 0+1+...+t = [t(t+1)]/2.
    //
    // So we get a quadratic equation in the form a*t^2 + b*t + c. Here we derive the
    // coefficients for all three equations, one for each dimension.
    let cx: f64 = (p1.pos.x - p2.pos.x) as f64;
    let bx: f64 = (p1.vel.x as f64 - p2.vel.x as f64 + (p1.acc.x as f64 / 2.0) -
                       (p2.acc.x as f64 / 2.0)) as f64;
    let ax: f64 = (p1.acc.x - p2.acc.x) as f64 / 2.0;

    let cy: f64 = (p1.pos.y - p2.pos.y) as f64;
    let by: f64 = (p1.vel.y as f64 - p2.vel.y as f64 + (p1.acc.y as f64 / 2.0) -
                       (p2.acc.y as f64 / 2.0)) as f64;
    let ay: f64 = (p1.acc.y - p2.acc.y) as f64 / 2.0;

    let cz: f64 = (p1.pos.z - p2.pos.z) as f64;
    let bz: f64 = (p1.vel.z as f64 - p2.vel.z as f64 + (p1.acc.z as f64 / 2.0) -
                       (p2.acc.z as f64 / 2.0)) as f64;
    let az: f64 = (p1.acc.z - p2.acc.z) as f64 / 2.0;

    let maybe_sol_x = solve_quadratic(ax, bx, cx);
    let maybe_sol_y = solve_quadratic(ay, by, cy);
    let maybe_sol_z = solve_quadratic(az, bz, cz);

    // We have to find at least one solution for each dimension, and there has to be at least one
    // solution in common between the three. A special case is when the vector with the solution
    // contains one element, NAN: this means the two particles follow the exact same trajectory
    // in that dimension so the equation has infinite solutions.
    if maybe_sol_x.is_none() || maybe_sol_y.is_none() || maybe_sol_z.is_none() {
        return None;
    }

    let sol_x = maybe_sol_x.unwrap();
    let sol_y = maybe_sol_y.unwrap();
    let sol_z = maybe_sol_z.unwrap();
    let mut all_solutions: Vec<f64> = Vec::new();

    if !sol_x[0].is_nan() {
        for x in sol_x {
            if (sol_y.contains(&x) || sol_y[0].is_nan()) &&
                (sol_z.contains(&x) || sol_z[0].is_nan())
            {
                all_solutions.push(x);
            }
        }
    } else if !sol_y[0].is_nan() {
        for y in sol_y {
            if sol_z.contains(&y) || sol_z[0].is_nan() {
                all_solutions.push(y);
            }
        }
    } else {
        for z in sol_z {
            all_solutions.push(z);
        }
    }

    if all_solutions.is_empty() {
        return None;
    }

    Some(all_solutions)
}

pub fn day_twenty() {
    let contents = read_input("data/day_twenty.txt");
    let swarm: Vec<Particle> = parse_swarm(&contents);
    use std::u32::MAX;
    let mut min_distance = MAX;
    let mut min_particle = 0;
    let mut index = 0;

    for mut particle in swarm {
        for _ in 0..10000 {
            particle.update();
        }

        if particle.distance() < min_distance {
            min_distance = particle.distance();
            min_particle = index;
        }

        index += 1;
    }

    println!(
        "Day 20 part 1. Closest particle has index {}.",
        min_particle
    );

    let swarm2: Vec<Particle> = parse_swarm(&contents);
    let mut to_remove: HashSet<usize> = HashSet::new();

    for i in 0..swarm2.len() {
        for j in i + 1..swarm2.len() {
            let solution = find_collision(&swarm2[i], &swarm2[j]);
            if solution.is_some() {
                let mut collisions = solution.unwrap();
                // Only consider collisions that are both positive, and integer numbers. Such
                // collision, if it exists, is the time at which the two particles meet.
                collisions.retain(|&x| x > 0.0 && x.floor() == x);

                if !collisions.is_empty() {
                    to_remove.insert(i);
                    to_remove.insert(j);
                }
            }
        }
    }

    println!(
        "Day 20 part 2. {} particles remain.",
        swarm2.len() - to_remove.len()
    );
}
