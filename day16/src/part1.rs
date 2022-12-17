use regex::Regex;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
use std::fs::read_to_string;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Valve {
    flow: i32,
    tunnels: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ValveSeq {
    opened: HashSet<String>,
    mins: Vec<i32>, // keeps track of flows opened at certain minutes
    curr: String,
    curr_min: i32,
    moves: VecDeque<String>
}

impl ValveSeq {
    fn pressure(&self) -> i32 {
        self.mins
            .iter()
            .enumerate()
            .fold(0, |acc, (i, p)| if i > 30 { acc } else { acc + (30 - i as i32) * p })
    }

    fn curr_is_opened(&self) -> bool {
        self.opened.contains(&self.curr)
    }

    fn add(&mut self, flow: i32) {
        self.mins.push(flow);
        self.opened.insert(self.curr.clone());
        self.curr_min += 1;
    }

    fn move_in_tunnel(&mut self, name: String) {
        self.mins.push(0);
        self.curr = name;
        self.curr_min += 1;

        self.moves.push_back(self.curr.clone());
        if self.moves.len() == 5 {
            self.moves.pop_front();
        }
    }
}

impl PartialOrd for ValveSeq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ValveSeq {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pressure().cmp(&other.pressure())
    }
}

pub fn main() {
    let re = Regex::new(r"Valve (?P<name>[A-Z]*) has flow rate=(?P<flow>[0-9]*); tunnels? leads? to valves? (?P<tunnels>[A-Z, ]*)").unwrap();

    let data = read_to_string("./data.txt").expect("File does not exist!");
    let captures = re.captures_iter(data.as_str());

    let mut valves = HashMap::new();
    captures.for_each(|c| {
        valves.insert(
            c["name"].to_string(),
            Valve {
                flow: c["flow"].parse::<i32>().unwrap(),
                tunnels: c["tunnels"]
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            },
        );
    });

    let mut seqs: BinaryHeap<ValveSeq> = BinaryHeap::new();
    seqs.push(ValveSeq {
        opened: HashSet::new(),
        mins: vec![0],
        curr: "AA".to_string(),
        curr_min: 0,
        moves: VecDeque::from(["AA".to_string()])
    });

    'a: loop {
        let mut process_seqs = VecDeque::new();
        // Get the top 1000 paths to process
        for _ in 0..1000 {
            if seqs.is_empty() {
                break;
            }
            process_seqs.push_back(seqs.pop().unwrap());
        }
        seqs.clear();
        while !process_seqs.is_empty() {
            let vs = process_seqs.pop_front().unwrap();
            if vs.curr_min == 31 {
                seqs.push(vs);
                break 'a;
            }

            // Opening the vs.curr valve
            if !vs.curr_is_opened() && valves[&vs.curr].flow != 0 {
                let mut vs_open = vs.clone();
                vs_open.add(valves[&vs.curr].flow);
                seqs.push(vs_open);
            }
            // Moving through all tunnels
            valves
                .get(&vs.curr)
                .unwrap()
                .tunnels
                .iter()
                .for_each(|name| {
                    if vs.opened.len() != valves.len() {
                        let mut next_vs = vs.clone();
                        next_vs.move_in_tunnel(name.clone());

                        seqs.push(next_vs);
                    }
                });

            // All valves are opened
            if vs.opened.len() == valves.len() {
                seqs.push(vs.clone());
            }
        }
    }

    println!("Part 1: {}", seqs.pop().unwrap().pressure());
}
