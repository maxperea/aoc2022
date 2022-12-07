use regex::Regex;

pub fn parse(input: &str) -> i32 {
    let re = Regex::new(r"\d+").unwrap();
    input
        .lines()
        .map(|line| {
            re.captures_iter(line)
                .map(|cap| cap[0].parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    0
}

pub fn solution_easy(input: &str) -> i32 {
    let data = parse(input);
    data
}

pub fn solution_hard(input: &str) -> i32 {
    let data = parse(input);
    data
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test() {
        let input = fs::read_to_string("input/test").expect("File not found.");
        assert_eq!(parse(&input), 0);
        assert_eq!(solution_easy(&input), 2);
        assert_eq!(solution_hard(&input), 4);
    }
}
