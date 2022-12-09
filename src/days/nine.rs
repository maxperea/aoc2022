pub fn solution_easy(input: &str) -> i32 {
    ropes(parse(input), 2)
}

pub fn solution_hard(input: &str) -> i32 {
    ropes(parse(input), 10)
}

type Position = (i32, i32);

fn parse(input: &str) -> Vec<(&str, i32)> {
    let mut data = vec![];
    for line in input.lines() {
        let words: Vec<&str> = line.split(" ").collect();
        data.push((words[0], words[1].parse().expect("Parse error!")));
    }
    data
}

fn step(dir: &str, (x, y): Position) -> Position {
    match dir {
        "D" => (x, y + 1),
        "U" => (x, y - 1),
        "R" => (x + 1, y),
        "L" => (x - 1, y),
        _ => panic!("Parse error!"),
    }
}

fn follow((x1, y1): Position, (x2, y2): Position) -> Position {
    if (x1 - x2).abs() <= 1 && (y1 - y2).abs() <= 1 {
        return (x1, y1);
    }
    let dx = (x2 - x1).signum();
    let dy = (y2 - y1).signum();
    (x1 + dx, y1 + dy)
}

fn ropes(steps: Vec<(&str, i32)>, size: usize) -> i32 {
    let mut seen = std::collections::HashSet::new();
    let mut tails = vec![(0, 0); size];
    for (dir, times) in steps {
        for _ in 0..times {
            tails[0] = step(dir, tails[0]);
            for t in 1..tails.len() {
                tails[t] = follow(tails[t], tails[t - 1]);
            }
            seen.insert(tails.last().unwrap().clone());
        }
    }
    seen.len() as i32
}
