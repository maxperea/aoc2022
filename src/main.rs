use crate::days::day17::*;
pub mod days;

fn main() {
    const FILE_NAME: &str = "input/latest";
    let input = std::fs::read_to_string(FILE_NAME).expect("File not found.");

    let now = std::time::Instant::now();
    println!("Easy answer is: {}", solution_easy(&input));
    let ms = now.elapsed().as_micros();
    println!("Solution found in: {}.{}ms", ms / 1000, ms % 1000);

    let now = std::time::Instant::now();
    println!("Hard answer is: {}", solution_hard(&input));
    let ms = now.elapsed().as_micros();
    println!("Solution found in: {}.{}ms", ms / 1000, ms % 1000);
}
