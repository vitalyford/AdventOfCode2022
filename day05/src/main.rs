use std::{fs::read_to_string};
use std::collections::{BTreeMap, VecDeque};

fn main() {
    let mut crates = BTreeMap::new();
    read_to_string("./data.txt")
        .expect("File does not exist!")
        .lines()
        .for_each(|line| 
            if !line.trim().is_empty() { 
                match &line.trim()[0..1] {
                    "[" =>  line.chars()
                                .collect::<Vec<char>>()
                                .chunks(4)
                                .map(|c| c.iter().collect::<String>())
                                .enumerate()
                                .filter_map(|(i, s)| match s.chars().nth(1).unwrap() {
                                    ' ' => None,
                                     _  => Some(((i + 1).to_string(), s[1..2].to_owned()))
                                })
                                .for_each(|(i, s)| crates.entry(i)
                                                         .or_insert(VecDeque::new())
                                                         .push_back(s)),
                    "m" =>  for i in 0..line.split(" ")
                                            .map(|e| e.to_string())
                                            .collect::<Vec<String>>()[1]
                                            .parse()
                                            .unwrap() {
                                let val = crates.get_mut(&line.split(" ")
                                                              .map(|e| e.to_string())
                                                              .collect::<Vec<String>>()[3])
                                                .unwrap()
                                                .pop_front()
                                                .unwrap();
                                crates.get_mut(&line.split(" ")
                                                    .map(|e| e.to_string())
                                                    .collect::<Vec<String>>()[5])
                                      .unwrap()
                                      .insert(i, val);
                    },
                    _ => ()
                }
            }
        );

    // Part 2
    // For Part 1, change Line 39 to push_front without the index i
    crates.iter_mut()
          .for_each(|(_, v)| print!("{}", v.pop_front().unwrap()));
}
