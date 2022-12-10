extern crate tuple_conv;
use crate::tuple_conv::RepeatedTuple;
use array_tool::vec::Intersect;
use std::fs::read_to_string;

fn main() {
    let ascii_a = 'A' as i32;
    let part1: i32 = read_to_string("./data.txt")
        .expect("File does not exist!")
        .lines()
        .map(|line| line.trim().split_at(line.len() / 2).to_vec())
        .map(|v: Vec<&str>| {
            v[0].chars()
                .collect::<Vec<char>>()
                .intersect(v[1].chars().collect::<Vec<char>>())
                .into_iter()
                .fold(0, |acc, c| acc + (c as i32 - ascii_a + 26) % 58 + 1)
        })
        .sum();

    let mut part2 = 0;
    read_to_string("./data.txt")
        .expect("File does not exist!")
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
        .chunks(3)
        .for_each(|v| {
            part2 += v[0]
                .intersect(v[1].to_vec())
                .intersect(v[2].to_vec())
                .into_iter()
                .fold(0, |acc, c| acc + (c as i32 - ascii_a + 26) % 58 + 1)
        });

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
