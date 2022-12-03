use regex::Regex;
use std::collections::HashSet;

pub fn solution_easy(input: &str) -> i32 {
    input.lines().map(line_score).sum()
}

pub fn solution_hard(input: &str) -> i32 {
    let re = Regex::new(r"\w+\n\w+\n\w+").unwrap();
    re.captures_iter(input)
        .map(|cap| badge_score(&cap[0]))
        .sum()
}

pub fn line_score(line: &str) -> i32 {
    let (first, second) = line.split_at(line.len() / 2);
    let first_set: HashSet<_> = first.chars().into_iter().collect();
    let second_set: HashSet<_> = second.chars().into_iter().collect();
    alpha_to_priority(*first_set.intersection(&second_set).last().unwrap()).unwrap()
}

pub fn badge_score(input: &str) -> i32 {
    let sets: Vec<HashSet<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let intersection = sets.iter().skip(1).fold(sets[0].clone(), |acc, hs| {
        acc.intersection(hs).cloned().collect()
    });
    alpha_to_priority(*intersection.iter().next().unwrap()).unwrap()
}

pub fn alpha_to_priority(alpha: char) -> Option<i32> {
    if alpha.is_uppercase() {
        return Some(26 + alpha as i32 - '@' as i32);
    } else if alpha.is_lowercase() {
        return Some(alpha as i32 - '`' as i32);
    } else {
        return None;
    }
}
