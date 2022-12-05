use crate::days::six::solution_easy;
use crate::days::six::solution_hard;
use std::fs;

pub mod days;

fn main() {
    const FILE_NAME: &str = "input/latest";
    let input = fs::read_to_string(FILE_NAME).expect("File not found.");
    println!("Easy answer is: {}", solution_easy(&input));
    println!("Hard answer is: {}", solution_hard(&input));
}
