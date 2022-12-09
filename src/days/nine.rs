use std::collections::HashSet;

pub fn solution_easy(input: &str) -> i32 {
    let data = parse(input);
    ropes(data, 2)
}

pub fn solution_hard(input: &str) -> i32 {
    let data = parse(input);
    ropes(data, 10)
}

use Direction::*;
type Position = (i32, i32);

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn from_string(s: &str) -> Self {
        match s {
            "R" => Right,
            "L" => Left,
            "U" => Up,
            "D" => Down,
            _ => panic!(),
        }
    }
}

fn parse(_input: &str) -> Vec<(Direction, i32)> {
    _input
        .lines()
        .map(|line| {
            let ls: Vec<&str> = line.split_whitespace().collect();
            (Direction::from_string(ls[0]), ls[1].parse::<i32>().unwrap())
        })
        .collect()
}

fn step(dir: Direction, (x, y): Position) -> Position {
    match dir {
        Up => (x, y - 1),
        Down => (x, y + 1),
        Right => (x + 1, y),
        Left => (x - 1, y),
    }
}

fn is_close((x1, y1): Position, (x2, y2): Position) -> bool {
    !((x1 - x2).abs() > 1 || (y1 - y2).abs() > 1)
}

fn follow((x1, y1): Position, (x2, y2): Position) -> Position {
    if is_close((x1, y1), (x2, y2)) {
        return (x1, y1);
    }
    let mut dx = x2 - x1;
    let mut dy = y2 - y1;
    if dx != 0 {
        dx = dx / dx.abs();
    }
    if dy != 0 {
        dy = dy / dy.abs();
    }
    (x1 + dx, y1 + dy)
}

fn ropes(steps: Vec<(Direction, i32)>, size: usize) -> i32 {
    let mut seen = HashSet::new();
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
