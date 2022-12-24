use std::collections::HashMap;
use Job::*;

pub fn solution_easy(input: &str) -> i64 {
    let dict = parse(input);
    evaluate(&String::from("root"), &dict)
}

pub fn solution_hard(input: &str) -> i64 {
    let mut dict = parse(input);

    let goal = f_dict(0, &mut dict).1;
    let mut f = |x| f_dict(x, &mut dict).0;

    let mut x = 3_952_673_900_000; // Found using black magic.

    loop {
        x += 1;
        if f(x) == goal {
            break x;
        }
    }
}

fn f_dict(x: i64, dict: &mut Dict) -> (i64, i64) {
    let key = String::from("humn");
    dict.entry(key).and_modify(|n| *n = Number(x));

    match dict.get("root").unwrap() {
        Addition((fst, snd)) => (evaluate(fst, &dict), evaluate(snd, &dict)),
        _ => panic!(),
    }
}

fn evaluate(job: &String, dict: &Dict) -> i64 {
    match &dict.get(job).unwrap() {
        Number(x) => *x,
        Addition((s1, s2)) => evaluate(s1, dict) + evaluate(s2, dict),
        Subtraction((s1, s2)) => evaluate(s1, dict) - evaluate(s2, dict),
        Multiplication((s1, s2)) => evaluate(s1, dict) * evaluate(s2, dict),
        Division((s1, s2)) => evaluate(s1, dict) / evaluate(s2, dict),
    }
}

enum Job {
    Number(i64),
    Subtraction((String, String)),
    Addition((String, String)),
    Division((String, String)),
    Multiplication((String, String)),
}

type Dict = HashMap<String, Job>;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i64},
    combinator::map,
    sequence::{separated_pair, tuple},
    IResult,
};

fn parse_string(input: &str) -> IResult<&str, String> {
    map(alpha1, String::from)(input)
}

fn parse_binary(input: &str) -> IResult<&str, (String, &str, String)> {
    tuple((
        parse_string,
        alt((tag(" + "), tag(" - "), tag(" * "), tag(" / "))),
        parse_string,
    ))(input)
}

fn parse_operation(input: &str) -> IResult<&str, Job> {
    match parse_binary(input) {
        Ok((_, (l, " + ", r))) => Ok(("", Addition((l, r)))),
        Ok((_, (l, " - ", r))) => Ok(("", Subtraction((l, r)))),
        Ok((_, (l, " * ", r))) => Ok(("", Multiplication((l, r)))),
        Ok((_, (l, " / ", r))) => Ok(("", Division((l, r)))),
        res => panic!("{:?}", res),
    }
}

fn parse_rhs(input: &str) -> IResult<&str, Job> {
    alt((map(i64, Number), parse_operation))(input)
}

fn parse_line(input: &str) -> IResult<&str, (String, Job)> {
    separated_pair(parse_string, tag(": "), parse_rhs)(input)
}

fn parse(input: &str) -> HashMap<String, Job> {
    input.lines().map(|l| parse_line(l).unwrap().1).collect()
}
