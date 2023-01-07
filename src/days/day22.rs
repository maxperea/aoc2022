pub fn solution_easy(input: &str) -> usize {
    let map = parse_map(&input);
    let instructions = parse_instructions(&input);
    let mut state = State {
        pos: Position { x: 0, y: 0 },
        face: Direction::Right,
    };
    for instruction in instructions {
        match instruction {
            Instruction::TurnLeft => state.turn_left(),
            Instruction::TurnRight => state.turn_right(),
            Instruction::Step(n) => state.step(&map, n - 1, &state.pos.next(state.change(), &map)),
        }
        // println!(
        //     "Y: {}, X: {}, Face: {}",
        //     state.pos.y,
        //     state.pos.x,
        //     state.face.to_num()
        // );
    }
    (state.pos.y + 1) * 1000 + (state.pos.x + 1) * 4 + state.face.to_num()
}

impl State {
    fn step(&mut self, map: &Map, count: usize, next: &Position) {
        match map[next.y][next.x] {
            Tile::Wall => return,
            Tile::None => self.step(map, count, &next.next(self.change(), map)),
            Tile::Open => {
                self.pos = next.clone();
                if count == 0 {
                    return;
                }
                self.step(map, count - 1, &next.next(self.change(), map));
            }
        }
    }

    fn change(&self) -> (i32, i32) {
        match self.face {
            Direction::Down => (0, 1),
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        }
    }

    fn turn_left(&mut self) {
        self.face = self.face.prev();
    }
    fn turn_right(&mut self) {
        self.face = self.face.next();
    }
}

impl Position {
    fn next(&self, (dx, dy): (i32, i32), map: &Map) -> Position {
        Position {
            x: ((self.x as i32 + dx).rem_euclid(map[0].len() as i32)) as usize,
            y: ((self.y as i32 + dy).rem_euclid(map.len() as i32)) as usize,
        }
    }
}

pub fn solution_hard(input: &str) -> i64 {
    let _data = parse(input);
    unimplemented!()
}

fn parse(_input: &str) -> i64 {
    let _re = regex::Regex::new(r"\d+").unwrap();
    unimplemented!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Open,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    TurnLeft,
    TurnRight,
    Step(usize),
}

#[derive(rotate_enum::RotateEnum, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn to_num(&self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

struct State {
    pos: Position,
    face: Direction,
}

#[derive(Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

// type Map = [[Tile; 16]; 12];
type Map = [[Tile; 150]; 200];

fn parse_map(input: &str) -> Map {
    // let mut map = [[Tile::None; 16]; 12];
    let mut map = [[Tile::None; 150]; 200];
    let map_input = input.split("\n\n").collect::<Vec<_>>()[0];
    for (y, l) in map_input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '#' => map[y][x] = Tile::Wall,
                '.' => map[y][x] = Tile::Open,
                ' ' => map[y][x] = Tile::None,
                _ => panic!("Parse error"),
            }
        }
    }
    map
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions = vec![];
    let instruction_input = input.split("\n\n").collect::<Vec<_>>()[1];
    let mut num = String::from("");
    for c in instruction_input.chars() {
        match c {
            'R' => {
                instructions.push(Instruction::Step(
                    num.parse().expect("Tried to parse number"),
                ));
                num.clear();

                instructions.push(Instruction::TurnRight);
            }
            'L' => {
                instructions.push(Instruction::Step(
                    num.parse().expect("Tried to parse number"),
                ));
                num.clear();

                instructions.push(Instruction::TurnLeft);
            }
            '\n' => (),
            n => num += &n.to_string(),
        }
    }
    instructions.push(Instruction::Step(
        num.parse().expect("Tried to parse number"),
    ));
    instructions
}
