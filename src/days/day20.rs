pub fn solution_easy(input: &str) -> i64 {
    let mut num_list = parse(input);
    mix(&mut num_list);
    get_grove(num_list)
}

pub fn solution_hard(input: &str) -> i64 {
    let mut num_list = parse(input);
    let dec_key = 811589153;
    num_list.iter_mut().for_each(|p| p.1 = p.1 * dec_key);
    for _ in 0..10 {
        mix(&mut num_list);
    }
    get_grove(num_list)
}

fn get_grove(num_list: Vec<(usize, i64)>) -> i64 {
    let size = num_list.len();
    let zero_pos = num_list.iter().position(|&(_, num)| num == 0).unwrap();
    [1000, 2000, 3000]
        .map(|offset| num_list[(zero_pos + offset) % size].1)
        .iter()
        .sum()
}

fn mix(num_list: &mut Vec<(usize, i64)>) {
    let size = num_list.len();
    for i in 0..size {
        let pos = num_list.iter().position(|&(j, _)| i == j).unwrap();
        let num = num_list.remove(pos);
        let new_index = (pos as i64 + num.1).rem_euclid(size as i64 - 1);
        num_list.insert(new_index as usize, num);
    }
}

fn parse(input: &str) -> Vec<(usize, i64)> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| (i, line.parse().unwrap()))
        .collect()
}
