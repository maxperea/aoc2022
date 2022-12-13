use std::cmp::Ordering;
use Ordering::*;
use Packet::*;

#[derive(Clone, Debug)]
enum Packet {
    Integer(i64),
    List(Vec<Packet>),
}

pub fn solution_easy(input: &str) -> i64 {
    let data = parse(input);
    let mut sum = 0;
    for (index, pair) in data.chunks(2).enumerate() {
        if compare(&pair[0], &pair[1]) == Less {
            sum += index + 1;
        }
    }
    sum as i64
}

pub fn solution_hard(input: &str) -> i64 {
    let mut data = parse(&input);
    let (_, divider1) = parse_packet("[[2]]").unwrap();
    let (_, divider2) = parse_packet("[[6]]").unwrap();
    data.push(divider1.clone());
    data.push(divider2.clone());
    data.sort_by(compare);

    let mut product = 1;

    for (index, item) in data.iter().enumerate() {
        if compare(item, &divider1) == Equal || compare(item, &divider2) == Equal {
            product *= index + 1;
        }
    }

    product as i64
}

fn compare(first: &Packet, second: &Packet) -> Ordering {
    match (first.clone(), second.clone()) {
        (Integer(x), Integer(y)) => x.cmp(&y),
        (Integer(x), List(y)) => compare(&List(vec![Integer(x)]), &List(y)),
        (List(x), Integer(y)) => compare(&List(x), &List(vec![Integer(y)])),

        (List(x), List(y)) if x.is_empty() && y.is_empty() => Equal,
        (List(x), List(_)) if x.is_empty() => Less,
        (List(_), List(y)) if y.is_empty() => Greater,

        (List(mut x), List(mut y)) => match compare(x.first().unwrap(), y.first().unwrap()) {
            Equal => compare(&List(x.split_off(1)), &List(y.split_off(1))),
            order => order,
        },
    }
}

use nom::{
    bytes::complete::tag, character::complete::i64, combinator::map, multi::separated_list0,
    sequence::delimited, IResult,
};

fn parse_integer(input: &str) -> IResult<&str, i64> {
    i64(input)
}

fn parse_list(input: &str) -> IResult<&str, Vec<Packet>> {
    use nom::Parser;

    delimited(
        tag("["),
        separated_list0(
            tag(","),
            map(parse_integer, Integer).or(map(parse_list, List)),
        ),
        tag("]"),
    )(input)
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    map(parse_list, |contents| List(contents))(input)
}

fn parse(input: &str) -> Vec<Packet> {
    let mut data = vec![];
    for line in input.lines() {
        if let Ok((_, res)) = parse_packet(line) {
            data.push(res);
        }
    }
    data
}
