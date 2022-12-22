pub fn solution_easy(input: &str) -> i64 {
    let winds = parse(input);
    let mut winds_iter = winds.iter().cycle();
    let blocks = get_blocks();
    let mut blocks_iter = blocks.iter().cycle();
    let mut floor = [0, 0, 0, 0, 0, 0, 0];
    let mut count = 0;
    while count <= 2022 {
        position(
            &mut winds_iter,
            *blocks_iter.next().as_ref().unwrap(),
            &mut floor,
        );
        count += 1;
    }

    *floor.iter().max().unwrap() as i64
}

pub fn solution_hard(input: &str) -> i64 {
    let data = parse(input);
    unimplemented!()
}

fn position<'a, I>(dir: &mut I, block: &Block, floor: &mut Floor)
where
    I: Iterator<Item = &'a Direction>,
{
    println!("{:?}", dir.next().as_ref().unwrap());
    print_block(block);
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

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

type Block = [[bool; 4]; 4];
type Floor = [u64; 7];

fn print_block(block: &Block) {
    for row in block {
        for b in row {
            if *b {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn get_blocks() -> Vec<Block> {
    let mut res = vec![];
    res.push([
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
        [true, true, true, true],
    ]);
    res.push([
        [false, false, false, false],
        [false, true, false, false],
        [true, true, true, false],
        [false, true, false, false],
    ]);
    res.push([
        [false, false, false, false],
        [false, false, true, false],
        [false, false, true, false],
        [true, true, true, false],
    ]);
    res.push([
        [true, false, false, false],
        [true, false, false, false],
        [true, false, false, false],
        [true, false, false, false],
    ]);
    res.push([
        [false, false, false, false],
        [false, false, false, false],
        [true, true, false, false],
        [true, true, false, false],
    ]);
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
