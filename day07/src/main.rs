use regex::Regex;
use std::{collections::VecDeque, fs::read_to_string};

fn main() {
    // Great programmers: I shall build a tree. I shall use recursion.
    // Me in Rust       : Pain is a pleasure.
    let cd_back = Regex::new(r"^\$ cd \.\.").unwrap();
    let cd_forw = Regex::new(r"^\$ cd (.*)").unwrap();
    let file = Regex::new(r"^([0-9]+).*").unwrap();
    let dir_reg = Regex::new(r"^dir .*").unwrap();

    let mut sum = 0;

    let mut dirs: VecDeque<&str> = VecDeque::new();
    let mut files: VecDeque<i32> = VecDeque::new();
    let mut go_up: VecDeque<i32> = VecDeque::new();

    let mut out: Vec<i32> = Vec::new();

    read_to_string("./data.txt")
        .expect("File does not exist!")
        .lines()
        .for_each(|line| {
            if cd_back.is_match(line) || line.trim().is_empty() {
                while !dirs.is_empty() {
                    dirs.pop_back();

                    if sum != -1 {
                        files.push_back(sum);
                        sum = -1;
                    }

                    out.push(files.pop_back().unwrap() + go_up.pop_back().unwrap());
                    let last = go_up.pop_back().unwrap_or(0);
                    go_up.push_back(out[out.len() - 1] + last);

                    if cd_back.is_match(line) {
                        break;
                    }
                }
            } else if cd_forw.is_match(line) {
                if sum != -1 {
                    files.push_back(sum);
                }
                sum = -1;
                go_up.push_back(0);
                dirs.push_back(
                    cd_forw
                        .captures(line)
                        .unwrap()
                        .get(0)
                        .map(|s| s.as_str())
                        .unwrap(),
                );
            } else if file.is_match(line) {
                if sum == -1 {
                    sum = 0;
                }
                sum += line.split(" ").next().unwrap().parse::<i32>().unwrap();
            } else if dir_reg.is_match(line) {
                sum = if sum == -1 { 0 } else { sum };
            }
        });

    println!(
        "Part 1: {}",
        out.iter()
            .fold(0, |sum, e| if *e <= 100000 { sum + e } else { sum })
    );
    println!(
        "Part 2: {}",
        out.iter()
            .filter(|n| **n >= (30000000 - (70000000 - out[out.len() - 1])))
            .min()
            .unwrap()
    );
}
