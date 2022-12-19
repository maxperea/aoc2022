use heapless::Vec as HVec;
use nom::bytes::complete::tag;
use nom::{character::complete::alpha1, combinator::map, multi::separated_list0, IResult};
use std::collections::{HashMap, HashSet, VecDeque};

pub fn solution_easy(input: &str) -> i64 {
    let valves = parse(input);
    bfs(&valves, neighbourhood_easy) as i64
}

pub fn solution_hard(input: &str) -> i64 {
    let valves = parse(input);
    bfs(&valves, neighbourhood_hard) as i64
}

fn bfs<F>(valves: &HVec<Valve, 64>, neighbourhood: F) -> u32
where
    F: Fn(&State, &HVec<Valve, 64>) -> HVec<State, 40>,
{
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut best = 0;
    queue.push_front(State::new());
    while let Some(state) = queue.pop_front() {
        if state.score > best {
            best = state.score;
        }
        for state in neighbourhood(&state, valves) {
            let test_state = (state.me, state.el, state.score, state.time);
            let below_score_cap = (1.2 * state.score as f32) < (best as f32);
            if visited.contains(&test_state) || below_score_cap {
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

fn neighbourhood_easy(state: &State, valves: &HVec<Valve, 64>) -> HVec<State, 40> {
    let mut new_states = HVec::new();
    if state.time >= 30 {
        return new_states;
    }
    if !is_open(state.open, state.me) && valves[state.me as usize].flow_rate > 0 {
        let _ = new_states.push(State {
            el: 0,
            me: state.me.clone(),
            open: set_open(state.open, state.me),
            score: state.score + (29 - state.time as u32) * valves[state.me as usize].flow_rate,
            time: state.time + 1,
        });
    }
    for valve in &valves[state.me as usize].connections {
        let _ = new_states.push(State {
            me: valve.clone(),
            el: 0,
            open: state.open.clone(),
            score: state.score,
            time: state.time + 1,
        });
    }
    new_states
}

fn neighbourhood_hard(state: &State, valves: &HVec<Valve, 64>) -> HVec<State, 40> {
    let mut new_states = HVec::new();
    if state.time >= 26 {
        return new_states;
    }
    let elephant_open = !is_open(state.open, state.el)
        && valves[state.el as usize].flow_rate > 0
        && state.me != state.el;
    let me_open = !is_open(state.open, state.me) && valves[state.me as usize].flow_rate > 0;

    if me_open && elephant_open {
        let mut new_valves = set_open(state.open, state.me);
        new_valves = set_open(new_valves, state.el);
        let mut new_score =
            state.score + (25 - state.time as u32) * valves[state.me as usize].flow_rate;
        new_score = new_score + (25 - state.time as u32) * valves[state.el as usize].flow_rate;
        let valve_open = State {
            me: state.me.clone(),
            el: state.el.clone(),
            open: new_valves,
            score: new_score,
            time: state.time + 1,
        };
        let _ = new_states.push(valve_open);
    } else if me_open {
        for valve in &valves[state.el as usize].connections {
            let new_valves = set_open(state.open, state.me);
            let new_score =
                state.score + (25 - state.time as u32) * valves[state.me as usize].flow_rate;
            let _ = new_states.push(State {
                me: state.me.clone(),
                el: valve.clone(),
                open: new_valves,
                score: new_score,
                time: state.time + 1,
            });
        }
    } else if elephant_open {
        for valve in &valves[state.me as usize].connections {
            let new_valves = set_open(state.open, state.el);
            let new_score =
                state.score + (25 - state.time as u32) * valves[state.el as usize].flow_rate;
            let _ = new_states.push(State {
                el: state.el.clone(),
                me: valve.clone(),
                open: new_valves,
                score: new_score,
                time: state.time + 1,
            });
        }
    }
    for valve in &valves[state.me as usize].connections {
        for e_valve in &valves[state.el as usize].connections {
            let _ = new_states.push(State {
                me: valve.clone(),
                el: e_valve.clone(),
                open: state.open.clone(),
                score: state.score,
                time: state.time + 1,
            });
        }
    }
    new_states
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct State {
    me: u8,
    el: u8,
    open: u64,
    score: u32,
    time: u8,
}

impl State {
    fn new() -> Self {
        State {
            me: 0,
            el: 0,
            open: 0,
            score: 0,
            time: 0,
        }
    }
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

fn parse(input: &str) -> HVec<Valve, 64> {
    let mut valves = HVec::new();
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
