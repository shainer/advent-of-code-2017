use utils::read_input;
use std::collections::HashSet;
use std::iter::FromIterator;

fn sort_word_chars(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    String::from_iter(chars)
}

pub fn day_four_part_two() {
    let contents = read_input("data/day_four.txt");
    let mut valid_count = 0;

    for passphrase in contents.split('\n') {
        let mut sorted_words: HashSet<String> = HashSet::new();
        let mut is_good = true;

        for word in passphrase.split(' ') {
            let sorted_word = sort_word_chars(word);

            if sorted_words.contains(&sorted_word) {
                is_good = false;
                break;
            }

            sorted_words.insert(sorted_word);
        }

        if is_good && sorted_words.len() > 1 {
            valid_count += 1;
        }
    }

    println!(
        "Day 4 part 2. Count of valid passphrases is {}.",
        valid_count
    );
}

pub fn day_four() {
    let contents = read_input("data/day_four.txt");
    let mut valid_count = 0;

    for passphrase in contents.split('\n') {
        let mut words = HashSet::new();
        let mut is_good = true;

        for word in passphrase.split(' ') {
            if words.contains(word) {
                is_good = false;
                break;
            }
            words.insert(word);
        }

        if is_good && words.len() > 1 {
            valid_count += 1;
        }
    }

    println!(
        "Day 4 part 1. Count of valid passphrases is {}",
        valid_count
    );
}
