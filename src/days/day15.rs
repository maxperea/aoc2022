pub fn solution_easy(input: &str) -> i64 {
    let sensors = parse(input);
    let y = 2_000_000;
    let mut count = 0;
    for x in -5_000_000..5_000_000 {
        if !can_have_beacon(&Position { x, y }, &sensors, true) {
            count += 1;
        }
    }
    count
}

pub fn solution_hard(input: &str) -> i64 {
    let sensors = parse(input);
    for pos in sensors.iter().flat_map(get_candidates) {
        if can_have_beacon(&pos, &sensors, false) {
            return pos.x * 4_000_000 + pos.y;
        }
    }
    0
}

#[derive(PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

struct Sensor {
    sensor: Position,
    beacon: Position,
}

fn get_candidates(Sensor { sensor, beacon }: &Sensor) -> Vec<Position> {
    let max = 4_000_000;
    let dist = 1 + manhattan(sensor, beacon);
    let min_x = 0.max(sensor.x - dist);
    let max_x = max.min(sensor.x + dist);
    let mut res = vec![];
    for x in min_x..=max_x {
        let dx = (sensor.x - x).abs();
        let dy = dist - dx;
        let y1 = sensor.y - dy;
        let y2 = sensor.y + dy;
        if y1 >= 0 {
            res.push(Position { x, y: y1 });
        }
        if y2 <= max {
            res.push(Position { x, y: y2 });
        }
    }
    res
}

fn can_have_beacon(pos: &Position, sensors: &[Sensor], easy: bool) -> bool {
    let f = |b, n| b && can_have_beacon_one(pos, n, easy);
    sensors.iter().fold(true, f)
}

fn can_have_beacon_one(pos: &Position, Sensor { sensor, beacon }: &Sensor, easy: bool) -> bool {
    (beacon == pos && easy) || manhattan(&sensor, pos) > manhattan(&sensor, &beacon)
}

fn manhattan(fst: &Position, snd: &Position) -> i64 {
    (fst.x - snd.x).abs() + (fst.y - snd.y).abs()
}

fn parse(input: &str) -> Vec<Sensor> {
    let re = regex::Regex::new(
        r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
    )
    .unwrap();
    let mut sensors = vec![];
    for cap in re.captures_iter(input) {
        sensors.push(Sensor {
            sensor: Position {
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
            },
            beacon: Position {
                x: cap[3].parse().unwrap(),
                y: cap[4].parse().unwrap(),
            },
        })
    }
    sensors
}
