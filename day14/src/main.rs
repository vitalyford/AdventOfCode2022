use itertools::Itertools;
use ndarray::{s, Array2};
use std::fs::{read_to_string, File};

use std::io::Write;

fn main() {
    // Read all coordinates as (x, y)
    let coors = read_to_string("./data.txt")
        .expect("File does not exist!")
        .lines()
        .map(|line| {
            line.trim()
                .split(" -> ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
        .iter()
        .map(|coors| {
            coors
                .iter()
                .flat_map(|s| {
                    s.split(",")
                        .map(|s| s.parse::<i32>().unwrap())
                        .tuples()
                        .collect::<Vec<(i32, i32)>>()
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .collect::<Vec<Vec<(i32, i32)>>>();

    // Find the cave's size: (max_x, max_y)
    // and add buff to x to account for sand on the floor
    let buff = 200;
    let cave_size = coors
        .clone()
        .into_iter()
        .flatten()
        .collect::<Vec<(i32, i32)>>()
        .iter()
        .fold((0, 0), |size, coor| {
            (
                if size.0 < coor.0 + buff { coor.0 + buff } else { size.0 },
                if size.1 < coor.1 { coor.1 } else { size.1 },
            )
        });

    // Create the cave and set everything to a space character
    let mut cave =
        Array2::from_shape_fn(((cave_size.1 + 2) as usize, cave_size.0 as usize), |_| ' ');

    // Fill up the cave with paths
    coors.iter().for_each(|v| {
        v.iter().tuple_windows().for_each(|((x1, y1), (x2, y2))| {
            cave.slice_mut(s![
                if *y1 < *y2 {
                    (*y1 as usize)..=(*y2 as usize)
                } else {
                    (*y2 as usize)..=(*y1 as usize)
                },
                if *x1 < *x2 {
                    (*x1 as usize)..=(*x2 as usize)
                } else {
                    (*x2 as usize)..=(*x1 as usize)
                }
            ])
            .map_inplace(|c| *c = '#')
        })
    });

    let mut done_part2 = false;
    let mut done_part1 = false;
    for i in 0.. {
        // (0, 1)
        // (y, x)
        let mut pos = (0, 500);
        loop {
            let initial = pos.clone();
            // Landing on the floor
            if pos.0 == cave.dim().0 - 1 {
                if !done_part1 {
                    done_part1 = true;
                    println!("Part 1: {}", i);
                }
                *cave.get_mut(pos).unwrap() = 'o';
                break;
            }
            // Try to go down, go left, and go right
            [
                (pos.0 + 1, pos.1),     // down
                (pos.0 + 1, pos.1 - 1), // down-left
                (pos.0 + 1, pos.1 + 1), // down-right
            ]
            .iter()
            .try_for_each(|index| {
                Some({
                    if cave.get(*index).unwrap().eq(&' ') {
                        pos = *index;
                        return None;
                    }
                })
            });
            // Time to rest!
            if initial == pos {
                *cave.get_mut(pos).unwrap() = 'o';
                done_part2 = pos == (0, 500); // We filled it up
                break;
            }
        }

        if done_part2 {
            println!("Part 2: {}", i + 1);
            break;
        }
    }

    // But we gotta see the cave for profit!
    let mut file = File::create("cave.txt")
        .expect("Failed to create file");

    for row in cave.outer_iter() {
        let row_string = row
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("");
        file.write_all(row_string.as_bytes())
            .expect("Failed to write to file");
        file.write_all(b"\n").expect("Failed to write to file");
    }
}
