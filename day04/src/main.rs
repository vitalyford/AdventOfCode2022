use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let mut part2 = 0;
    let part1: i32 = read_to_string("./data.txt")
        .expect("File does not exist!")
        .lines()
        .map(|line| {
            Regex::new(r"-|,")
                .unwrap()
                .split(line.trim())
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .fold(0, |part1, v| {
            part2 += if (v[0] <= v[2] && v[1] >= v[2]) || (v[2] <= v[0] && v[3] >= v[0]) {
                1
            } else {
                0
            };
            part1
                + if (v[0] <= v[2] && v[1] >= v[3]) || (v[0] >= v[2] && v[1] <= v[3]) {
                    1
                } else {
                    0
                }
        });

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
