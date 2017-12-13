use utils::read_input;

fn parse_firewall(input: &String) -> Vec<u32> {
    let mut firewall: Vec<u32> = Vec::new();

    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }

        let data: Vec<&str> = line.split(": ").collect();
        assert_eq!(2, data.len());

        let depth: usize = data[0].parse().expect("Expected a number.");
        let range: u32 = data[1].parse().expect("Expected a number.");

        for i in firewall.len()..depth {
            firewall.insert(i, 0);
        }

        firewall.insert(depth, range);
    }

    firewall
}

fn compute_scanner_position(range: u32, picosecond: u32, delay: u32) -> u32 {
    let mut pos = (picosecond + delay) % (range * 2 - 2);

    if pos >= range {
        pos = range - (pos - range) - 1;
    }

    pos
}

pub fn day_thirteen() {
    let contents = read_input("data/day_thirteen.txt");
    let firewall: Vec<u32> = parse_firewall(&contents);

    let mut severity = 0;

    // packet_layer identifies both the layer the packet is found at, and the
    // current picosecond.
    for packet_layer in 0..firewall.len() as u32 {
        let scanner_range: u32 = firewall[packet_layer as usize];
        if scanner_range == 0 {
            continue;
        }

        if compute_scanner_position(scanner_range, packet_layer, 0) == 0 {
            severity += scanner_range * packet_layer;
        }
    }

    let mut pico_delay: u32 = 1;

    loop {
        let mut caught = false;

        for packet_layer in 0..firewall.len() as u32 {
            let scanner_range: u32 = firewall[packet_layer as usize];
            if scanner_range == 0 {
                continue;
            }

            if compute_scanner_position(scanner_range, packet_layer, pico_delay) == 0 {
                caught = true;
                break;
            }
        }

        if !caught {
            break;
        }

        pico_delay += 1;
    }

    println!("Day 13 part 1. Severity of the path is {}.", severity);
    println!("Day 13 part 2. Minimum delay is {}.", pico_delay);

    // 35744 too low.
}
