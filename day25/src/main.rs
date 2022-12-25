use std::{collections::HashMap, fs::read_to_string};

fn main() {
    println!(
        "Part 1: {}",
        convert_10_to_5(
            read_to_string("./data.txt")
                .expect("File does not exist!")
                .lines()
                .map(|line| { convert_5_to_10(line) })
                .collect::<Vec<i64>>()
                .iter()
                .sum::<i64>()
        )
    );
}

fn convert_5_to_10(n: &str) -> i64 {
    let mut result = 0;
    let mut multiplier = 1;
    for c in n.chars().rev() {
        match c {
            '1' => result += multiplier,
            '2' => result += 2 * multiplier,
            '-' => result -= multiplier,
            '=' => result -= 2 * multiplier,
            _ => (),
        }
        multiplier *= 5;
    }
    result
}

fn convert_10_to_5(n: i64) -> String {
    let table = HashMap::from([
        (0, "0"),
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
    ]);
    let mut result = String::new();
    let mut n = n;
    let mut carry = 0;
    while n != 0 {
        let val = table[&(n % 5 + carry)];
        result.push_str(&val[val.len() - 1..]);
        carry = if val.len() == 2 {
            val[..1].parse::<i64>().unwrap()
        } else {
            0
        };
        n /= 5;
    }
    if carry != 0 {
        result.push_str(&carry.to_string());
    }
    result.chars().rev().collect()
}
