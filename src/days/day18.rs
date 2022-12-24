use std::collections::{HashSet, VecDeque};

pub fn solution_easy(input: &str) -> i64 {
    let data = parse(input);
    let mut grid = [[[None; 32]; 32]; 32];
    for coord in data.chunks(3) {
        match coord {
            &[x, y, z] => {
                assert!(grid[x][y][z] == None);
                grid[x][y][z] = Some(Cube::new());
                for (dx, dy, dz) in neighbourhood(&(x, y, z)) {
                    if let Some(_cube) = grid[dx][dy][dz] {
                        grid[dx][dy][dz].as_mut().unwrap().add_neighbour();
                        grid[x][y][z].as_mut().unwrap().add_neighbour();
                    }
                }
            }
            _ => panic!("Bad input"),
        }
    }
    let mut total = 0;
    for line in grid {
        for row in line {
            for maybe_cube in row {
                if let Some(cube) = maybe_cube {
                    total += cube.sides_exposed;
                }
            }
        }
    }
    total as i64
}

pub fn solution_hard(input: &str) -> i64 {
    let data = parse(input);
    let mut grid = [[[None; 32]; 32]; 32];
    for coord in data.chunks(3) {
        match coord {
            &[x, y, z] => {
                grid[x][y][z] = Some(Cube::new());
            }
            _ => panic!("Bad input"),
        }
    }

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let mut res = 0;

    let start = (0, 0, 0);
    queue.push_front(start);

    while let Some(pos) = queue.pop_front() {
        for (x, y, z) in neighbourhood(&pos) {
            if visited.contains(&(x, y, z)) {
                continue;
            }
            if grid[x][y][z] == None {
                queue.push_back((x, y, z));
                visited.insert((x, y, z));
            } else {
                res += 1;
            }
        }
    }

    res
}

fn parse(input: &str) -> Vec<usize> {
    let mut res = vec![];
    for line in input.lines() {
        for num in line.split(",") {
            res.push(num.parse().unwrap());
        }
    }
    res
}

fn neighbourhood(&(x, y, z): &Coordinate) -> Vec<Coordinate> {
    let mut res = vec![];
    if x > 0 {
        res.push((x - 1, y, z));
    }
    if x + 1 < 32 {
        res.push((x + 1, y, z));
    }
    if y > 0 {
        res.push((x, y - 1, z));
    }
    if y + 1 < 32 {
        res.push((x, y + 1, z));
    }
    if z > 0 {
        res.push((x, y, z - 1));
    }
    if z + 1 < 32 {
        res.push((x, y, z + 1));
    }
    res
}

type Coordinate = (usize, usize, usize);

#[derive(PartialEq, Eq, Clone, Copy)]
struct Cube {
    sides_exposed: u32,
}

impl Cube {
    fn new() -> Self {
        Cube { sides_exposed: 6 }
    }
    fn add_neighbour(&mut self) {
        assert!(self.sides_exposed > 0);
        self.sides_exposed -= 1;
    }
}
