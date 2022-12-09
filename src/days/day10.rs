pub fn solution_easy(input: &str) -> i32 {
    let data = parse(input);
    data
}

pub fn solution_hard(input: &str) -> i32 {
    let data = parse(input);
    data
}

fn parse(_input: &str) -> i32 {
    let _re = regex::Regex::new(r"\d+").unwrap();
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test() {
        let test_input = fs::read_to_string("input/test").expect("File not found.");
        assert_eq!(solution_easy(&test_input), 0);
        assert_eq!(solution_hard(&test_input), 0);
    }
}
