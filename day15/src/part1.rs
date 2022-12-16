use std::fs::read_to_string;
use std::collections::HashSet;
use regex::Regex;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32
}

pub fn main() {
    // Points of no beacons
    let mut no_b = HashSet::new();

    let re = Regex::new(r"Sensor at x=(?P<s_x>[0-9-]*), y=(?P<s_y>[0-9-]*): closest beacon is at x=(?P<b_x>[0-9-]*), y=(?P<b_y>[0-9-]*)").unwrap();

    let data = read_to_string("./data.txt").expect("File does not exist!");
    let captures = re.captures_iter(data.as_str());
    
    captures.for_each(|c| {
        let part1_y = 2000000;

        let s = Point { x: c["s_x"].parse::<i32>().unwrap(), y: c["s_y"].parse::<i32>().unwrap() };
        let b = Point { x: c["b_x"].parse::<i32>().unwrap(), y: c["b_y"].parse::<i32>().unwrap() };

        let dist = (s.x - b.x).abs() + (s.y - b.y).abs();

        if (s.y - part1_y).abs() <= dist {
            let delta = dist - (s.y - part1_y).abs();
            for dx in s.x-delta..=s.x+delta {
                let p = Point { x: dx, y: part1_y };
                if p != s && p != b {
                    no_b.insert(p);
                }
            }
        }
    });

    println!("Part 1: {}", no_b.len());
}
