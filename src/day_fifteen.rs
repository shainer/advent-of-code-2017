struct Generator {
    factor: u64,
    divisor: u64,
    current_value: u64,
    starting_value: u64,
}

impl Generator {
    fn new(factor: u64, divisor: u64, starting_value: u64) -> Generator {
        Generator {
            factor,
            divisor,
            current_value: starting_value,
            starting_value,
        }
    }

    fn next(&mut self) -> u64 {
        self.current_value = (self.current_value * self.factor) % self.divisor;
        return self.current_value;
    }

    fn reset(&mut self) {
        self.current_value = self.starting_value;
    }

    fn next_picky(&mut self, multiple: u64) -> u64 {
        loop {
            let n = self.next();

            if n % multiple == 0 {
                return n;
            }
        }
    }
}

fn lowest_bits_match(v1: u64, v2: u64) -> bool {
    let base: u64 = 2;
    let b1 = v1 & (base.pow(16) - 1);
    let b2 = v2 & (base.pow(16) - 1);

    b1 == b2
}

pub fn day_fifteen() {
    let mut a = Generator::new(16807, 2147483647, 883);
    let mut b = Generator::new(48271, 2147483647, 879);
    let mut judge_count = 0;
    let mut second_judge_count = 0;
    let ten: u64 = 10;

    for _ in 0..40 * ten.pow(6) {
        if lowest_bits_match(a.next(), b.next()) {
            judge_count += 1;
        }
    }

    a.reset();
    b.reset();

    for _ in 0..5 * ten.pow(6) {
        if lowest_bits_match(a.next_picky(4), b.next_picky(8)) {
            second_judge_count += 1;
        }
    }

    println!("Day 15 part 1. Judge count is {}.", judge_count);
    println!(
        "Day 15 part 2. Second judge count is {}.",
        second_judge_count
    );
}
