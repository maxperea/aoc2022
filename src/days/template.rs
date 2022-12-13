pub fn solution_easy(input: &str) -> i64 {
    let data = parse(input);
    unimplemented!()
}

pub fn solution_hard(input: &str) -> i64 {
    let data = parse(input);
    unimplemented!()
}

fn parse(input: &str) -> i64 {
    let re = regex::Regex::new(r"\d+").unwrap();
    unimplemented!()
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
