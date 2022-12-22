use std::collections::{HashMap, VecDeque};
use BlockType::*;
use Direction::*;

pub fn solution_easy(input: &str) -> u64 {
    let winds = parse(input);
    let mut winds_iter = winds.iter().cycle().enumerate();
    let blocks = get_blocks();
    let mut blocks_iter = blocks.iter().cycle();
    let mut floor = Floor::new();
    let mut count = 0;
    while count < 2022 {
        drop_block(&mut winds_iter, blocks_iter.next().unwrap(), &mut floor);
        count += 1;
    }

    floor.height
}

pub fn solution_hard(input: &str) -> u64 {
    let jets = parse(input);
    let mut jets_iter = jets.iter().cycle().enumerate();

    let blocks = get_blocks();
    let mut blocks_iter = blocks.iter().cycle();

    let mut floor = Floor::new();

    let mut seen = HashMap::new();
    let mut count = 0;

    let (start, start_height) = loop {
        count += 1;
        let jet =
            drop_block(&mut jets_iter, blocks_iter.next().unwrap(), &mut floor) % jets.len() as u32;
        if seen.contains_key(&(jet, floor.pieces)) {
            break seen.get(&(jet, floor.pieces)).unwrap();
        } else {
            seen.insert((jet, floor.pieces.clone()), (count, floor.height));
        }
    };

    let period = count - start;
    let period_height = floor.height - start_height;

    let remaining = 1_000_000_000_000 - count;
    let result = (remaining / period) * period_height + floor.height;
    let last_few = remaining % period;

    let before = floor.height;
    for _ in 0..last_few {
        drop_block(&mut jets_iter, blocks_iter.next().unwrap(), &mut floor);
    }

    result + (floor.height - before)
}

fn move_block(block: &Block, dir: &Direction, floor: &Floor) -> Option<Block> {
    let mut new = block.clone();
    match dir {
        Left => {
            if block[0] != 0 {
                return None;
            }
            new.pop_front();
            new.push_back(0);
        }
        Right => {
            if block[6] != 0 {
                return None;
            }
            new.pop_back();
            new.push_front(0);
        }
        Down => {
            for i in 0..7 {
                new[i] <<= 1;
            }
        }
    };
    for i in 0..7 {
        if (new[i] & floor.pieces[i]) > 0 {
            return None;
        }
    }
    Some(new)
}

fn drop_block<'a, I>(dir: &mut I, block_type: &BlockType, floor: &mut Floor) -> u32
where
    I: Iterator<Item = (usize, &'a Direction)>,
{
    let mut block = get_block(block_type);
    loop {
        let next = dir.next();
        let (i, next_dir) = next.unwrap();
        if let Some(b) = move_block(&block, next_dir, floor) {
            block = b;
        }
        if let Some(b) = move_block(&block, &Down, floor) {
            block = b;
        } else {
            update_floor(floor, &block);
            return i as u32;
        }
    }
}

fn update_floor(floor: &mut Floor, block: &Block) {
    for i in 0..7 {
        floor.pieces[i] |= block[i];
    }
    let space = floor.pieces.iter().map(|x| x.trailing_zeros()).min();
    let space_diff = 7 - space.unwrap();

    if space_diff > 0 {
        floor.height += space_diff as u64;
        for i in 0..7 {
            floor.pieces[i] <<= space_diff;
        }
    }
}

fn get_blocks() -> Vec<BlockType> {
    vec![Horizontal, Cross, LShape, Vertical, Square]
}

fn get_block(block_type: &BlockType) -> Block {
    match block_type {
        Horizontal => VecDeque::from([0, 0, 0b1000, 0b1000, 0b1000, 0b1000, 0]),
        Cross => VecDeque::from([0, 0, 0b0100, 0b1110, 0b0100, 0, 0]),
        LShape => VecDeque::from([0, 0, 0b1000, 0b1000, 0b1110, 0, 0]),
        Vertical => VecDeque::from([0, 0, 0b1111, 0, 0, 0, 0]),
        Square => VecDeque::from([0, 0, 0b1100, 0b1100, 0, 0, 0]),
    }
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

type Block = VecDeque<u64>;

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct Floor {
    pieces: [u64; 7],
    height: u64,
}

impl Floor {
    fn new() -> Self {
        Floor {
            pieces: [128, 128, 128, 128, 128, 128, 128],
            height: 0,
        }
    }
}

fn parse(input: &str) -> Vec<Direction> {
    let mut res = vec![];
    for c in input.chars() {
        if c == '<' {
            res.push(Direction::Left);
        } else if c == '>' {
            res.push(Direction::Right);
        }
    }
    res
}
