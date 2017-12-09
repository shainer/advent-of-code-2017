use std::collections::HashSet;
use utils::read_input;

fn read_integers(filename: &str) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for line in read_input(filename).split('\n') {
        if line.len() == 0 {
            break;
        }

        result.push(line.parse().expect("Not a number"));
    }

    result
}

fn find_max_index(nums: &Vec<i32>) -> usize {
    let mut max: i32 = 0;
    let mut max_index: usize = 0;

    for i in 0..nums.len() {
        if nums[i] > max {
            max = nums[i];
            max_index = i;
        }
    }

    max_index
}

fn one_cycle(mut nums: Vec<i32>) -> (i32, Vec<i32>) {
    let mut configs: HashSet<Vec<i32>> = HashSet::new();
    let mut count_steps = 0;

    loop {
        if configs.contains(&nums) {
            break;
        }

        configs.insert(nums.clone());
        let max_index = find_max_index(&nums);
        let mut blocks = nums[max_index];
        let mut index = (max_index + 1) % (nums.len());
        nums[max_index] = 0;

        while blocks > 0 {
            nums[index] += 1;
            blocks -= 1;
            index = (index + 1) % (nums.len());
        }

        count_steps += 1;
    }

    (count_steps, nums)
}

pub fn day_six() {
    let nums: Vec<i32> = read_integers("data/day_six.txt");
    let (count_steps, new_config) = one_cycle(nums);
    let (count_steps_two, _) = one_cycle(new_config);

    println!(
        "Day 6 part 1. It took {} steps to return to a known configuration.",
        count_steps
    );
    println!("Day 6 part 2. It took {} steps.", count_steps_two);
}
