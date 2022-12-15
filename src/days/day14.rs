use itertools::Itertools;

pub fn solution_easy(input: &str) -> i64 {
    let (max, mut cave) = get_cave(&input);
    let mut count = 0;
    while let Some((x, y)) = drop_sand(&mut cave, max) {
        if y >= max {
            break;
        }
        cave[y][x] = true;
        count += 1;
    }
    count
}

pub fn solution_hard(input: &str) -> i64 {
    let (max, mut cave) = get_cave(&input);
    let mut count = 0;
    while let Some((x, y)) = drop_sand(&mut cave, max + 1) {
        cave[y][x] = true;
        count += 1;
        if (x, y) == (500, 0) {
            break;
        }
    }
    count
}

type Cave = Vec<Vec<bool>>;
type Path = Vec<Pos>;
type Pos = (usize, usize);

fn get_cave(input: &str) -> (usize, Cave) {
    let mut cave = vec![vec![false; 1000]; 300];

    let paths = parse(input);
    let max = y_max(&paths);

    for path in paths {
        set_path(&path, &mut cave);
    }
    (max, cave)
}

fn y_max(path: &Vec<Path>) -> usize {
    path.iter().flatten().map(|(_, y)| y).max().unwrap().clone()
}

fn drop_sand(cave: &mut Cave, limit: usize) -> Option<Pos> {
    let (mut x, mut y) = (500, 0);
    loop {
        if y == limit {
            return Some((x, y));
        } else if !cave[y + 1][x] {
            y += 1;
        } else if !cave[y + 1][x - 1] {
            y += 1;
            x -= 1;
        } else if !cave[y + 1][x + 1] {
            y += 1;
            x += 1;
        } else {
            return Some((x, y));
        }
    }
}

fn set_path(path: &Path, cave: &mut Cave) {
    for (prev, next) in path.iter().tuple_windows() {
        if prev.0 == next.0 {
            let iter_y = prev.1.min(next.1)..=prev.1.max(next.1);

            let x = prev.0;
            for y in iter_y {
                cave[y][x] = true;
            }
        } else {
            let iter_x = prev.0.min(next.0)..=prev.0.max(next.0);

            let y = prev.1;
            for x in iter_x {
                cave[y][x] = true;
            }
        }
    }
}

use nom::{
    bytes::complete::tag, character::complete::u64, combinator::map, multi::separated_list0,
    sequence::separated_pair, IResult,
};

fn parse_line(input: &str) -> IResult<&str, Path> {
    separated_list0(tag(" -> "), parse_pair)(input)
}

fn parse_pair(input: &str) -> IResult<&str, Pos> {
    let to_usize = |x| x as usize;
    separated_pair(map(u64, to_usize), tag(","), map(u64, to_usize))(input)
}

fn parse(input: &str) -> Vec<Path> {
    let (_, right): (Vec<_>, Vec<_>) = input.lines().map(parse_line).flatten().unzip();
    right
}
