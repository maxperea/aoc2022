use regex::Regex;
use std::collections::VecDeque;

pub fn solution_easy(input: &str) -> String {
    let mut res = String::new();
    let (mut stacks, moves) = parse(&input);
    for m in moves {
        let amount = m[0];
        for _ in 0..amount {
            let from_index = m[1] - 1;
            let to_index = m[2] - 1;
            let c = stacks[from_index].pop_back().unwrap();
            stacks[to_index].push_back(c);
        }
    }
    for mut stack in stacks {
        res += &stack.pop_back().unwrap().to_string();
    }
    res
}

pub fn solution_hard(input: &str) -> String {
    let mut res = String::new();
    let (mut stacks, moves) = parse(&input);
    for m in moves {
        let amount = m[0];
        let mut middle_stack = VecDeque::new();
        let from_index = m[1] - 1;
        let to_index = m[2] - 1;
        for _ in 0..amount {
            let c = stacks[from_index].pop_back().unwrap();
            middle_stack.push_back(c);
        }
        while let Some(item) = middle_stack.pop_back() {
            stacks[to_index].push_back(item);
        }
    }
    for mut stack in stacks {
        res += &stack.pop_back().unwrap().to_string();
    }
    res
}

type Move = Vec<usize>;
type Stack = VecDeque<char>;

fn parse(input: &str) -> (Vec<Stack>, Vec<Move>) {
    let strs: Vec<&str> = input.split("\n\n").collect();
    let stacks: Vec<Stack> = parse_stacks(strs[0]);
    let moves: Vec<Move> = parse_moves(strs[1]);

    return (stacks, moves);
}

fn parse_stacks(input: &str) -> Vec<Stack> {
    let count = 9;
    let mut stacks: Vec<Stack> = Vec::new();
    for _ in 0..count {
        let stack = VecDeque::new();
        stacks.push(stack);
    }

    let lines: Vec<Vec<char>> = input.lines().rev().map(|l| l.chars().collect()).collect();

    for line in lines {
        for i in 0..count {
            let index = 1 + 4 * i;
            if line[index].is_alphabetic() {
                stacks[i].push_back(line[index]);
            }
        }
    }

    stacks
}

fn parse_moves(input: &str) -> Vec<Move> {
    let re = Regex::new(r"\d+").unwrap();
    input
        .lines()
        .map(|line| {
            re.captures_iter(line)
                .map(|cap| cap[0].parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}
