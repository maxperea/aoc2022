pub fn solution_easy(input: &str) -> i32 {
    let calories: Vec<i32> = input
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|line| line.to_string().parse::<i32>().unwrap_or(0))
                .sum::<i32>()
        })
        .collect();

    *calories.iter().max().unwrap()
}

pub fn solution_hard(input: &str) -> i32 {
    let mut calories: Vec<i32> = input
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|line| line.to_string().parse::<i32>().unwrap_or(0))
                .sum::<i32>()
        })
        .collect();

    calories.sort();
    calories.reverse();
    calories[..3].iter().sum()
}
