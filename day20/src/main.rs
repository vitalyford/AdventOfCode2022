use std::{collections::VecDeque, fs::read_to_string};

fn mix(nums: &mut VecDeque<(i64, i64)>, times: usize) {
    for _ in 0..times {
        for i in 0..nums.len() {
            // Rotate until the first element's enumerated index is i
            while nums[0].0 != i as i64 {
                nums.rotate_left(1);
            }
            let (ind, num) = nums.pop_front().unwrap();

            if num < 0 {
                nums.rotate_right((num.abs() % nums.len() as i64) as usize);
            } else if num > 0 {
                nums.rotate_left((num % nums.len() as i64) as usize);
            }
            nums.push_front((ind, num));
        }
    }
}

fn grove(nums: &mut VecDeque<(i64, i64)>) -> i64 {
    // Rotate until the first element is 0
    while nums[0].1 != 0 as i64 {
        nums.rotate_left(1);
    }

    [1000, 2000, 3000]
        .iter()
        .fold(0, |acc, i| acc + nums[i % nums.len()].1)
}

fn main() {
    let mut nums = read_to_string("./data.txt")
        .expect("File does not exist!")
        .lines()
        .map(|line| line.trim().parse::<i64>().unwrap())
        .collect::<VecDeque<i64>>()
        .iter()
        .enumerate()
        .map(|(i, e)| (i as i64, *e))
        .collect::<VecDeque<(i64, i64)>>();

    let nums_clone = nums.clone();

    mix(&mut nums, 1);

    println!("Part 1: {}", grove(&mut nums));

    let mut corrected = nums_clone
        .iter()
        .map(|(i, num)| (*i, *num * 811589153))
        .collect::<VecDeque<(i64, i64)>>();

    mix(&mut corrected, 10);

    println!("Part 2: {}", grove(&mut corrected));
}
