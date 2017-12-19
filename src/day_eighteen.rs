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

// Simulator for part 1: has only one program and stores the last frequency played.
struct Simulator<'a> {
    registers: HashMap<String, i64>,
    commands: &'a Vec<Command>,
    last_sound: i64,
}

impl<'a> Simulator<'a> {
    fn new(commands: &Vec<Command>) -> Simulator {
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

// Simulator for part 2: manages multiple "execution environments", one for each new program.
// Handles communication between environments through queues of messages (integer values).
struct Program {
    registers: HashMap<String, i64>,
    is_waiting: bool,
    send_total_count: u32,
    send_queue: Vec<i64>,
    recv_queue: Vec<i64>,
}

impl Program {
    fn new(id: i64) -> Program {
        let mut rs: HashMap<String, i64> = HashMap::new();
        rs.insert(String::from("p"), id);

        Program {
            registers: rs,
            is_waiting: false,
            send_total_count: 0,
            send_queue: Vec::new(),
            recv_queue: Vec::new(),
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

    fn run_command(&mut self, command: &Command) -> (i64, bool) {
        let mut ip_offset: i64 = 1;
        let mut has_new_message = false;

        match command.instruction {
            Instruction::PlaySound => {
                let value = self.get_naked_value_or_register(&command.operands[0]);
                self.send_queue.insert(0, value);
                self.send_total_count += 1;
                has_new_message = true;
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

                if self.recv_queue.is_empty() {
                    ip_offset = 0;
                    self.is_waiting = true;
                } else {
                    self.is_waiting = false;
                    let value = self.recv_queue.pop().unwrap();
                    self.registers.insert(register, value);
                }
            }
            Instruction::JumpIfPositive => {
                let cond_value = self.get_naked_value_or_register(&command.operands[0]);

                if cond_value > 0 {
                    ip_offset = self.get_naked_value_or_register(&command.operands[1]);
                }
            }
        };

        (ip_offset, has_new_message)
    }
}

struct Simulator2<'b> {
    program0: Program,
    program1: Program,
    commands: &'b Vec<Command>,
}

impl<'b> Simulator2<'b> {
    fn new(commands: &Vec<Command>) -> Simulator2 {
        Simulator2 {
            program0: Program::new(0),
            program1: Program::new(1),
            commands,
        }
    }

    fn run_commands(&mut self) -> u32 {
        let mut index0: i64 = 0;
        let mut index1: i64 = 0;

        loop {
            if index0 >= 0 && index0 < self.commands.len() as i64 {
                let command = &self.commands[index0 as usize];
                let (offset, has_message) = self.program0.run_command(command);

                index0 += offset;
                if has_message {
                    let message = self.program0.send_queue.pop().unwrap();
                    self.program1.recv_queue.insert(0, message);
                }
            }

            if index1 >= 0 && index1 < self.commands.len() as i64 {
                let command = &self.commands[index1 as usize];
                let (offset, has_message) = self.program1.run_command(command);

                index1 += offset;
                if has_message {
                    let message = self.program1.send_queue.pop().unwrap();
                    self.program0.recv_queue.insert(0, message);
                }
            }

            if self.program0.is_waiting && self.program1.is_waiting {
                println!("Terminated by deadlock!");
                break;
            }

            if (index0 < 0 || index0 >= self.commands.len() as i64) &&
                (index1 < 0 || index1 >= self.commands.len() as i64)
            {
                println!("Both programs reached the end.");
                break;
            }
        }

        self.program1.send_total_count
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

    {
        let mut simulator = Simulator::new(&commands);
        match simulator.run_commands() {
            Some(x) => println!("Day 18 part 1. Result is {}.", x),
            None => panic!("Could not compute solution."),
        };
    }

    {
        let mut new_simulator = Simulator2::new(&commands);
        println!(
            "Day 18 part 2. Program 1 has sent {} messages.",
            new_simulator.run_commands()
        );
    }
}
