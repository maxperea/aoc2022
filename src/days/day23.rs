use std::collections::{HashMap, HashSet};

pub fn solution_easy(input: &str) -> i64 {
    let mut elfs = parse(input);
    let mut dir = Direction::North;
    for _ in 0..10 {
        elfs = do_round(&elfs, &dir).unwrap();
        dir = dir.next();
    }
    let (x1, x2, y1, y2) = get_bounding_box(&elfs);
    (y2 - y1 + 1) * (x2 - x1 + 1) - elfs.iter().len() as i64
}

pub fn solution_hard(input: &str) -> i64 {
    let mut elfs = parse(input);
    let mut dir = Direction::North;
    let mut rounds = 1;
    while let Some(next) = do_round(&elfs, &dir) {
        elfs = next;
        dir = dir.next();
        rounds += 1;
    }
    rounds
}

fn do_round(elfs: &Elfs, start_dir: &Direction) -> Option<Elfs> {
    let mut does_nothing = HashSet::new();
    let mut new_positions = HashMap::new();
    for elf in elfs.iter() {
        if let Some(new_pos) = elf.get_next_valid_pos(start_dir, elfs) {
            if new_positions.contains_key(&new_pos) {
                does_nothing.insert(elf.clone());
                does_nothing.insert(new_positions.remove_entry(&new_pos).unwrap().1);
            } else {
                new_positions.insert(new_pos, *elf);
            }
        } else {
            does_nothing.insert(elf.clone());
        }
    }
    if new_positions.is_empty() {
        return None;
    }
    for elf in new_positions.keys() {
        does_nothing.insert(elf.clone());
    }
    Some(does_nothing)
}

fn get_bounding_box(elfs: &Elfs) -> (i64, i64, i64, i64) {
    let elf_to_pair = |elf: &Elf| (elf.x, elf.y);
    let (xs, ys): (Vec<_>, Vec<_>) = elfs.iter().map(elf_to_pair).unzip();
    (
        *xs.iter().min().unwrap(),
        *xs.iter().max().unwrap(),
        *ys.iter().min().unwrap(),
        *ys.iter().max().unwrap(),
    )
}

fn valid_direction(elfs: &Elfs, positions: Vec<Elf>) -> bool {
    positions
        .iter()
        .fold(true, |acc, next| acc && !elfs.contains(&next))
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Elf {
    x: i64,
    y: i64,
}

#[derive(rotate_enum::RotateEnum, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

use Direction::*;
type Elfs = HashSet<Elf>;

impl Elf {
    fn positions(&self, dir: Direction) -> Vec<Self> {
        match dir {
            North => vec![(-1, -1), (0, -1), (1, -1)],
            South => vec![(-1, 1), (0, 1), (1, 1)],
            West => vec![(-1, -1), (-1, 0), (-1, 1)],
            East => vec![(1, -1), (1, 0), (1, 1)],
        }
        .iter()
        .map(|(dx, dy)| Elf {
            x: self.x + dx,
            y: self.y + dy,
        })
        .collect()
    }

    fn step(&self, dir: &Direction) -> Self {
        let (dx, dy) = match dir {
            North => (0, -1),
            South => (0, 1),
            West => (-1, 0),
            East => (1, 0),
        };
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    fn get_next_valid_pos(&self, start_dir: &Direction, elfs: &Elfs) -> Option<Elf> {
        if [North, South, West, East].iter().fold(true, |acc, next| {
            acc && valid_direction(elfs, self.positions(*next))
        }) {
            return None;
        }
        let mut dir = start_dir.clone();
        for _ in 0..4 {
            if valid_direction(elfs, self.positions(dir)) {
                return Some(self.step(&dir));
            }
            dir = dir.next();
        }
        None
    }
}

fn parse(input: &str) -> Elfs {
    let mut elfs: Elfs = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elfs.insert(Elf {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }
    elfs
}
