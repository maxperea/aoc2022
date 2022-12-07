use std::collections::{HashSet, VecDeque};

pub fn solution_easy(input: &str) -> i32 {
    find_signal(input, 4).unwrap()
}

pub fn solution_hard(input: &str) -> i32 {
    find_signal(input, 14).unwrap()
}

pub fn find_signal(input: &str, signal_length: usize) -> Option<i32> {
    let mut word = VecDeque::new();
    for (i, c) in input.chars().enumerate() {
        word.push_back(c);
        if word.len() == signal_length {
            if all_unique(&word) {
                return Some(i as i32 + 1);
            } else {
                word.pop_front();
            }
        }
    }
    None
}

fn all_unique(word: &VecDeque<char>) -> bool {
    let wset: HashSet<&char> = word.iter().collect();
    wset.len() == word.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input0 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let input1 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let input2 = "nppdvjthqldpwncqszvftbrmjlhg";
        let input3 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let input4 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(solution_easy(&input0), 7);
        assert_eq!(solution_easy(&input1), 5);
        assert_eq!(solution_easy(&input2), 6);
        assert_eq!(solution_easy(&input3), 10);
        assert_eq!(solution_easy(&input4), 11);
    }

    #[test]
    fn test2() {
        let input0 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let input1 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let input2 = "nppdvjthqldpwncqszvftbrmjlhg";
        let input3 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let input4 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(solution_hard(&input0), 19);
        assert_eq!(solution_hard(&input1), 23);
        assert_eq!(solution_hard(&input2), 23);
        assert_eq!(solution_hard(&input3), 29);
        assert_eq!(solution_hard(&input4), 26);
    }
}
