use itertools::Itertools;
use serde::Deserialize;
use std::{cmp::Ordering, fs::read_to_string};

#[derive(Eq, PartialEq, Deserialize)]
#[serde(untagged)]
enum Packet {
    Item(i32),
    Nested(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;
        match (self, other) {
            (Item(l),   Item(r))   => l.cmp(r),
            (Item(l),   Nested(r)) => [Item(*l)][..].cmp(r),
            (Nested(l), Nested(r)) => l.cmp(r),
            (Nested(l), Item(r))   => l[..].cmp(&[Item(*r)]),
        }
    }
}

fn main() {
    let mut data = read_to_string("./data.txt")
        .expect("File does not exist!")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str::<Packet>(line.trim()).unwrap())
        .collect::<Vec<Packet>>();

    let part1 =
        data.iter().tuples().enumerate().fold(
            0,
            |acc, (i, (left, right))| {
                if left < right {
                    acc + i + 1
                } else {
                    acc
                }
            },
        );

    ["[[2]]", "[[6]]"]
        .iter()
        .for_each(|e| data.push(serde_json::from_str::<Packet>(e).unwrap()));

    data.sort();

    let part2 = (data
        .iter()
        .position(|p| *p == serde_json::from_str::<Packet>("[[2]]").unwrap())
        .unwrap()
        + 1)
        * (data
            .iter()
            .position(|p| *p == serde_json::from_str::<Packet>("[[6]]").unwrap())
            .unwrap()
            + 1);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
