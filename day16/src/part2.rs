use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};

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
    moves: VecDeque<String>,
    prev: String
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Couple {
    v1: ValveSeq,
    v2: ValveSeq,
}

impl Couple {
    fn pressure(&self) -> i32 {
        self.v1.pressure() + self.v2.pressure()
    }
}

impl Hash for ValveSeq {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.mins.hash(state);
        self.curr.hash(state);
        self.curr_min.hash(state);
        self.prev.hash(state);
        for s in self.opened.iter() {
            s.hash(state);
        }
    }
}

impl ValveSeq {
    fn pressure(&self) -> i32 {
        self.mins.iter().enumerate().fold(0, |acc, (i, p)| {
            if i > 26 {
                acc
            } else {
                acc + (26 - i as i32) * p
            }
        })
    }

    fn curr_is_opened(&self) -> bool {
        self.opened.contains(&self.curr)
    }

    fn add(&mut self, flow: i32) {
        self.mins.push(flow);
        self.opened.insert(self.curr.clone());
        self.curr_min += 1;
        self.prev = self.curr.clone();
    }

    fn move_in_tunnel(&mut self, name: String) {
        self.mins.push(0);
        self.prev = self.curr.clone();
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

impl PartialOrd for Couple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Couple {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pressure().cmp(&other.pressure())
    }
}

fn next_paths(vs: ValveSeq, valves: &HashMap<String, Valve>) -> Vec<ValveSeq> {
    if vs.curr_min == 31 {
        return vec![vs];
    }

    let mut vs_list: Vec<ValveSeq> = Vec::new();

    // Opening the vs.curr valve
    if !vs.curr_is_opened() && valves[&vs.curr].flow != 0 {
        let mut vs_open = vs.clone();
        vs_open.add(valves[&vs.curr].flow);
        vs_list.push(vs_open);
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

                vs_list.push(next_vs);
            }
        });

    // All valves are opened
    if vs.opened.len() == valves.len() {
        vs_list.push(vs.clone());
    }

    vs_list
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

    let mut seqs: BinaryHeap<Couple> = BinaryHeap::new();
    seqs.push(Couple {
        v1: ValveSeq {
            opened: HashSet::new(),
            mins: vec![0],
            curr: "AA".to_string(),
            curr_min: 0,
            moves: VecDeque::from(["AA".to_string()]),
            prev: "1".to_string()
        },
        v2: ValveSeq {
            opened: HashSet::new(),
            mins: vec![0],
            curr: "AA".to_string(),
            curr_min: 0,
            moves: VecDeque::from(["AA".to_string()]),
            prev: "2".to_string()
        },
    });

    'a: loop {
        let mut process_seqs = VecDeque::new();
        let mut dups: HashSet<Couple> = HashSet::new();
        // Get the top 100000 couples to process
        // We could probably do less but it worked with 100K (!)
        let mut count = 100000;
        while count >= 0 {
            if seqs.is_empty() {
                break;
            }
            let couple = seqs.pop().unwrap();
            if !dups.contains(&couple.clone()) {
                dups.insert(couple.clone());
                process_seqs.push_back(couple);
                count -= 1;
            }
        }
        seqs.clear();
        while !process_seqs.is_empty() {
            let couple = process_seqs.pop_front().unwrap();

            if couple.v1.curr_min >= 27 && couple.v2.curr_min >= 27 {
                seqs.push(couple);
                break 'a;
            }

            let paths1 = next_paths(couple.v1, &valves);
            let paths2 = next_paths(couple.v2, &valves);

            for p1 in &paths1 {
                for p2 in &paths2 {
                    if !(p1.prev.eq(&p2.prev) && p1.curr.eq(&p2.curr) && p1.prev.eq(&p1.curr)) {
                        let mut clone_p1 = p1.clone();
                        let mut clone_p2 = p2.clone();
                        clone_p1.opened.extend(p2.clone().opened);
                        clone_p2.opened.extend(p1.clone().opened);
                        seqs.push(Couple { v1: clone_p1, v2: clone_p2 });
                    }
                }
            }
        }
    }

    println!("Part 2: {}", seqs.pop().unwrap().pressure());
}
