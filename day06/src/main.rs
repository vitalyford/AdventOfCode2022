use std::{fs::read_to_string};

use itertools::Itertools;

fn main() {
    // How come we are not finding all such possible windows?!
    // Gotta do it, we cannot be satisfied with just one or two
    // And how about we avoid HashSet at all cost? Because why not
    let mut p1_p2: Vec<Vec<(bool, usize)>> = vec![vec![], vec![]];
    [4, 14].into_iter()
           .for_each(|n| {
                read_to_string("./data.txt")
                    .expect("File does not exist!")
                    .lines()
                    .next()
                    .unwrap()
                    .chars()
                    .collect::<Vec<char>>()
                    .windows(n)
                    .enumerate()
                    .for_each(|(i, w)| {
                        p1_p2[(n % 4) / 2].push((w.iter().all_unique(), i + n));
                    })
           });
    
    // Printing both answers for part 1 and part 2
    p1_p2.iter()
         .enumerate()
         .for_each(|(p, v)| {
            v.iter()
             .find(|(is_unique, _)| *is_unique)
             .and_then(|(_, index)| Some(println!("Part {}: {}", p + 1, index)));
         });
}
