use std::collections::HashMap;

use Item::*;

pub enum Item {
    Dir(String),
    File(i32),
}

fn parse(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect()
}

fn path(dir_stack: &Vec<String>) -> String {
    dir_stack
        .iter()
        .fold("".to_owned(), |acc, next| acc.to_owned() + next)
}

fn get_directories(data: Vec<Vec<&str>>) -> HashMap<String, Vec<Item>> {
    let mut directories = HashMap::new();
    let mut dir_stack = vec![];

    for line in data {
        match line[..] {
            ["$", "ls"] => {}
            ["$", "cd", ".."] => {
                dir_stack.pop();
            }
            ["$", "cd", dir] => dir_stack.push(dir.to_owned() + "/"),
            ["dir", dir] => directories
                .entry(path(&dir_stack))
                .or_insert(vec![])
                .push(Dir(path(&dir_stack) + dir + "/")),
            [size, _fname] => directories
                .entry(path(&dir_stack))
                .or_insert(vec![])
                .push(File(size.parse().unwrap())),
            _ => {}
        };
    }

    directories
}

fn dir_size(dir: &str, dirs: &HashMap<String, Vec<Item>>) -> i32 {
    let mut size = 0;

    for item in dirs.get(dir).unwrap() {
        match item {
            Dir(file) => {
                size += dir_size(&file, dirs);
            }
            File(fsize) => {
                size += fsize;
            }
        }
    }

    size
}

pub fn solution_easy(input: &str) -> i32 {
    let dirs = get_directories(parse(input));

    let mut total = 0;

    for dir in dirs.keys() {
        let size = dir_size(dir, &dirs);
        if size <= 100000 {
            total += size;
        }
    }

    total
}

pub fn solution_hard(input: &str) -> i32 {
    let dirs = get_directories(parse(input));

    let used_space = dir_size("//", &dirs);
    let free_space = 70000000 - used_space;
    let req_space = 30000000 - free_space;

    let mut ans = used_space;

    for dir in dirs.keys() {
        let size = dir_size(dir, &dirs);
        if size < ans && size >= req_space {
            ans = size;
        }
    }

    ans
}
