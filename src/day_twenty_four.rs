use std::collections::HashMap;
use utils::read_input;

#[derive(Clone, PartialEq)]
struct Port {
    c0: u32,
    c1: u32,
}

fn parse_input() -> HashMap<u32, Vec<Port>> {
    let mut ports: HashMap<u32, Vec<Port>> = HashMap::new();
    let contents = read_input("data/day_twenty_four.txt");

    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }

        let mut port = Port { c0: 0, c1: 0 };
        let mut index = 0;
        for component in line.split('/') {
            if index == 0 {
                port.c0 = component.parse().expect("Not a number");
            } else {
                port.c1 = component.parse().expect("Not a number");
            }

            index += 1;
        }

        {
            let p = ports.entry(port.c0).or_insert(Vec::new());
            p.push(port.clone());
        }

        {
            let p = ports.entry(port.c1).or_insert(Vec::new());
            p.push(port);
        }
    }

    ports
}

static mut MAX_STRENGTH: u32 = 0;
static mut MAX_LENGTH: u32 = 0;

fn find_max_length(ports: HashMap<u32, Vec<Port>>, next_number: u32, length: u32) {
    let good_ports = ports.get(&next_number);

    if good_ports.is_none() || good_ports.unwrap().is_empty() {
        unsafe {
            if length > MAX_LENGTH {
                MAX_LENGTH = length;
                return;
            }
        }
    }

    for port in good_ports.unwrap() {
        let mut clone = ports.clone();

        {
            let p = clone.entry(port.c0).or_insert(Vec::new());
            p.retain(|x| *x != *port);
        }

        {
            let p = clone.entry(port.c1).or_insert(Vec::new());
            p.retain(|x| *x != *port);
        }

        let number: u32;

        if next_number == port.c0 {
            number = port.c1;
        } else {
            number = port.c0;
        }

        find_max_length(clone, number, length + 1);
    }
}

fn find_max_strength_with_min_len(
    ports: HashMap<u32, Vec<Port>>,
    next_number: u32,
    strength: u32,
    length: u32,
    min_length: u32,
) {
    let good_ports = ports.get(&next_number);

    if good_ports.is_none() || good_ports.unwrap().is_empty() {
        unsafe {
            if length >= min_length && strength > MAX_STRENGTH {
                MAX_STRENGTH = strength;
                return;
            }
        }
    }

    for port in good_ports.unwrap() {
        let mut clone = ports.clone();

        {
            let p = clone.entry(port.c0).or_insert(Vec::new());
            p.retain(|x| *x != *port);
        }

        {
            let p = clone.entry(port.c1).or_insert(Vec::new());
            p.retain(|x| *x != *port);
        }

        let new_strength = strength + port.c0 + port.c1;
        let number: u32;

        if next_number == port.c0 {
            number = port.c1;
        } else {
            number = port.c0;
        }

        find_max_strength_with_min_len(clone, number, new_strength, length + 1, min_length);
    }
}

pub fn day_twenty_four() {
    {
        let ports = parse_input();
        unsafe {
            MAX_STRENGTH = 0;
            MAX_LENGTH = 0;
        }
        find_max_strength_with_min_len(ports, 0, 0, 0, 0);

        unsafe {
            println!("Day 24 part 1. Maximum strength is {}.", MAX_STRENGTH);
        }
    }

    unsafe {
        let ports = parse_input();
        MAX_STRENGTH = 0;
        MAX_LENGTH = 0;
        find_max_length(ports.clone(), 0, 0);
        find_max_strength_with_min_len(ports, 0, 0, 0, MAX_LENGTH);
        println!(
            "Day 24 part 2. Strength of longest bridge is {}.",
            MAX_STRENGTH
        );
    }
}
