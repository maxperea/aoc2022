pub fn solution_easy(input: &str) -> i32 {
    let data = parse(input);
    let mut reg_x = 1;
    let mut cycle = 0;
    let mut ans = 0;
    for entry in data {
        cycle += 1;
        ans += check_cycle(cycle, reg_x);
        match entry {
            Add(x) => {
                cycle += 1;
                ans += check_cycle(cycle, reg_x);
                reg_x += x;
            }
            Noop => {}
        }
    }
    ans
}

pub fn solution_hard(input: &str) -> i32 {
    let data = parse(input);
    let mut reg_x = 1;
    let mut cycle = 0;
    for entry in data {
        draw_screen(cycle, reg_x);
        cycle += 1;
        match entry {
            Add(x) => {
                draw_screen(cycle, reg_x);
                cycle += 1;
                reg_x += x;
            }
            Noop => {}
        }
    }
    0
}

fn check_cycle(cycle: i32, signal: i32) -> i32 {
    match (cycle + 20) % 40 {
        0 => cycle * signal,
        _ => 0,
    }
}

fn draw_screen(cycle: i32, reg_x: i32) {
    let crt_pos = cycle % 40;
    match (crt_pos - reg_x).abs() <= 1 {
        true => print!("#"),
        false => print!("."),
    }
    if crt_pos == 39 {
        println!();
    }
}

enum Signal {
    Add(i32),
    Noop,
}

use Signal::*;

fn parse(input: &str) -> Vec<Signal> {
    let mut data = vec![];
    for line in input.lines() {
        let slice: Vec<&str> = line.split(" ").collect();
        match slice[..] {
            ["addx", arg] => data.push(Add(arg.parse().unwrap())),
            ["noop"] => data.push(Noop),
            _ => panic!("Parse error!"),
        }
    }
    data
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test() {
        let test_input = fs::read_to_string("input/test").expect("File not found.");
        assert_eq!(solution_easy(&test_input), 13140);
        assert_eq!(solution_hard(&test_input), 0);
    }
}
