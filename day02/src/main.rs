use std::{fs::read_to_string};

fn main() {
    let ascii_a = 'A' as i32;
    let ascii_x = 'X' as i32;
    let mut part2: i32 = 0;
    let part1: Vec<i32> = read_to_string("./data.txt")
        .expect("File does not exist!")
        .lines()
        .map(|line| line.trim()
                              .split(' ')
                              .map(|s| s.chars().next().unwrap()  as i32)
                              .collect::<Vec<i32>>())
        .scan(0, |part1: &mut i32, x: Vec<i32>| {
            // solve part 1
            match (x[1] - ascii_x) - (x[0] - ascii_a) {
                -2 | 1 => *part1 += x[1] - ascii_x + 7, // 1 + 6 (won)
                   0   => *part1 += x[1] - ascii_x + 4, // 1 + 3 (drew)
                   _   => *part1 += x[1] - ascii_x + 1  // 1 + 0 (lost)
            };
            // solve part 2
            match x[1] - ascii_x {
                0 => part2 += if x[0] - ascii_a == 0 { 3 } else { x[0] - ascii_a },    // need to lose: + 0
                1 => part2 += x[0] - ascii_a + 4,                                      // need to draw: + 3
                _ => part2 += if x[0] - ascii_a == 2 { 7 } else { x[0] - ascii_a + 8 } // need to win:  + 6
            };
            Some(*part1)
        })
        .collect();
    
    println!("Part 1: {}", part1[part1.len() - 1]);
    println!("Part 1: {}", part2);
}
