struct TuringMachine {
    tape: Vec<u8>,
    current_state: char,
    cursor: usize,
    diagnostic_steps: u32,
    one_counter: u32,
}

impl TuringMachine {
    fn new() -> TuringMachine {
        TuringMachine {
            tape: Vec::new(),
            current_state: 'A',
            cursor: 0,
            diagnostic_steps: 12919244,
            one_counter: 0,
        }
    }

    fn increment_cursor(&mut self, positive: bool) {
        if !positive {
            if self.cursor == 0 {
                self.tape.insert(0, 0);
            } else {
                self.cursor -= 1;
            }
        } else {
            if self.cursor == self.tape.len() - 1 {
                self.tape.push(0);
            }

            self.cursor += 1;
        }
    }

    fn set_cell(&mut self, value: u8) {
        self.tape[self.cursor] = value;

        if value == 1 {
            self.one_counter += 1;
        } else {
            self.one_counter -= 1;
        }
    }

    fn run(&mut self) {
        self.tape.push(0);

        for _ in 0..self.diagnostic_steps {
            let value = self.tape[self.cursor];

            match self.current_state {
                'A' => {
                    if value == 0 {
                        self.set_cell(1);
                        self.increment_cursor(true);
                        self.current_state = 'B';
                    } else {
                        self.set_cell(0);
                        self.increment_cursor(false);
                        self.current_state = 'C';
                    }
                }
                'B' => {
                    if value == 0 {
                        self.set_cell(1);
                        self.increment_cursor(false);
                        self.current_state = 'A';
                    } else {
                        self.increment_cursor(true);
                        self.current_state = 'D';
                    }
                }
                'C' => {
                    if value == 0 {
                        self.set_cell(1);
                        self.increment_cursor(true);
                        self.current_state = 'A';
                    } else {
                        self.set_cell(0);
                        self.increment_cursor(false);
                        self.current_state = 'E';
                    }
                }
                'D' => {
                    if value == 0 {
                        self.set_cell(1);
                        self.increment_cursor(true);
                        self.current_state = 'A';
                    } else {
                        self.set_cell(0);
                        self.increment_cursor(true);
                        self.current_state = 'B';
                    }
                }
                'E' => {
                    if value == 0 {
                        self.set_cell(1);
                        self.increment_cursor(false);
                        self.current_state = 'F';
                    } else {
                        self.increment_cursor(false);
                        self.current_state = 'C';
                    }
                }
                'F' => {
                    if value == 0 {
                        self.set_cell(1);
                        self.increment_cursor(true);
                        self.current_state = 'D';
                    } else {
                        self.increment_cursor(true);
                        self.current_state = 'A';
                    }
                }
                _ => panic!("Unrecognized state {}.", self.current_state),
            }
        }
    }
}

pub fn day_twenty_five() {
    let mut m = TuringMachine::new();
    m.run();

    println!(
        "Day 25 part 1. Number of 1s at diagnostic time is {}.",
        m.one_counter
    );
}
