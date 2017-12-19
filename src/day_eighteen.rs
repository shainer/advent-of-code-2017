use utils::read_input;
use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    PlaySound,
    Set,
    Add,
    Mul,
    Mod,
    RecoverSound,
    JumpIfPositive,
}

struct Operand {
    register: String,
    value: i64,
    has_register: bool,
}

impl Operand {
    fn from_string(register: String) -> Operand {
        Operand {
            register,
            value: 0,
            has_register: true,
        }
    }

    fn from_value(value: i64) -> Operand {
        Operand {
            register: String::new(),
            value,
            has_register: false,
        }
    }

    fn get_value(&self) -> Option<i64> {
        if self.has_register {
            return None;
        }

        Some(self.value.clone())
    }

    fn get_register(&self) -> Option<String> {
        if !self.has_register {
            return None;
        }

        Some(self.register.clone())
    }
}

struct Command {
    instruction: Instruction,
    operands: Vec<Operand>,
}

struct Simulator {
    registers: HashMap<String, i64>,
    commands: Vec<Command>,
    last_sound: i64,
}

impl Simulator {
    fn new(commands: Vec<Command>) -> Simulator {
        Simulator {
            registers: HashMap::new(),
            commands,
            last_sound: 0,
        }
    }

    fn get_register_value(&self, register: &String) -> i64 {
        match self.registers.get(register) {
            Some(x) => *x,
            None => 0,
        }
    }

    fn get_naked_value_or_register(&self, operand: &Operand) -> i64 {
        match operand.get_value() {
            Some(x) => x,
            None => {
                let register = operand.get_register().unwrap();
                self.get_register_value(&register)
            }
        }
    }

    fn run_commands(&mut self) -> Option<i64> {
        let mut index: i64 = 0;

        while index >= 0 && index < self.commands.len() as i64 {
            let command = &self.commands[index as usize];
            index += 1;

            match command.instruction {
                Instruction::PlaySound => {
                    self.last_sound = self.get_naked_value_or_register(&command.operands[0]);
                }
                Instruction::Set => {
                    let register = command.operands[0].get_register().unwrap();
                    let value = self.get_naked_value_or_register(&command.operands[1]);

                    self.registers.insert(register, value);
                }
                Instruction::Add => {
                    let base_reg = command.operands[0].get_register().unwrap();
                    let old_value = self.get_register_value(&base_reg);
                    let addend = self.get_naked_value_or_register(&command.operands[1]);

                    self.registers.insert(base_reg, old_value + addend);
                }
                Instruction::Mul => {
                    let base_reg = command.operands[0].get_register().unwrap();
                    let old_value = self.get_register_value(&base_reg);
                    let multiplier = self.get_naked_value_or_register(&command.operands[1]);

                    self.registers.insert(base_reg, old_value * multiplier);
                }
                Instruction::Mod => {
                    let base_reg = command.operands[0].get_register().unwrap();
                    let old_value = self.get_register_value(&base_reg);
                    let m = self.get_naked_value_or_register(&command.operands[1]);

                    self.registers.insert(base_reg, old_value % m);
                }
                Instruction::RecoverSound => {
                    let register = command.operands[0].get_register().unwrap();
                    let value = self.get_register_value(&register);

                    if value != 0 {
                        return Some(self.last_sound);
                    }
                }
                Instruction::JumpIfPositive => {
                    let cond_value = self.get_naked_value_or_register(&command.operands[0]);

                    if cond_value > 0 {
                        let offset = self.get_naked_value_or_register(&command.operands[1]);
                        index -= 1;
                        index += offset;
                    }
                }
            }
        }

        return None;
    }
}

struct Simulator2 {
    registers1: HashMap<String, i64>,
    registers2: HashMap<String, i64>,
    queue1: Vec<i64>,
    queue2: Vec<i64>,
    is_p1_waiting: bool,
    is_p2_waiting: bool,
    p1_count: u32,
    commands: Vec<Command>,
}

impl Simulator2 {
    fn new(commands: Vec<Command>) -> Simulator2 {
        let mut sim = Simulator2 {
            registers1: HashMap::new(),
            registers2: HashMap::new(),
            queue1: Vec::new(),
            queue2: Vec::new(),
            is_p1_waiting: false,
            is_p2_waiting: false,
            p1_count: 0,
            commands,
        };

        sim.registers1.insert(String::from("p"), 0);
        sim.registers2.insert(String::from("p"), 1);
        sim
    }

    fn get_register_value(&self, register: &String, program_id: u32) -> i64 {
        if program_id == 0 {
            return match self.registers1.get(register) {
                Some(x) => *x,
                None => 0,
            };
        }

        match self.registers2.get(register) {
            Some(x) => *x,
            None => 0,
        }
    }

    fn get_naked_value_or_register(&self, operand: &Operand, program_id: u32) -> i64 {
        match operand.get_value() {
            Some(x) => x,
            None => {
                let register = operand.get_register().unwrap();
                self.get_register_value(&register, program_id)
            }
        }
    }

    fn run_one_command(&mut self, i: i64, program_id: u32) -> i64 {
        let mut index = i;
        let command: &Command = &self.commands[index as usize];
        index += 1;
        let mut ins: (String, i64) = (String::new(), 0);

        match command.instruction {
            Instruction::PlaySound => {
                let value = self.get_naked_value_or_register(&command.operands[0], program_id);

                if program_id == 0 {
                    self.queue2.insert(0, value);
                } else {
                    self.p1_count += 1;
                    self.queue1.insert(0, value);
                }
            }
            Instruction::Set => {
                let register = command.operands[0].get_register().unwrap();
                let value = self.get_naked_value_or_register(&command.operands[1], program_id);
                ins = (register, value);
            }
            Instruction::Add => {
                let base_reg = command.operands[0].get_register().unwrap();
                let old_value = self.get_register_value(&base_reg, program_id);
                let addend = self.get_naked_value_or_register(&command.operands[1], program_id);

                ins = (base_reg, old_value + addend);
            }
            Instruction::Mul => {
                let base_reg = command.operands[0].get_register().unwrap();
                let old_value = self.get_register_value(&base_reg, program_id);
                let multiplier = self.get_naked_value_or_register(&command.operands[1], program_id);

                ins = (base_reg, old_value * multiplier);
            }
            Instruction::Mod => {
                let base_reg = command.operands[0].get_register().unwrap();
                let old_value = self.get_register_value(&base_reg, program_id);
                let m = self.get_naked_value_or_register(&command.operands[1], program_id);

                ins = (base_reg, old_value % m);
            }
            Instruction::RecoverSound => {
                let register = command.operands[0].get_register().unwrap();

                if program_id == 0 {
                    if self.queue1.is_empty() {
                        index -= 1; // stay on the same instruction.
                        self.is_p1_waiting = true;
                    } else {
                        self.is_p1_waiting = false;
                        let value = self.queue1.pop().unwrap();
                        ins = (register, value);
                    }
                } else {
                    if self.queue2.is_empty() {
                        index -= 1;
                        self.is_p2_waiting = true;
                    } else {
                        self.is_p2_waiting = false;
                        let value = self.queue2.pop().unwrap();
                        ins = (register, value);
                    }
                }
            }
            Instruction::JumpIfPositive => {
                let cond_value = self.get_naked_value_or_register(&command.operands[0], program_id);

                if cond_value > 0 {
                    let offset = self.get_naked_value_or_register(&command.operands[1], program_id);
                    index -= 1;
                    index += offset;
                }
            }
        };

        if !ins.0.is_empty() {
            if program_id == 0 {
                self.registers1.insert(ins.0, ins.1);
            } else {
                self.registers2.insert(ins.0, ins.1);
            }
        }

        index
    }

    fn run_commands(&mut self) -> u32 {
        let mut index1: i64 = 0;
        let mut index2: i64 = 0;

        loop {
            if index1 >= 0 && index1 < self.commands.len() as i64 {
                index1 = self.run_one_command(index1, 0);
            }

            if index2 >= 0 && index2 < self.commands.len() as i64 {
                index2 = self.run_one_command(index2, 1);
            }

            if self.is_p1_waiting && self.is_p2_waiting {
                println!("Terminated by deadlock!");
                break;
            }

            if (index1 < 0 || index1 >= self.commands.len() as i64) &&
                (index2 < 0 || index2 >= self.commands.len() as i64)
            {
                println!("Both programs reached the end.");
                break;
            }
        }

        return self.p1_count;
    }
}

fn string_to_instruction(command: &str) -> Instruction {
    match command {
        "snd" => Instruction::PlaySound,
        "set" => Instruction::Set,
        "add" => Instruction::Add,
        "mul" => Instruction::Mul,
        "mod" => Instruction::Mod,
        "rcv" => Instruction::RecoverSound,
        "jgz" => Instruction::JumpIfPositive,
        _ => panic!("Unrecognized command {}.", command),
    }
}

fn parse_commands(program: &String) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();

    for line in program.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut index = 0;
        let mut instr: Instruction = Instruction::Set;
        let mut operands: Vec<Operand> = Vec::new();

        for piece in line.split(' ') {
            if index == 0 {
                instr = string_to_instruction(piece);
            } else {
                let int_operand = piece.parse::<i64>();
                if int_operand.is_err() {
                    operands.push(Operand::from_string(String::from(piece)));
                } else {
                    operands.push(Operand::from_value(int_operand.unwrap()));
                }
            }
            index += 1;
        }

        commands.push(Command {
            instruction: instr,
            operands,
        });
    }

    commands
}

pub fn day_eighteen() {
    let program = read_input("data/day_eighteen.txt");
    let commands = parse_commands(&program);
    // let mut simulator = Simulator::new(commands);
    //
    // let result = simulator.run_commands();
    // match result {
    //     Some(x) => println!("Day 18 part 1. Result is {}.", x),
    //     None => panic!("Could not compute solution."),
    // };

    let mut new_simulator = Simulator2::new(commands);
    println!("Day 18 part 2. Result is {}", new_simulator.run_commands());
    // 8128 too high.
}
