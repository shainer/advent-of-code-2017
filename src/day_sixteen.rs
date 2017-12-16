use utils::read_input;

struct Dance {
    dancers: Vec<char>,
}

impl Dance {
    fn new() -> Dance {
        let d: Vec<char> = [
            'a',
            'b',
            'c',
            'd',
            'e',
            'f',
            'g',
            'h',
            'i',
            'j',
            'k',
            'l',
            'm',
            'n',
            'o',
            'p',
        ].to_vec();

        Dance { dancers: d }
    }

    fn spin(&mut self, x: u32) {
        for _ in 0..x {
            let dancer = match self.dancers.pop() {
                Some(item) => item,
                None => panic!("Not enough dancers to spin."),
            };
            self.dancers.insert(0, dancer);
        }
    }

    fn exchange(&mut self, pos_a: usize, pos_b: usize) {
        self.dancers.swap(pos_a, pos_b);
    }

    fn partner(&mut self, a: char, b: char) {
        let pos_a = match self.dancers.iter().position(|&x| x == a) {
            Some(p) => p,
            None => panic!("Dancer {} not found.", a),
        };
        let pos_b = match self.dancers.iter().position(|&x| x == b) {
            Some(p) => p,
            None => panic!("Dancer {} not found.", b),
        };

        self.exchange(pos_a, pos_b);
    }

    fn final_order(&self) -> String {
        let mut res: String = String::with_capacity(16);

        for i in 0..self.dancers.len() {
            res.push(self.dancers[i]);
        }

        res
    }
}

pub fn day_sixteen() {
    let mut dance = Dance::new();
    let input_content = read_input("data/day_sixteen.txt");

    // If the initial position is iteration 0, by using a HashSet we find that dance positions
    // repeat themselves after 30 iterations. 10**9 % 30 = 10. So we only need 10 iterations of
    // the dance to find the final position after one billion iterations.
    for count in 1..11 {
        for command in input_content.split(',') {
            if command.is_empty() {
                continue;
            }

            match command.chars().nth(0).unwrap() {
                's' => {
                    let spin_num: u32 = command[1..].parse().expect("Not a number.");
                    dance.spin(spin_num);
                }
                'x' => {
                    let pieces: Vec<&str> = command[1..].split('/').collect();
                    let pos_a: usize = pieces[0].trim().parse().expect("Not a number.");
                    let pos_b: usize = pieces[1].trim().parse().expect("Not a number.");
                    dance.exchange(pos_a, pos_b);
                }
                'p' => {
                    let pieces: Vec<&str> = command[1..].split('/').collect();
                    let a: &str = pieces[0].trim();
                    let b: &str = pieces[1].trim();
                    dance.partner(a.chars().next().unwrap(), b.chars().next().unwrap());
                }
                _ => panic!("Unrecognized command {}", command),
            };
        }
        let final_order = dance.final_order();

        if count == 1 {
            println!(
                "Day 16 part 1. Final order of the dancers is {}.",
                final_order
            );
        }
    }

    println!(
        "Day 16 part 2. Final order of the dancers is {}.",
        dance.final_order()
    );
}
