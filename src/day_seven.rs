use std::collections::HashMap;
use utils::read_input;

struct Program {
    name: String,
    weight: i32,
    children: Vec<String>,
}

impl Program {
    fn new(name: String, weight: i32, children: Vec<String>) -> Program {
        Program {
            name,
            weight,
            children,
        }
    }

    fn has_child(&self, child: &String) -> bool {
        self.children.contains(child)
    }
}

fn parse_input_programs(filename: &str) -> HashMap<String, Program> {
    let mut programs: HashMap<String, Program> = HashMap::new();

    for line in read_input(filename).split('\n') {
        if line.len() == 0 {
            continue;
        }

        let pieces: Vec<&str> = line.split(' ').collect();
        let name: String = String::from(pieces[0]);

        let mut tmp = String::from(pieces[1]);
        tmp.remove(0);
        tmp.pop();
        let weight: i32 = tmp.parse().expect("Invalid weight found.");
        let mut children: Vec<String> = Vec::new();

        if pieces.len() > 3 {
            for i in 3..pieces.len() {
                let mut child = String::from(pieces[i]);

                if i < pieces.len() - 1 {
                    child.pop();
                }
                children.push(child);
            }
        }

        programs.insert(name.clone(), Program::new(name, weight, children));
    }

    programs
}

fn find_root(programs: &HashMap<String, Program>) -> Option<String> {
    for ref p1 in programs.values() {
        let mut is_parentless = true;

        for ref p2 in programs.values() {
            if p1.name == p2.name {
                continue;
            }

            if p2.has_child(&p1.name) {
                is_parentless = false;
                break;
            }
        }

        if is_parentless {
            return Some(p1.name.clone());
        }
    }

    None
}

fn compute_subtower_weight(child: &String, programs: &HashMap<String, Program>) -> i32 {
    let child_program = programs.get(child).expect("Nope.");
    let mut subtower_weight = child_program.weight;

    for grandchild in child_program.children.iter() {
        subtower_weight += compute_subtower_weight(grandchild, programs);
    }

    subtower_weight
}

fn all_weights_the_same(weights: &Vec<i32>) -> bool {
    if weights.is_empty() {
        return true;
    }

    let shared_item = weights[0];
    for item in weights {
        if *item != shared_item {
            return false;
        }
    }

    true
}

fn all_the_same_but_one(weights: &Vec<i32>) -> bool {
    if weights.is_empty() {
        return false;
    }

    for i in 0..weights.len() {
        let shared_item = weights[(i + 1) % weights.len()];
        let mut all_the_same = true;

        for j in 0..weights.len() {
            if i == j {
                continue;
            }

            if weights[j] != shared_item {
                all_the_same = false;
                break;
            }
        }

        if all_the_same {
            return true;
        }
    }

    return false;
}

fn find_correct_weight(programs: HashMap<String, Program>) -> Option<usize> {
    let root_name: String = find_root(&programs).expect("Could not find root.");
    let mut queue: Vec<&String> = Vec::new();

    queue.push(&root_name);

    while !queue.is_empty() {
        let node_name = queue.pop().expect("Impossible.");
        let node = programs.get(node_name).expect("Impossible.");
        let mut weights: Vec<i32> = Vec::new();

        for child in node.children.iter() {
            weights.push(compute_subtower_weight(&child, &programs));
        }

        if !all_weights_the_same(&weights) {
            for child in node.children.iter() {
                queue.push(child);
            }
        }

        if !all_weights_the_same(&weights) && all_the_same_but_one(&weights) {
            println!("{:?}", weights);
            println!("{:?}", node.children);

            for child in node.children.iter() {
                let child_node = programs.get(child).expect("Nope");
                println!("Child {} has weight {}", child, child_node.weight);
            }
        }
    }
    None
}

pub fn day_seven() {
    let all_programs = parse_input_programs("data/day_seven.txt");

    let root_name: String = find_root(&all_programs).expect("Could not find root.");
    println!("Day 7 part 1. Root of the tree is program {}.", root_name);

    let result = find_correct_weight(all_programs).expect("Could not find correct weight.");
    println!("Day 7 part 2. Correct weight is {}.", result);
}
