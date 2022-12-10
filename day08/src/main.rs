use ndarray::{s, Array2};
use std::fs::read_to_string;
use take_until::TakeUntilExt;

/*
Python: Go over the matrix from all directions
        and as you visit trees, keep the max at all times,
        and then keep the trees that are less
        than the max in the HashSet. O(n). EZ.
Rust:   You're weak. We are not afraid of O(n*sqrt(n)). We are speed.
        We shall prove it. We are Rust.
*/
fn main() {
    // Read the data
    let trees: Array2<i32> = Array2::from_shape_vec(
        (99, 99),
        read_to_string("./data.txt")
            .expect("File does not exist!")
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| c.to_string().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<i32>>(),
    )
    .unwrap();

    let mut part1 = 0;
    let mut part2: Vec<usize> = Vec::new();

    trees.indexed_iter().for_each(|((r, c), val)| {
        // Solve part 1
        if r > 0 && r < trees.dim().0 - 1 && c > 0 && c < trees.dim().1 - 1 {
            part1 = if [
                (trees.row(r),    s![..c]),
                (trees.row(r),    s![c + 1..]),
                (trees.column(c), s![..r]),
                (trees.column(c), s![r + 1..]),
            ]
            .iter()
            .any(|(view, s)| view.slice(s).iter().all(|tree| tree < val))
            {
                part1 + 1
            } else {
                part1
            }
        }

        // Solve part 2
        part2.push(
            [
                (trees.row(r),    s![..c; -1]),
                (trees.row(r),    s![c + 1..]),
                (trees.column(c), s![..r; -1]),
                (trees.column(c), s![r + 1..]),
            ]
            .iter()
            .fold(1, |prod, (view, s)| {
                prod * view.slice(s).iter().take_until(|tree| tree >= &val).count()
            }),
        );
    });

    println!("Part 1: {}", (part1 + 2 * trees.dim().0 + 2 * trees.dim().1 - 4));
    println!("Part 2: {}", part2.iter().max().unwrap());
}
