use crate::days::three::solution_easy;
use crate::days::three::solution_hard;
use std::fs;

pub mod days;

fn main() {
    let input = fs::read_to_string("input/3").expect("File not found.");
    println!("Easy answer is: {}", solution_easy(&input));
    println!("Hard answer is: {}", solution_hard(&input));
}
