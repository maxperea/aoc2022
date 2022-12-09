use std::collections::{HashSet, VecDeque};

pub fn solution_easy(input: &str) -> i32 {
    find_signal(input, 4).unwrap()
}

pub fn solution_hard(input: &str) -> i32 {
    find_signal(input, 14).unwrap()
}

fn find_signal(input: &str, signal_length: usize) -> Option<i32> {
    let mut word = VecDeque::new();
    for (i, c) in input.chars().enumerate() {
        word.push_back(c);
        if word.len() == signal_length {
            if all_unique(&word) {
                return Some(i as i32 + 1);
            } else {
                word.pop_front();
            }
        }
    }
    None
}

fn all_unique(word: &VecDeque<char>) -> bool {
    let wset: HashSet<&char> = word.iter().collect();
    wset.len() == word.len()
}
