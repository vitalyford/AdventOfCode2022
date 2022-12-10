use itertools::Itertools;
use std::{collections::HashSet, fs::read_to_string};

fn tail_pos(size: usize) -> usize {
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); size];

    // Read the data
    let mut moves = HashSet::new();

    read_to_string("./data.txt")
        .expect("File does not exist!")
        .lines()
        .map(|line| {
            line.trim()
                .split(" ")
                .collect_tuple::<(&str, &str)>()
                .unwrap()
        })
        .map(|(direction, step)| {
            (
                // +1 or -1 unit step
                ((direction.chars().next().unwrap() as i32) - ('O' as i32)).signum(),
                // A tuple for how many moves we gotta make for the head
                // If the difference with 'O' is 3, then it's R or L, otherwise it's U or D
                if ((direction.chars().next().unwrap() as i32) - ('O' as i32)).abs() == 3 {
                    (
                        step.parse::<i32>().unwrap()
                            * ((direction.chars().next().unwrap() as i32) - ('O' as i32)).signum(),
                        0,
                    )
                } else {
                    (
                        0,
                        step.parse::<i32>().unwrap()
                            * ((direction.chars().next().unwrap() as i32) - ('O' as i32)).signum(),
                    )
                },
            )
        })
        .for_each(|(unit, mut move_by)| {
            // (x, y)
            while !(move_by.0 == 0 && move_by.1 == 0) {
                let mut head: (i32, i32) = rope[0];

                // Moving the front head
                head = (
                    head.0 + if move_by.0 == 0 { 0 } else { unit },
                    head.1 + if move_by.1 == 0 { 0 } else { unit },
                );

                rope[0] = head;

                // Moving all the tails, one after another
                for head_index in 0..size - 1 {
                    let head = rope[head_index];

                    let mut tail: (i32, i32) = rope[head_index + 1];

                    // Moving tail straight alongside with the head back to back
                    // or moving it diagonally to catch up behind the front knot
                    // that is two diagonal steps away (it's represented by "== 4" below)
                    if ((head.0 - tail.0).abs() + (head.1 - tail.1).abs() == 2
                        && (head.0 == tail.0 || head.1 == tail.1))
                        || (head.0 - tail.0).abs() + (head.1 - tail.1).abs() == 4
                    {
                        tail = ((tail.0 + head.0) / 2, (tail.1 + head.1) / 2)
                    }
                    // Moving tail diagonally
                    else if (head.0 - tail.0).abs() + (head.1 - tail.1).abs() == 3 {
                        tail = if (head.0 - tail.0).abs() == 1 {
                            (head.0, (tail.1 + head.1) / 2)
                        } else {
                            // (head.1 - tail.1).abs() == 1
                            ((tail.0 + head.0) / 2, head.1)
                        }
                    }

                    rope[head_index + 1] = tail;
                }

                // Updating how many more moves we gotta make
                move_by = (
                    if move_by.0 == 0 {
                        move_by.0
                    } else {
                        move_by.0 - unit
                    },
                    if move_by.1 == 0 {
                        move_by.1
                    } else {
                        move_by.1 - unit
                    },
                );

                moves.insert(rope[rope.len() - 1]);
            }
        });

    moves.insert((0, 0));

    moves.len()
}

fn main() {
    println!("Part 1: {}", tail_pos(2));
    println!("Part 2: {}", tail_pos(10));
}
