use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    let mut curr_cycle = 0;
    let mut x = 1;
    let mut sprite = 0;

    println!("Part 2 (gonna be drawn below): ");

    let sig = read_to_string("./data.txt")
        .expect("File does not exist!")
        .lines()
        .map(|line| {
            line.trim()
                .split(" ")
                .collect_tuple::<(&str, &str)>()
                .unwrap_or(("0", "1"))
        })
        .map(|(op, val)| {
            match op {
                // add (value, cycles)
                "0" => (0, 1),
                _ => (val.parse::<i32>().unwrap(), 2),
            }
        })
        .collect::<Vec<(i32, i32)>>()
        .iter()
        .fold(0, |mut acc, (val, cycles)| {
            for _ in 0..*cycles {
                if sprite <= curr_cycle % 40 && curr_cycle % 40 <= sprite + 2 {
                    print!("#");
                } else {
                    print!(" ");
                }

                curr_cycle += 1;

                if curr_cycle % 40 == 0 {
                    println!("");
                }

                if curr_cycle == 20 || (curr_cycle - 20) % 40 == 0 {
                    acc += curr_cycle * x;
                }
            }
            x += val;
            sprite = x - 1;
            acc
        });

    println!("Part 1: {}", sig);
}
