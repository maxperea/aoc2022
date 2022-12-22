use std::collections::HashSet;

pub fn solution_easy(input: &str) -> i64 {
    let winds = parse(input);
    let mut winds_iter = winds.iter().cycle();
    let blocks = get_blocks();
    let mut blocks_iter = blocks.iter().cycle();
    let mut floor = Floor {
        pieces: HashSet::new(),
        height: 0,
    };

    floor.pieces.insert((0, 0));
    floor.pieces.insert((1, 0));
    floor.pieces.insert((2, 0));
    floor.pieces.insert((3, 0));
    floor.pieces.insert((4, 0));
    floor.pieces.insert((5, 0));
    floor.pieces.insert((6, 0));
    let mut count: i64 = 1;
    println!("{}", winds.len());
    while count <= 2022 {
        position(
            &mut winds_iter,
            *blocks_iter.next().as_ref().unwrap(),
            &mut floor,
        );
        count += 1;
    }

    floor.height
}

pub fn solution_hard(input: &str) -> i64 {
    let winds = parse(input);
    let mut winds_iter = winds.iter().cycle();
    let blocks = get_blocks();
    let mut blocks_iter = blocks.iter().cycle();
    let mut floor = Floor {
        pieces: HashSet::new(),
        height: 0,
    };

    floor.pieces.insert((0, 0));
    floor.pieces.insert((1, 0));
    floor.pieces.insert((2, 0));
    floor.pieces.insert((3, 0));
    floor.pieces.insert((4, 0));
    floor.pieces.insert((5, 0));
    floor.pieces.insert((6, 0));

    let mut count: i64 = 0;
    let period = winds.len() as i64 * 5 * 347;
    // let period = winds.len() as i64 * 5 * 7;

    let mut previous;
    let mut current = 0;

    let mut diff = 0;
    for p in 1..3 {
        while count < period * p {
            position(
                &mut winds_iter,
                *blocks_iter.next().as_ref().unwrap(),
                &mut floor,
            );
            count += 1;
        }

        previous = current;
        current = floor.height;
        diff = current - previous;
    }
    let mut result = floor.height;
    while count < 1_000_000_000_000 {
        count += period;
        result += diff;
    }
    count -= period;
    result -= diff;

    while count < 1_000_000_000_000 {
        position(
            &mut winds_iter,
            *blocks_iter.next().as_ref().unwrap(),
            &mut floor,
        );
        count += 1;
    }
    result + (floor.height - current)
}

fn move_block(block: &Block, dir: &Direction, floor: &Floor) -> Option<Block> {
    let mut res = vec![];
    for &Position { x, y } in block.iter() {
        let pos = match dir {
            Direction::Left => Position { x: x - 1, y },
            Direction::Right => Position { x: x + 1, y },
            Direction::Down => Position { x, y: y - 1 },
        };
        if collision(&pos, floor) {
            return None;
        } else {
            res.push(pos);
        }
    }
    Some(res)
}

fn collision(&Position { x, y }: &Position, floor: &Floor) -> bool {
    x < 0 || x >= 7 || floor.pieces.contains(&(x, y))
}

fn position<'a, I>(dir: &mut I, block_type: &BlockType, floor: &mut Floor)
where
    I: Iterator<Item = &'a Direction>,
{
    let mut block = get_block(block_type, floor.height);
    loop {
        let next_dir = dir.next();
        if let Some(b) = move_block(&block, next_dir.as_ref().unwrap(), floor) {
            block = b;
        }
        if let Some(b) = move_block(&block, &Direction::Down, floor) {
            block = b;
        } else {
            update_floor(floor, &block);
            return;
        }
    }
}

fn update_floor(floor: &mut Floor, block: &Block) {
    for &Position { x, y } in block {
        floor.pieces.insert((x, y));
        if y > floor.height {
            floor.height = y;
        }
    }
}

fn get_blocks() -> Vec<BlockType> {
    let mut blocks = vec![];
    blocks.push(BlockType::Horizontal);
    blocks.push(BlockType::Cross);
    blocks.push(BlockType::LShape);
    blocks.push(BlockType::Vertical);
    blocks.push(BlockType::Square);
    blocks
}

fn get_block(btype: &BlockType, height: i64) -> Block {
    let mut res = vec![];
    let y = height + 4;
    let x = 2;
    match *btype {
        BlockType::Horizontal => {
            res.push(Position { x, y });
            res.push(Position { x: x + 1, y });
            res.push(Position { x: x + 2, y });
            res.push(Position { x: x + 3, y });
        }
        BlockType::Cross => {
            res.push(Position { x: x + 1, y });
            res.push(Position { x: x + 1, y: y + 1 });
            res.push(Position { x: x + 2, y: y + 1 });
            res.push(Position { x, y: y + 1 });
            res.push(Position { x: x + 1, y: y + 2 });
        }
        BlockType::LShape => {
            res.push(Position { x, y });
            res.push(Position { x: x + 1, y });
            res.push(Position { x: x + 2, y });
            res.push(Position { x: x + 2, y: y + 1 });
            res.push(Position { x: x + 2, y: y + 2 });
        }
        BlockType::Vertical => {
            res.push(Position { x, y });
            res.push(Position { x, y: y + 1 });
            res.push(Position { x, y: y + 2 });
            res.push(Position { x, y: y + 3 });
        }
        BlockType::Square => {
            res.push(Position { x, y });
            res.push(Position { x: x + 1, y: y + 1 });
            res.push(Position { x: x + 1, y });
            res.push(Position { x, y: y + 1 });
        }
    }
    res
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Down,
}

#[derive(Debug)]
enum BlockType {
    Horizontal,
    Cross,
    LShape,
    Vertical,
    Square,
}

#[derive(Debug)]
struct Position {
    x: i8,
    y: i64,
}

type Block = Vec<Position>;
struct Floor {
    pieces: HashSet<(i8, i64)>,
    height: i64,
}

fn parse(input: &str) -> Vec<Direction> {
    let mut res = vec![];
    for c in input.chars() {
        if c == '<' {
            res.push(Direction::Left);
        } else if c == '>' {
            res.push(Direction::Right);
        } else {
            println!("-{}-", c);
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test() {
        let test_input = fs::read_to_string("input/test").expect("File not found.");
        assert_eq!(solution_easy(&test_input), 0);
        assert_eq!(solution_hard(&test_input), 0);
    }
}
