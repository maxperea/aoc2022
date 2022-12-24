use natural_derive::*;
use std::collections::{HashSet, VecDeque};

pub fn solution_easy(input: &str) -> i64 {
    let data = parse(input);
    let mut result = 0;
    for blueprint in data {
        result += get_geodes(&blueprint, 24) * blueprint.index;
    }
    result as i64
}

pub fn solution_hard(input: &str) -> i64 {
    let data = parse(input);
    let mut result = 1;
    for blueprint in &data[0..3] {
        result *= get_geodes(&blueprint, 32);
    }
    result as i64
}

fn get_geodes(blueprint: &Blueprint, max_time: u8) -> u32 {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let mut max_geodes: u32 = 0;
    let mut max_geode_robots = 0;

    let start = State::new();
    queue.push_front(start);

    while let Some(state) = queue.pop_front() {
        for next in state.branch(blueprint) {
            if next.geode_robots.0 < max_geode_robots
                || next.time_spent > max_time
                || visited.contains(&next)
            {
                continue;
            }
            max_geode_robots = max_geode_robots.max(next.geode_robots.0);
            max_geodes = max_geodes.max(next.geode.0 as u32);
            visited.insert(next.clone());
            queue.push_back(next);
        }
    }
    max_geodes
}

impl State {
    fn new() -> Self {
        State {
            ore: Ore(0),
            clay: Clay(0),
            obsidian: Obsidian(0),
            geode: Geode(0),
            ore_robots: Ore(1),
            clay_robots: Clay(0),
            obsidian_robots: Obsidian(0),
            geode_robots: Geode(0),
            time_spent: 0,
        }
    }

    fn step(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
        self.time_spent += 1;
    }

    fn branch(&self, blueprint: &Blueprint) -> Vec<Self> {
        let mut branches = vec![];
        if self.ore >= blueprint.geode_robot_price.0
            && self.obsidian >= blueprint.geode_robot_price.1
        {
            let mut buy_geode = self.clone();
            buy_geode.ore -= blueprint.geode_robot_price.0;
            buy_geode.obsidian -= blueprint.geode_robot_price.1;
            buy_geode.step();
            buy_geode.geode_robots += Geode(1);
            branches.push(buy_geode);
            return branches; // If we can produce geodes, we ignore the rest.
        }
        if self.ore >= blueprint.obsidian_robot_price.0
            && self.clay >= blueprint.obsidian_robot_price.1
            && self.obsidian_robots < blueprint.geode_robot_price.1
        {
            let mut buy_obsidian = self.clone();
            buy_obsidian.ore -= blueprint.obsidian_robot_price.0;
            buy_obsidian.clay -= blueprint.obsidian_robot_price.1;
            buy_obsidian.step();
            buy_obsidian.obsidian_robots += Obsidian(1);
            branches.push(buy_obsidian);
            return branches; // Obsidian can also be strictly prioritized.
        }
        if self.ore >= blueprint.clay_robot_price
            && self.clay_robots < blueprint.obsidian_robot_price.1
        {
            let mut buy_clay = self.clone();
            buy_clay.ore -= blueprint.clay_robot_price;
            buy_clay.step();
            buy_clay.clay_robots += Clay(1);
            branches.push(buy_clay);
        }
        if self.ore >= blueprint.ore_robot_price && self.ore_robots < blueprint.get_max_ore() {
            let mut buy_ore = self.clone();
            buy_ore.ore -= blueprint.ore_robot_price;
            buy_ore.step();
            buy_ore.ore_robots += Ore(1);
            branches.push(buy_ore);
        }
        {
            let mut buy_nothing = self.clone();
            buy_nothing.step();
            branches.push(buy_nothing);
        }
        branches
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct State {
    ore: Ore,
    clay: Clay,
    obsidian: Obsidian,
    geode: Geode,
    ore_robots: Ore,
    clay_robots: Clay,
    obsidian_robots: Obsidian,
    geode_robots: Geode,
    time_spent: u8,
}

#[derive(Debug, Hash, AddAssign, SubAssign, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Ore(u8);
#[derive(Debug, Hash, AddAssign, SubAssign, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Clay(u8);
#[derive(Debug, Hash, AddAssign, SubAssign, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Obsidian(u8);
#[derive(Debug, Hash, AddAssign, SubAssign, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Geode(u8);

struct Blueprint {
    index: u32,
    ore_robot_price: Ore,
    clay_robot_price: Ore,
    obsidian_robot_price: (Ore, Clay),
    geode_robot_price: (Ore, Obsidian),
}

impl Blueprint {
    fn get_max_ore(&self) -> Ore {
        self.ore_robot_price
            .max(self.clay_robot_price)
            .max(self.obsidian_robot_price.0)
    }
}

fn parse(input: &str) -> Vec<Blueprint> {
    let re = regex::Regex::new(
r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.",
    )
    .unwrap();
    let mut blueprints = vec![];
    for cap in re.captures_iter(input) {
        blueprints.push(Blueprint {
            index: cap[1].parse().unwrap(),
            ore_robot_price: Ore(cap[2].parse().unwrap()),
            clay_robot_price: Ore(cap[3].parse().unwrap()),
            obsidian_robot_price: (Ore(cap[4].parse().unwrap()), Clay(cap[5].parse().unwrap())),
            geode_robot_price: (
                Ore(cap[6].parse().unwrap()),
                Obsidian(cap[7].parse().unwrap()),
            ),
        });
    }
    blueprints
}
