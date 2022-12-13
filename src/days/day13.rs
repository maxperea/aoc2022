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
    compare_aux(Some(first.clone()), Some(second.clone()))
}

fn compare_aux(first: Option<Packet>, second: Option<Packet>) -> Ordering {
    match (first, second) {
        (Some(Integer(x)), Some(Integer(y))) => x.cmp(&y),

        (None, Some(_)) => Less,
        (Some(_), None) => Greater,

        (Some(Integer(x)), Some(List(y))) => {
            compare_aux(Some(List(vec![Integer(x)])), Some(List(y)))
        }

        (Some(List(x)), Some(Integer(y))) => {
            compare_aux(Some(List(x)), Some(List(vec![Integer(y)])))
        }

        (Some(List(x)), Some(List(y))) if x.is_empty() && y.is_empty() => Equal,
        (Some(List(x)), Some(List(_))) if x.is_empty() => Less,
        (Some(List(_)), Some(List(y))) if y.is_empty() => Greater,

        (Some(List(mut x)), Some(List(mut y))) => {
            match compare_aux(x.first().cloned(), y.first().cloned()) {
                Equal => compare_aux(Some(List(x.split_off(1))), Some(List(y.split_off(1)))),
                res => res,
            }
        }

        (None, None) => Equal,
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
            map(parse_integer, Packet::Integer).or(map(parse_list, Packet::List)),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test2() {
        let (_, less) = parse_packet("[[4,4],4,4]").unwrap();
        let (_, greater) = parse_packet("[[4,4],4,4,4]").unwrap();
        assert_eq!(compare(&greater, &less), Greater);
    }

    #[test]
    fn test() {
        assert_eq!(compare(&Integer(2), &Integer(3)), Less);
        assert_eq!(
            compare(
                &List(vec![
                    Integer(1),
                    Integer(1),
                    Integer(3),
                    Integer(1),
                    Integer(1)
                ]),
                &List(vec![
                    Integer(1),
                    Integer(1),
                    Integer(5),
                    Integer(1),
                    Integer(1)
                ])
            ),
            Less
        );
        assert_eq!(
            compare(
                &List(vec![Integer(9),]),
                &List(vec![List(vec![Integer(8), Integer(7), Integer(6),]),])
            ),
            Greater
        );
    }
}
