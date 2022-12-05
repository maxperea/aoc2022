use regex::Regex;

pub fn parse(input: &str) -> Vec<Vec<i32>> {
    let re = Regex::new(r"\d+").unwrap();
    input
        .lines()
        .map(|line| {
            re.captures_iter(line)
                .map(|cap| cap[0].parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn solution_easy(input: &str) -> i32 {
    let data = parse(input);
    data.iter()
        .map(|line| {
            if line[0] <= line[2] && line[1] >= line[3] {
                1
            } else if line[2] <= line[0] && line[3] >= line[1] {
                1
            } else {
                0
            }
        })
        .sum()
}

pub fn solution_hard(input: &str) -> i32 {
    let data = parse(input);
    data.iter()
        .map(|line| {
            if line[1] < line[2] || line[3] < line[0] {
                0
            } else {
                1
            }
        })
        .sum()
}
