use std::collections::VecDeque;

pub fn solution_easy(input: &str) -> i64 {
    let map = parse(input);
    let start_state = start(&map);
    shortest_path(&map, start_state).unwrap()
}

pub fn solution_hard(input: &str) -> i64 {
    let map = parse(input);
    let spots = a_spots(&map);
    spots
        .iter()
        .map(|start| shortest_path(&map, *start))
        .flatten()
        .min()
        .unwrap()
}

fn shortest_path(map: &Map, start: State) -> Option<i64> {
    let mut queue = VecDeque::new();
    let mut visited = [[false; 200]; 50];
    queue.push_front(start);
    while let Some(state) = queue.pop_front() {
        let y = state.0 .0 as usize;
        let x = state.0 .1 as usize;
        if map[y][x] == 'E' {
            return Some(state.1); // Solution found!
        }
        for state in neighbourhood(&map, state) {
            let y = state.0 .0 as usize;
            let x = state.0 .1 as usize;
            if visited[y][x] {
                continue;
            }
            queue.push_back(state);
            visited[y][x] = true;
        }
    }
    None
}

type Steps = i64;
type Hill = char;
type Map = Vec<Vec<Hill>>;
type Position = (i64, i64);
type State = (Position, Steps, Hill);

fn a_spots(map: &Map) -> Vec<State> {
    let mut spots = vec![];
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'a' {
                spots.push(((y as i64, x as i64), 0, 'a'));
            }
        }
    }
    spots
}

fn start(map: &Map) -> State {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                return ((y as i64, x as i64), 0, 'a');
            }
        }
    }
    panic!()
}

fn neighbourhood(map: &Map, ((y, x), moves, elevation): State) -> Vec<State> {
    let mut neighbourhood = vec![];
    let candidates = [
        ((y + 1, x), moves + 1),
        ((y - 1, x), moves + 1),
        ((y, x + 1), moves + 1),
        ((y, x - 1), moves + 1),
    ];

    for candidate in candidates {
        if let Some(new_elevation) = valid(elevation, &candidate, map) {
            neighbourhood.push((candidate.0, candidate.1, new_elevation));
        }
    }

    neighbourhood
}

fn valid(elevation: char, ((y, x), _): &(Position, Steps), map: &Map) -> Option<Hill> {
    if *y < 0 || *x < 0 || *y as usize >= map.len() || *x as usize >= map[0].len() {
        return None;
    }
    let target_elevation = map[*y as usize][*x as usize];
    if target_elevation == 'E' && (elevation == 'y' || elevation == 'z') {
        return Some(target_elevation);
    }
    if target_elevation == 'E' || target_elevation == 'S' {
        return None;
    }
    if target_elevation.to_digit(36).unwrap() as i64 - elevation.to_digit(36).unwrap() as i64 > 1 {
        return None;
    }
    Some(target_elevation)
}

fn parse(input: &str) -> Map {
    input.lines().map(|line| line.chars().collect()).collect()
}
