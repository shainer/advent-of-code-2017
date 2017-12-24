use std::collections::HashMap;
use utils::read_input;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Instruction {
    PlaySound,
    Set,
    Sub,
    Add,
    Mul,
    Mod,
    RecoverSound,
    JumpIfNotZero,
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
    instruction_count: HashMap<Instruction, u32>,
    last_sound: i64,
}

impl<'a> Simulator<'a> {
    fn new(commands: &Vec<Command>) -> Simulator {
        Simulator {
            registers: HashMap::new(),
            commands,
            instruction_count: HashMap::new(),
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

    fn get_instruction_count(&self, i: Instruction) -> u32 {
        *self.instruction_count.get(&i).unwrap()
    }

    // fn run_code(&mut self) {
    //     let mut a = 1;
    //     let mut b = 67;
    //     let mut c = b;
    //     let mut g = 0;
    //
    //     b *= 100;
    //     b -= 100000;
    //     c = b;
    //     c -= 17000;
    //     let mut f = 1;
    //     let mut d = 2;
    //     let mut e = 0;
    //     let mut h = 0;
    //
    //     loop {
    //         e = 2;
    //         loop {
    //             g = d;
    //             g *= e;
    //             g -= b;
    //
    //             if g == 0 {
    //                 f = 0;
    //             }
    //
    //             e -= 1;
    //             g = e;
    //             g -= b;
    //
    //             if g == 0 {
    //                 break;
    //             }
    //         }
    //
    //         d -= 1;
    //         g = d;
    //         g -= b;
    //         if g == 0 {
    //             break;
    //         }
    //     }
    //
    //     if f == 0 {
    //         h -= 1;
    //     }
    //
    //     jnz f 2
    //     sub h -1
    //     set g b
    //     sub g c
    //     jnz g 2
    //     jnz 1 3
    //     sub b -17
    //     jnz 1 -23
    //
    // }

    // 1001/1000 too high.
    // 500/500 too low.
    // 750 incorrect.
    fn run_commands(&mut self) {
        let mut index: i64 = 0;
        let mut count = 0;
        let limit = 51;

        while index >= 0 && index < self.commands.len() as i64 {
            let command = &self.commands[index as usize];
            {
                let instr_count = self.instruction_count
                    .entry(command.instruction.clone())
                    .or_insert(0);
                *instr_count += 1;
            }

            index += 1;
            count += 1;

            match command.instruction {
                Instruction::PlaySound => {
                    self.last_sound = self.get_naked_value_or_register(&command.operands[0]);
                }
                Instruction::Set => {
                    let register = command.operands[0].get_register().unwrap();
                    let value = self.get_naked_value_or_register(&command.operands[1]);

                    if count < limit {
                        println!("[{}] Set {} to {}", index - 1, register, value);
                    }
                    self.registers.insert(register, value);
                }
                Instruction::Add => {
                    let base_reg = command.operands[0].get_register().unwrap();
                    let old_value = self.get_register_value(&base_reg);
                    let addend = self.get_naked_value_or_register(&command.operands[1]);

                    self.registers.insert(base_reg, old_value + addend);
                }
                Instruction::Sub => {
                    let base_reg = command.operands[0].get_register().unwrap();
                    let old_value = self.get_register_value(&base_reg);
                    let addend = self.get_naked_value_or_register(&command.operands[1]);

                    if count < limit {
                        println!(
                            "[{}] {} - {} = {}",
                            index - 1,
                            base_reg,
                            addend,
                            old_value - addend
                        );
                    }
                    self.registers.insert(base_reg, old_value - addend);
                }
                Instruction::Mul => {
                    let base_reg = command.operands[0].get_register().unwrap();
                    let old_value = self.get_register_value(&base_reg);
                    let multiplier = self.get_naked_value_or_register(&command.operands[1]);

                    if count < limit {
                        println!(
                            "[{}] {} * {} = {}",
                            index - 1,
                            base_reg,
                            multiplier,
                            old_value * multiplier
                        );
                    }
                    self.registers.insert(base_reg, old_value * multiplier);
                }
                Instruction::Mod => {
                    let base_reg = command.operands[0].get_register().unwrap();
                    let old_value = self.get_register_value(&base_reg);
                    let m = self.get_naked_value_or_register(&command.operands[1]);

                    self.registers.insert(base_reg, old_value % m);
                }
                Instruction::RecoverSound => {
                    println!("Last sound was {}", self.last_sound);
                }
                Instruction::JumpIfNotZero => {
                    let cond_value = self.get_naked_value_or_register(&command.operands[0]);

                    if cond_value != 0 {
                        let offset = self.get_naked_value_or_register(&command.operands[1]);
                        if count < limit {
                            println!("[{}] JNZ by {}", index - 1, offset);
                        }
                        index -= 1;
                        index += offset;
                    } else {
                        if count < limit {
                            println!(
                                "[{}] JNZ did not go, value of {} is {}",
                                index - 1,
                                command.operands[0].get_register().unwrap(),
                                cond_value
                            );
                        }
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
    }
}

fn string_to_instruction(command: &str) -> Instruction {
    match command {
        "snd" => Instruction::PlaySound,
        "set" => Instruction::Set,
        "sub" => Instruction::Sub,
        "add" => Instruction::Add,
        "mul" => Instruction::Mul,
        "mod" => Instruction::Mod,
        "rcv" => Instruction::RecoverSound,
        "jnz" => Instruction::JumpIfNotZero,
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

pub fn day_twenty_three() {
    let program = read_input("data/day_twenty_three_optimized.txt");
    let commands = parse_commands(&program);

    // {
    //     let mut simulator = Simulator::new(&commands);
    //     simulator.run_commands();
    //     println!(
    //         "Day 23 part 1. Mul has been executed {} times.",
    //         simulator.get_instruction_count(Instruction::Mul)
    //     );
    // }

    {
        let mut simulator = Simulator::new(&commands);
        simulator.registers.insert(String::from("a"), 1);
        simulator.run_commands();
        println!(
            "Day 23 part 2. Register h has value {}.",
            simulator.get_register_value(&String::from("h"))
        );
    }
}
