use nom::bytes::complete::tag;
use nom::{character::complete::alpha1, combinator::map, multi::separated_list0, IResult};
use std::collections::{HashMap, HashSet, VecDeque};
// use heapless::Vec;

pub fn solution_easy(input: &str) -> i64 {
    let valves = parse(input);
    bfs(&valves, 30, neighbourhood_easy) as i64
}

pub fn solution_hard(input: &str) -> i64 {
    let valves = parse(input);
    bfs(&valves, 26, neighbourhood_hard) as i64
}

fn bfs<F>(valves: &heapless::Vec<Valve, 64>, end: u8, neighbourhood: F) -> u32
where
    F: Fn(&State, &heapless::Vec<Valve, 64>) -> heapless::Vec<State, 40>,
{
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut best = 0;
    let start = State {
        pos: 0,
        e_pos: 0,
        valves: 0,
        score: 0,
        steps: 0,
    };
    queue.push_front(start);
    while let Some(state) = queue.pop_front() {
        if state.steps == end && state.score > best {
            best = state.score;
        }
        for state in neighbourhood(&state, valves) {
            let test_state = (state.pos, state.e_pos, state.score, state.steps);
            if visited.contains(&test_state) {
                continue;
            }
            queue.push_back(state.clone());
            visited.insert(test_state);
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

fn neighbourhood_easy(
    state: &State,
    valves: &heapless::Vec<Valve, 64>,
) -> heapless::Vec<State, 40> {
    let mut new_states = heapless::Vec::new();
    if state.steps >= 30 {
        return new_states;
    }
    if !is_open(state.valves, state.pos) && valves[state.pos as usize].flow_rate > 0 {
        let new_valves = set_open(state.valves, state.pos);
        let new_score =
            state.score + (29 - state.steps as u32) * valves[state.pos as usize].flow_rate;
        let valve_open = State {
            e_pos: 0,
            pos: state.pos.clone(),
            valves: new_valves,
            score: new_score,
            steps: state.steps + 1,
        };
        let _ = new_states.push(valve_open);
    }
    for valve in &valves[state.pos as usize].connections {
        let _ = new_states.push(State {
            pos: valve.clone(),
            e_pos: 0,
            valves: state.valves.clone(),
            score: state.score,
            steps: state.steps + 1,
        });
    }
    new_states
}

fn neighbourhood_hard(
    state: &State,
    valves: &heapless::Vec<Valve, 64>,
) -> heapless::Vec<State, 40> {
    let mut new_states = heapless::Vec::new();
    if state.steps >= 26 {
        return new_states;
    }
    let elephant_open = !is_open(state.valves, state.e_pos)
        && valves[state.e_pos as usize].flow_rate > 0
        && state.pos != state.e_pos;
    let me_open = !is_open(state.valves, state.pos) && valves[state.pos as usize].flow_rate > 0;

    if me_open && elephant_open {
        let mut new_valves = set_open(state.valves, state.pos);
        new_valves = set_open(new_valves, state.e_pos);
        let mut new_score =
            state.score + (25 - state.steps as u32) * valves[state.pos as usize].flow_rate;
        new_score = new_score + (25 - state.steps as u32) * valves[state.e_pos as usize].flow_rate;
        let valve_open = State {
            pos: state.pos.clone(),
            e_pos: state.e_pos.clone(),
            valves: new_valves,
            score: new_score,
            steps: state.steps + 1,
        };
        let _ = new_states.push(valve_open);
    } else if me_open {
        for valve in &valves[state.e_pos as usize].connections {
            let new_valves = set_open(state.valves, state.pos);
            let new_score =
                state.score + (25 - state.steps as u32) * valves[state.pos as usize].flow_rate;
            let _ = new_states.push(State {
                pos: state.pos.clone(),
                e_pos: valve.clone(),
                valves: new_valves,
                score: new_score,
                steps: state.steps + 1,
            });
        }
    } else if elephant_open {
        for valve in &valves[state.pos as usize].connections {
            let new_valves = set_open(state.valves, state.e_pos);
            let new_score =
                state.score + (25 - state.steps as u32) * valves[state.e_pos as usize].flow_rate;
            let _ = new_states.push(State {
                e_pos: state.e_pos.clone(),
                pos: valve.clone(),
                valves: new_valves,
                score: new_score,
                steps: state.steps + 1,
            });
        }
    }
    for valve in &valves[state.pos as usize].connections {
        for e_valve in &valves[state.e_pos as usize].connections {
            let _ = new_states.push(State {
                pos: valve.clone(),
                e_pos: e_valve.clone(),
                valves: state.valves.clone(),
                score: state.score,
                steps: state.steps + 1,
            });
        }
    }
    new_states
}

fn get_map(valves: &heapless::Vec<Valve, 64>) -> Map {
    let mut flow_rate_valves = vec![];
    let mut valve_map = HashMap::new();
    for (i, valve) in valves.iter().enumerate() {
        if valve.flow_rate > 0 {
            flow_rate_valves.push(i);
        }
    }

    for fr_valve in flow_rate_valves {
        let mut others = vec![];
        for other in &valves[fr_valve].connections {
            others.push(get_distance(fr_valve as u8, *other, valves));
        }
        valve_map.insert(fr_valve as u8, others);
    }
    valve_map
}

fn get_distance(pos: u8, other: u8, valves: &heapless::Vec<Valve, 64>) -> u8 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let start = (pos, 0);
    queue.push_front(start);
    while let Some((test, dist)) = queue.pop_front() {
        if test == other {
            return dist;
        }
        for next in &valves[test as usize].connections {
            if visited.contains(&next) {
                continue;
            }
            queue.push_back((*next, dist + 1));
            visited.insert(next);
        }
    }
    panic!()
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct State {
    pos: u8,
    e_pos: u8,
    valves: u64,
    score: u32,
    steps: u8,
}

struct Node {
    distance: u8,
    pos: u8,
}

type Map = HashMap<u8, Vec<u8>>;

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
