use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

pub fn main() {
    let re = Regex::new(r"Sensor at x=(?P<s_x>[0-9-]*), y=(?P<s_y>[0-9-]*): closest beacon is at x=(?P<b_x>[0-9-]*), y=(?P<b_y>[0-9-]*)").unwrap();

    let data = read_to_string("./data.txt").expect("File does not exist!");
    let pairs = re.captures_iter(data.as_str())
        .map(|c| (Point { x: c["s_x"].parse::<i32>().unwrap(), y: c["s_y"].parse::<i32>().unwrap() }, Point { x: c["b_x"].parse::<i32>().unwrap(), y: c["b_y"].parse::<i32>().unwrap() }))
        .collect::<Vec<(Point, Point)>>();
    
    'a: for y in 0..=4000000 {
        // Min-heap didn't help at all so I stuck with Vec
        let mut ranges: Vec<Vec<i32>> = Vec::new();
        pairs.iter().for_each(|p| {
            let s = &p.0;
            let b = &p.1;
            
            let d = (s.x - b.x).abs() + (s.y - b.y).abs();

            if (s.y - y).abs() <= d {
                let delta = d - (s.y - y).abs();
                ranges.push(vec![s.x - delta, s.x + delta]);
            }
        });

        ranges.sort();

        if ranges.len() > 0 {
            let mut merged_range = ranges[0].clone();
            for i in 1..ranges.len() {
                if ranges[i][0] > merged_range[1] {
                    // Found the lost beacon
                    println!("Part 2: {}", (ranges[i][0] - 1) as i64 * 4000000 + y as i64);
                    break 'a;
                }
                else {
                    merged_range[1] = std::cmp::max(ranges[i][1], merged_range[1]);
                }
            }
        }
    }
}
