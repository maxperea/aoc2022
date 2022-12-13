use std::collections::VecDeque;

pub fn solution_easy(input: &str) -> i64 {
    let mut monkeys = parse(input);
    let worry_modifier = |x: i64| -> i64 { x / 3 };
    for _ in 0..20 {
        do_round(&mut monkeys, worry_modifier);
    }
    get_monkey_business(monkeys) as i64
}

pub fn solution_hard(input: &str) -> i64 {
    let mut monkeys = parse(input);
    let total_divisor: i64 = monkeys.iter().map(|m| m.divisor).product();
    let worry_modifier = |x: i64| -> i64 { x % total_divisor };
    for _ in 0..10_000 {
        do_round(&mut monkeys, worry_modifier);
    }
    get_monkey_business(monkeys) as i64
}

type Divisor = i64;
type MonkeyIndex = usize;
type WorryLevel = i64;
type Operation = Box<dyn Fn(WorryLevel) -> WorryLevel>;

struct Item {
    worry_level: WorryLevel,
}

struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    divisor: Divisor,
    true_next: MonkeyIndex,
    false_next: MonkeyIndex,
    inspected_count: usize,
}

fn get_monkey_business(monkeys: Vec<Monkey>) -> usize {
    let mut counts: Vec<_> = monkeys
        .iter()
        .map(|monkey| monkey.inspected_count)
        .collect();
    counts.sort();
    counts.pop().unwrap() * counts.pop().unwrap()
}

fn do_round<F>(monkeys: &mut Vec<Monkey>, worry_modifier: F)
where
    F: Fn(i64) -> i64,
{
    for i in 0..monkeys.len() {
        let thrown_items = monkeys[i].throw_items(&worry_modifier);
        for (index, item) in thrown_items {
            monkeys[index].receive(item);
        }
    }
}

impl Monkey {
    fn inspect<F>(&mut self, item: &mut Item, worry_modifier: F) -> MonkeyIndex
    where
        F: Fn(i64) -> i64,
    {
        self.inspected_count += 1;
        item.worry_level = (self.operation)(item.worry_level);
        item.worry_level = worry_modifier(item.worry_level);

        if item.worry_level % self.divisor == 0 {
            self.true_next
        } else {
            self.false_next
        }
    }

    fn throw_items<F>(&mut self, worry_modifier: F) -> Vec<(MonkeyIndex, Item)>
    where
        F: Fn(i64) -> i64,
    {
        let mut throws = vec![];
        while !self.items.is_empty() {
            let mut item = self.items.pop_front().unwrap();
            let monkey_index = self.inspect(&mut item, &worry_modifier);
            throws.push((monkey_index, item));
        }
        throws
    }

    fn receive(&mut self, item: Item) {
        self.items.push_back(item);
    }
}

fn parse(input: &str) -> Vec<Monkey> {
    let mut monkeys = vec![];
    for chunk in input.split("\n\n") {
        monkeys.push(parse_monkey(chunk));
    }
    monkeys
}

fn parse_monkey(input: &str) -> Monkey {
    let lines: Vec<_> = input.lines().collect();
    let num_at_end = |s: &str| s.split_whitespace().last().unwrap().parse().unwrap();
    let divisor: i64 = num_at_end(lines[3]);
    let true_next: usize = num_at_end(lines[4]) as usize;
    let false_next: usize = num_at_end(lines[5]) as usize;
    Monkey {
        items: parse_items(lines[1]),
        operation: parse_operation(lines[2]),
        divisor,
        true_next,
        false_next,
        inspected_count: 0,
    }
}

fn parse_items(input: &str) -> VecDeque<Item> {
    let mut items: VecDeque<Item> = VecDeque::new();
    let re = regex::Regex::new(r"\d+").unwrap();
    for num_str in re.captures_iter(input) {
        let num = &num_str[0].parse().unwrap();
        items.push_back(Item { worry_level: *num });
    }
    items
}

fn parse_operation(input: &str) -> Box<dyn Fn(WorryLevel) -> WorryLevel> {
    let op_strs: Vec<_> = input
        .split(" = ")
        .last()
        .unwrap()
        .split_whitespace()
        .collect();
    match op_strs[..] {
        ["old", "+", "old"] => Box::new(|x| x + x),
        ["old", "*", "old"] => Box::new(|x| x * x),
        ["old", "+", num] => {
            let n = num.parse::<i64>().unwrap();
            Box::new(move |x| x + n)
        }
        ["old", "*", num] => {
            let n = num.parse::<i64>().unwrap();
            Box::new(move |x| x * n)
        }
        _ => panic!(),
    }
}
