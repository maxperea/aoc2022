use crate::days::eight::solution_easy;
use crate::days::eight::solution_hard;
use std::fs;
use std::time::Instant;

pub mod days;

fn main() {
    const FILE_NAME: &str = "input/latest";
    let input = fs::read_to_string(FILE_NAME).expect("File not found.");

    let now = Instant::now();
    println!("Easy answer is: {}", solution_easy(&input));
    let ms = now.elapsed().as_micros();
    println!("Solution found in: {}.{}ms", ms / 1000, ms % 1000);

    let now = Instant::now();
    println!("Hard answer is: {}", solution_hard(&input));
    let ms = now.elapsed().as_micros();
    println!("Solution found in: {}.{}ms", ms / 1000, ms % 1000);
}
