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
    let candidates = get_all_candidates(&sensors);
    for pos in candidates {
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
    let mut res = vec![];
    let dist = man_dist(sensor, beacon) + 1;
    let min_x = 0.max(sensor.x - dist);
    let max_x = max.min(sensor.x + dist);
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

fn get_all_candidates(sensors: &Vec<Sensor>) -> Vec<Position> {
    sensors.iter().map(get_candidates).flatten().collect()
}

fn can_have_beacon(pos: &Position, sensors: &Vec<Sensor>, easy: bool) -> bool {
    let f = |b, n| b && can_have_beacon_aux(pos, n, easy);
    sensors.iter().fold(true, f)
}

fn can_have_beacon_aux(pos: &Position, Sensor { sensor, beacon }: &Sensor, easy: bool) -> bool {
    (beacon == pos && easy) || man_dist(&sensor, pos) > man_dist(&sensor, &beacon)
}

fn man_dist(fst: &Position, snd: &Position) -> i64 {
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
