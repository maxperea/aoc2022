use std::collections::HashMap;

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

fn dir_stack_to_str(dir_stack: &Vec<String>) -> String {
    dir_stack
        .iter()
        .fold("".to_owned(), |acc, next| acc.to_owned() + next)
        .clone()
}

fn get_directories(data: Vec<Vec<&str>>) -> HashMap<String, Vec<Item>> {
    let mut directories: HashMap<String, Vec<Item>> = HashMap::new();
    let mut dir_stack = vec![];
    for line in data {
        match line[0] {
            "$" => match line[1] {
                "ls" => {}
                "cd" => match line[2] {
                    ".." => {
                        dir_stack.pop();
                    }
                    dir => {
                        dir_stack.push("/".to_owned() + dir);
                    }
                },
                _ => panic!("Parse error!"),
            },
            "dir" => {
                if directories.contains_key(dir_stack_to_str(&dir_stack).as_str()) {
                    directories
                        .get_mut(dir_stack_to_str(&dir_stack).as_str())
                        .unwrap()
                        .push(Item::Dir(dir_stack_to_str(&dir_stack) + "/" + line[1]));
                } else {
                    directories.insert(
                        dir_stack_to_str(&dir_stack),
                        vec![Item::Dir(dir_stack_to_str(&dir_stack) + "/" + line[1])],
                    );
                }
            }
            num => {
                if directories.contains_key(dir_stack_to_str(&dir_stack).as_str()) {
                    directories
                        .get_mut(dir_stack_to_str(&dir_stack).as_str())
                        .unwrap()
                        .push(Item::File(num.parse().unwrap()));
                } else {
                    directories.insert(
                        dir_stack_to_str(&dir_stack),
                        vec![Item::File(num.parse().unwrap())],
                    );
                }
            }
        }
    }
    directories
}

fn size_of_dir(dir: &str, dirs: &HashMap<String, Vec<Item>>) -> i32 {
    let mut total_size = 0;
    if let Some(root) = dirs.get(dir) {
        for item in root {
            match item {
                Item::Dir(file) => {
                    total_size += size_of_dir(&file, dirs);
                }
                Item::File(size) => {
                    total_size += size;
                }
            }
        }
    }
    total_size
}

pub fn solution_easy(input: &str) -> i32 {
    let data = parse(input);
    let dirs = get_directories(data);

    let mut total = 0;

    for dir in dirs.keys() {
        let size = size_of_dir(dir, &dirs);
        if size <= 100000 {
            total += size;
        }
    }

    total
}

pub fn solution_hard(input: &str) -> i32 {
    let data = parse(input);
    let dirs = get_directories(data);
    let used_space = size_of_dir("//", &dirs);
    let free_space = 70000000 - used_space;
    let required_space = 30000000 - free_space;

    let mut candidate = used_space;

    for dir in dirs.keys() {
        let test = size_of_dir(dir, &dirs);
        if test < candidate && test >= required_space {
            candidate = test;
        }
    }

    candidate
}
