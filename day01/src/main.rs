use std::{collections::BinaryHeap, fs::read_to_string};

fn main() {
    let mut cals: BinaryHeap<u32> = read_to_string("./data.txt")
        .expect("File does not exist!")
        .lines()
        .map(|line| line.trim())
        .scan(0, |acc: &mut u32, x: &str| {
            match x.parse::<u32>() {
                Ok(num) => {
                    *acc += num;
                    return Some(0);
                }
                Err(_) => {
                    let res = *acc;
                    *acc = 0;
                    return Some(res);
                }
            };
        })
        .filter(|x| *x != 0 as u32)
        .collect();

    let part1 = cals.pop().unwrap();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part1 + cals.pop().unwrap() + cals.pop().unwrap());
}
