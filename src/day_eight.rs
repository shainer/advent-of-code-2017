use std::collections::HashMap;
use utils::read_input;

enum ComparisonType {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
}

struct Instruction {
    register: String,
    increment: bool,
    value: i32,
    cond_register: String,
    comparison: ComparisonType,
    compared_value: i32,
}

fn parse_line(line: &str) -> Instruction {
    let words: Vec<&str> = line.split(' ').collect();

    let increment: bool = words[1] == "inc";
    let value: i32 = words[2].parse().expect("Syntax error!");
    let comparison = match words[5] {
        "==" => ComparisonType::Equal,
        "!=" => ComparisonType::NotEqual,
        ">" => ComparisonType::GreaterThan,
        ">=" => ComparisonType::GreaterThanOrEqualTo,
        "<" => ComparisonType::LessThan,
        "<=" => ComparisonType::LessThanOrEqualTo,
        _ => panic!("Unrecognized comparison type {}.", words[5]),
    };
    let compared_value: i32 = words[6].parse().expect("Syntax error!");

    Instruction {
        register: String::from(words[0]),
        increment,
        value,
        cond_register: String::from(words[4]),
        comparison,
        compared_value,
    }
}

fn eval_condition(reg_value: &i32, comparison: ComparisonType, comp_value: &i32) -> bool {
    match comparison {
        ComparisonType::Equal => reg_value == comp_value,
        ComparisonType::NotEqual => reg_value != comp_value,
        ComparisonType::GreaterThan => reg_value > comp_value,
        ComparisonType::GreaterThanOrEqualTo => reg_value >= comp_value,
        ComparisonType::LessThan => reg_value < comp_value,
        ComparisonType::LessThanOrEqualTo => reg_value <= comp_value,
    }
}

pub fn day_eight() {
    let contents = read_input("data/day_eight.txt");
    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut global_max_value = 0;

    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }

        let instruction: Instruction = parse_line(&line);
        let cond_register_value = match registers.get(&instruction.cond_register) {
            Some(x) => *x,
            None => 0,
        };

        if eval_condition(
            &cond_register_value,
            instruction.comparison,
            &instruction.compared_value,
        )
        {
            let mut v: i32 = match registers.get(&instruction.register) {
                Some(x) => *x,
                None => 0,
            };
            if instruction.increment {
                v += instruction.value;
            } else {
                v -= instruction.value;
            }

            if v > global_max_value {
                global_max_value = v;
            }
            registers.insert(instruction.register, v);
        }
    }

    let mut max_value: i32 = 0;

    for (_, value) in registers {
        if value > max_value {
            max_value = value;
        }
    }

    println!("Day 8 part 1. Maximum value is {}.", max_value);
    println!(
        "Day 8 part 2. Global maximum value is {}.",
        global_max_value
    );
}
