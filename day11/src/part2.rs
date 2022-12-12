use regex::Regex;
use std::{collections::VecDeque, fs::read_to_string};

#[derive(Clone)]
struct Monkey {
    items:    VecDeque<u64>,
    op:       (String, String),
    test:     u64,
    if_true:  usize,
    if_false: usize,
    inspects: u64
}

impl Monkey {
    fn from_captures(captures: regex::Captures) -> Self {
        let items = captures["items"]
            .split(", ")
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<VecDeque<u64>>();

        let ops = captures["op"].split_once(" ").unwrap();
        let op  = (ops.0.to_string(), ops.1.to_string());

        let test     = captures["test"].parse::<u64>().unwrap();
        let if_true  = captures["if_true"].parse::<usize>().unwrap();
        let if_false = captures["if_false"].parse::<usize>().unwrap();
        let inspects = 0;

        Monkey {
            items,
            op,
            test,
            if_true,
            if_false,
            inspects
        }
    }
}

// Matches for Windows with \r\n. If running on Linux, change to \n only.
pub fn main() {
    let re = Regex::new(r"Monkey (?P<monkey_id>[0-9]):\r\n.*?Starting items: (?P<items>[0-9, ]*)\r\n.*?Operation: new = old (?P<op>[*-+/ a-z0-9]*)\r\n.*?Test: divisible by (?P<test>[0-9]*)\r\n.*?If true: throw to monkey (?P<if_true>[0-9])\r\n.*?If false: throw to monkey (?P<if_false>[0-9])\r\n").unwrap();

    let data        = read_to_string("./data.txt").expect("File does not exist!");
    let captures    = re.captures_iter(data.as_str());
    let mut monkeys = captures.map(Monkey::from_captures).collect::<Vec<Monkey>>();

    // Find the least common multiple of all tests (divisors) from monkeys.
    // We could have just multiplied all tests (divisors) as they are all coprime
    // but that's too weak.
    let modulo = monkeys.iter().fold(1, |prod, m|
        if prod % m.test == 0 { prod } else { prod * m.test }
    );

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() != 0 {
                let m = &mut monkeys[i];
                let val = m.op.1.parse::<u64>().unwrap_or(*m.items.get(0).unwrap());
                let level = match m.op.0.as_str() {
                    "*" => (m.items.pop_front().unwrap() * val) % modulo,
                     _  => (m.items.pop_front().unwrap() + val) % modulo,
                };
                m.inspects += 1;
                let throw_id = if level % m.test == 0 { m.if_true } else { m.if_false };
                monkeys[throw_id].items.push_back(level);
            }
        }
    }

    monkeys.sort_by(|m1, m2| m2.inspects.cmp(&m1.inspects));

    println!("Part 2: {}", monkeys[0].inspects * monkeys[1].inspects);
}
