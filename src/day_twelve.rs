use utils::read_input;
use std::collections::HashMap;
use std::collections::HashSet;

// Builds the adjacency list of the programs from the input.
fn build_adjacency_list(contents: &str) -> HashMap<i32, Vec<i32>> {
    let mut adjacency_list: HashMap<i32, Vec<i32>> = HashMap::new();

    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }

        let pieces: Vec<&str> = line.split(' ').collect();

        let num: i32 = pieces[0].parse().expect("Invalid number in input.");
        let mut neighbours: Vec<i32> = Vec::new();

        for i in 2..pieces.len() {
            let mut tmp = String::from(pieces[i]);

            if pieces[i].ends_with(',') {
                tmp.pop();
            }

            let neighbour: i32 = tmp.parse().expect("Invalid number in input.");
            neighbours.push(neighbour);
        }

        adjacency_list.insert(num, neighbours);
    }

    adjacency_list
}

pub fn day_twelve() {
    let contents = read_input("data/day_twelve.txt");
    let adjacency_list = build_adjacency_list(&contents);

    let mut all_programs: Vec<&i32> = adjacency_list.keys().collect();
    let mut count_groups = 0;

    // This solves part 2 and implicitly part 1 with a trick.
    while !all_programs.is_empty() {
        let mut queue: Vec<i32> = Vec::new();
        let mut seen: HashSet<i32> = HashSet::new();

        let first_element: &i32 = all_programs.pop().expect("Impossible!");
        // Used because remove_item() is still in the experimental builds. This is
        // a viable workaround.
        all_programs.retain(|&x| x != first_element);

        // Each new "first element" starts a new group, so we increment the counter.
        queue.push(*first_element);
        count_groups += 1;

        let mut group_size = 1;
        let mut group_contains_0 = false;

        // Here we build the entire group of programs reachable from |first_element|, and
        // we count its members. If we find program 0 among the members, we will print
        // the number of members at the end.
        while !queue.is_empty() {
            let next = queue.pop().expect(
                "Queue appears to be empty when it should not be.",
            );
            seen.insert(next);

            if next == 0 {
                group_contains_0 = true;
            }

            match adjacency_list.get(&next) {
                Some(neighbours) => {
                    for neighbour in neighbours {
                        if !seen.contains(neighbour) {
                            all_programs.retain(|&x| x != neighbour);
                            queue.push(*neighbour);
                            group_size += 1;
                        }
                    }
                }
                None => (),
            }
        }

        if group_contains_0 {
            println!(
                "Day 12 part 1. Group with program 0 contains {} programs.",
                group_size
            );
        }
    }

    println!("Day 12 part 2. There are {} groups.", count_groups);
}
