pub fn solution_easy(input: &str) -> i64 {
    let valves = parse(input);
    bfs(&valves) as i64
}

pub fn solution_hard(input: &str) -> i64 {
    let data = parse(input);
    unimplemented!()
}

fn bfs(valves: &heapless::Vec<Valve, 64>) -> u32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let start = State {
        pos: 0,
        open_valves: 0,
        score: 0,
        steps: 0,
    };
    let mut best = 0;
    queue.push_front(start);
    while let Some(state) = queue.pop_front() {
        if state.steps == 30 && state.score > best {
            best = state.score;
            println!("New best:{}", best);
        }
        for state in neighbourhood(&state, valves) {
            if visited.contains(&state) {
                continue;
            }
            queue.push_back(state.clone());
            visited.insert(state);
        }
    }
    best
}

fn set_open(valves: u64, pos: u8) -> u64 {
    valves | (1 << pos)
}

fn is_open(valves: u64, pos: u8) -> bool {
    valves & (1 << pos) > 0
}

fn neighbourhood(state: &State, valves: &heapless::Vec<Valve, 64>) -> heapless::Vec<State, 6> {
    let mut new_states = heapless::Vec::new();
    if state.steps >= 30 {
        return new_states;
    }
    if !is_open(state.open_valves, state.pos) && valves[state.pos as usize].flow_rate > 0 {
        let new_valves = set_open(state.open_valves, state.pos);
        let new_score =
            state.score + (29 - state.steps as u32) * valves[state.pos as usize].flow_rate;
        let valve_open = State {
            pos: state.pos.clone(),
            open_valves: new_valves,
            score: new_score,
            steps: state.steps + 1,
        };
        let _ = new_states.push(valve_open);
    }
    for valve in &valves[state.pos as usize].connections {
        let _ = new_states.push(State {
            pos: valve.clone(),
            open_valves: state.open_valves.clone(),
            score: state.score,
            steps: state.steps + 1,
        });
    }
    new_states
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct State {
    pos: u8,
    open_valves: u64,
    score: u32,
    steps: u8,
}

struct Valve {
    flow_rate: u32,
    connections: Vec<u8>,
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct FirstValve {
    name: String,
    flow_rate: u32,
    connections: Vec<String>,
}

use std::collections::{HashMap, HashSet, VecDeque};

use nom::bytes::complete::tag;
use nom::{character::complete::alpha1, combinator::map, multi::separated_list0, IResult};

fn parse(input: &str) -> heapless::Vec<Valve, 64> {
    let mut valves = heapless::Vec::new();
    let mut firstvalves = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let first_valve = parse_line(line);

        firstvalves.insert(first_valve.name, i);
    }

    for line in input.lines() {
        let first_valve = parse_line(line);
        let mut connections = vec![];
        let index = firstvalves.get(&first_valve.name).unwrap();
        for con in first_valve.connections {
            connections.push(*firstvalves.get(&con).unwrap() as u8);
        }
        let _ = valves.insert(
            *index as usize,
            Valve {
                flow_rate: first_valve.flow_rate,
                connections,
            },
        );
    }

    valves
}

fn parse_line(input: &str) -> FirstValve {
    let res: Vec<_> = input.split(";").collect();
    let (_, connections) = parse_list(res[2]).unwrap();
    FirstValve {
        name: String::from(res[0]),
        flow_rate: res[1].parse().unwrap(),
        connections,
    }
}

fn parse_list(input: &str) -> IResult<&str, Vec<String>> {
    separated_list0(tag(", "), map(alpha1, |s: &str| String::from(s)))(input)
}
