use std::collections::HashMap;
use utils::read_input;

struct Program {
    name: String,
    weight: i32,
    children: Vec<String>
}

impl Program {
    fn new(name: String, weight: i32, children : Vec<String>) -> Program {
        Program { name, weight, children }
    }

    fn add_child(mut self, c: String) {
        self.children.push(c);
    }

    fn has_child(&self, child: &String) -> bool {
        self.children.contains(child)
    }

    fn print(&self) {
        println!("Name {} and weight {}, children {:?}", self.name, self.weight, self.children);
    }
}

fn parse_input_programs(filename : &str) -> HashMap<String, Program> {
    let mut programs : HashMap<String, Program> = HashMap::new();

    for line in read_input(filename).split('\n') {
        if line.len() == 0 {
            continue;
        }

        let pieces : Vec<&str> = line.split(' ').collect();
        let name : String = String::from(pieces[0]);

        let mut tmp = String::from(pieces[1]);
        tmp.remove(0);
        tmp.pop();
        let weight : i32 = tmp.parse().expect("Invalid weight found.");
        let mut children : Vec<String> = Vec::new();

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

fn all_children_same_weight(children: &Vec<String>, programs: &HashMap<String, Program>) -> Option<i32> {
    let shared_weight : i32;

    match programs.get(&children[0]) {
        Some(child) => shared_weight = child.weight,
        None => panic!("Expect at least 1 child, got zero")
    }

    for child_name in children {
        match programs.get(child_name) {
            Some(child) => if shared_weight != child.weight { return None; }
            None => panic!("Expect to find child.")
        }
    }

    return Some(shared_weight);
}

fn find_correct_weight(mut programs: HashMap<String, Program>) -> Option<usize> {
    let mut shared_subtower_weights : HashMap<String, i32> = HashMap::new();
    let mut imbalanced_name : String = String::new();

    for p in programs.values() {
        if shared_subtower_weights.contains_key(&p.name) {
            continue;
        }

        if p.children.is_empty() {
            shared_subtower_weights.insert(p.name.clone(), p.weight);
        } else {
            match all_children_same_weight(&p.children, &programs) {
                Some(weight) => { shared_subtower_weights.insert(p.name.clone(), weight); },
                None => { imbalanced_name = p.name.clone(); }
            }
        }
    }

    let imbalanced = programs.get(&imbalanced_name).expect("ciao tesoro");

    println!("Imbalanced tower has root at {}.", imbalanced_name);

    for child_name in &imbalanced.children {
        match shared_subtower_weights.get(child_name) {
            Some(w) => println!("Subtower weight is {}", w),
            None => panic!("Can't recover sorry.")
        }
    }

    None
}

pub fn day_seven() {
    let all_programs = parse_input_programs("data/day_seven.txt");

    let root_name : String = find_root(&all_programs).expect("Could not find root.");
    println!("Day 7 part 1. Root of the tree is program {}.", root_name);

    let result = find_correct_weight(all_programs).expect("Could not find correct weight.");
    println!("Day 7 part 2. Correct weight is {}.", result);
}
