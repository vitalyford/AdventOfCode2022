use ndarray::Array2;
use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

pub fn main() {
    let heights: Array2<i32> = Array2::from_shape_vec(
        (41, 154),
        read_to_string("./data.txt")
            .expect("File does not exist!")
            .lines()
            .map(|line| line.trim().chars().map(|c| c as i32).collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<i32>>(),
    )
    .unwrap();

    let start = heights.iter().position(|val| *val == ('E' as i32)).unwrap();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    //                 (steps, r,   c,  val)
    let mut q: VecDeque<(i32, i32, i32, i32)> = VecDeque::new();

    q.push_back((
        0,
        (start / heights.dim().1) as i32,
        (start % heights.dim().1) as i32,
        'z' as i32,
    ));

    while !q.is_empty() && q[0].3 != 'a' as i32 {
        let curr = q.pop_front().unwrap();
        if visited.contains(&(curr.1, curr.2)) {
            continue;
        }

        visited.insert((curr.1, curr.2));

        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (rdr, cdc) = (curr.1 + dr, curr.2 + dc);
            if rdr >= 0
                && cdc >= 0
                && rdr < heights.dim().0 as i32
                && cdc < heights.dim().1 as i32
                && (curr.3 - heights[(rdr as usize, cdc as usize)] <= 1
                    || heights[(rdr as usize, cdc as usize)] > curr.3)
            {
                q.push_back((
                    curr.0 + 1,
                    rdr,
                    cdc,
                    heights[(rdr as usize, cdc as usize)],
                ));
            }
        }
    }

    println!("Part 2: {}", q[0].0);
}
