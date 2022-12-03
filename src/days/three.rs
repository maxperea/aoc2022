use regex::Regex;
use std::collections::HashSet;

pub fn solution_easy(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut items = HashSet::new();
            let mut duplicate: Option<char> = None;
            for (i, x) in line.chars().enumerate() {
                if i < (line.len() / 2) {
                    items.insert(x);
                } else if items.contains(&x) {
                    duplicate = Some(x);
                } else {
                    continue;
                }
            }
            let priority = alpha_to_priority(duplicate.unwrap()).unwrap();
            priority
        })
        .sum()
}

pub fn solution_hard(input: &str) -> i32 {
    let re = Regex::new(r"\w+\n\w+\n\w+").unwrap();
    let mut sum = 0;
    for cap in re.captures_iter(input) {
        sum += badge_score(&cap[0]);
    }
    sum
}

pub fn badge_score(input: &str) -> i32 {
    let mut group1 = HashSet::new();
    let mut group2 = HashSet::new();
    let lines: Vec<&str> = input.lines().collect();
    for c in lines[0].chars() {
        group1.insert(c);
    }
    for c in lines[1].chars() {
        group2.insert(c);
    }
    for c in lines[2].chars() {
        if group1.contains(&c) && group2.contains(&c) {
            return alpha_to_priority(c).unwrap();
        }
    }

    0
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
