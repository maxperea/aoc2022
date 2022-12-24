use std::collections::HashMap;
use Job::*;

pub fn solution_easy(input: &str) -> i64 {
    let dict = parse(input);
    evaluate(dict.get("root").unwrap(), &dict)
}

pub fn solution_hard(input: &str) -> i64 {
    let mut dict = parse(input);
    let goal = f_dict(0, &mut dict).1;
    let mut f = |x| f_dict(x, &mut dict).0;

    let mut x = 3952673900000;
    println!("Something: {}", f(x));
    println!("Goal     : {}", goal);

    let mut test = f(x);
    while test != goal {
        x += 1;
        test = f(x);
    }
    x
}

fn f_dict(x: i64, dict: &mut Dict) -> (i64, i64) {
    let human_key = String::from("humn");
    let mut first = String::from("");
    let mut second = String::from("");
    if let Addition((fst, snd)) = dict.get("root").unwrap() {
        first = fst.clone();
        second = snd.clone();
    };

    dict.entry(human_key.clone()).and_modify(|n| *n = Number(x));

    (
        evaluate(dict.get(&first).unwrap(), &dict),
        evaluate(dict.get(&second).unwrap(), &dict),
    )
}

enum Job {
    Number(i64),
    Subtraction((String, String)),
    Addition((String, String)),
    Division((String, String)),
    Multiplication((String, String)),
}

type Dict = HashMap<String, Job>;

fn evaluate(job: &Job, dict: &Dict) -> i64 {
    match &job {
        Number(x) => *x,
        Addition((s1, s2)) => {
            evaluate(dict.get(s1).unwrap(), dict) + evaluate(dict.get(s2).unwrap(), dict)
        }
        Subtraction((s1, s2)) => {
            evaluate(dict.get(s1).unwrap(), dict) - evaluate(dict.get(s2).unwrap(), dict)
        }
        Multiplication((s1, s2)) => {
            evaluate(dict.get(s1).unwrap(), dict) * evaluate(dict.get(s2).unwrap(), dict)
        }
        Division((s1, s2)) => {
            evaluate(dict.get(s1).unwrap(), dict) / evaluate(dict.get(s2).unwrap(), dict)
        }
    }
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i64},
    combinator::map,
    sequence::separated_pair,
    IResult,
};

fn parse_string(input: &str) -> IResult<&str, String> {
    map(alpha1, String::from)(input)
}

fn parse_line(input: &str) -> IResult<&str, (String, Job)> {
    separated_pair(parse_string, tag(": "), parse_rhs)(input)
}

fn parse_rhs(input: &str) -> IResult<&str, Job> {
    alt((map(i64, Number), parse_operation))(input)
}

fn parse_operation(input: &str) -> IResult<&str, Job> {
    alt((parse_add, parse_sub, parse_mul, parse_div))(input)
}

fn parse_add(input: &str) -> IResult<&str, Job> {
    map(
        separated_pair(parse_string, tag(" + "), parse_string),
        Addition,
    )(input)
}
fn parse_sub(input: &str) -> IResult<&str, Job> {
    map(
        separated_pair(parse_string, tag(" - "), parse_string),
        Subtraction,
    )(input)
}
fn parse_mul(input: &str) -> IResult<&str, Job> {
    map(
        separated_pair(parse_string, tag(" * "), parse_string),
        Multiplication,
    )(input)
}
fn parse_div(input: &str) -> IResult<&str, Job> {
    map(
        separated_pair(parse_string, tag(" / "), parse_string),
        Division,
    )(input)
}

fn parse(input: &str) -> HashMap<String, Job> {
    let (_, right): (Vec<_>, HashMap<_, _>) = input.lines().map(parse_line).flatten().unzip();
    right
}
