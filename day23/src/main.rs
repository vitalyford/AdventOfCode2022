use std::{fs::read_to_string, collections::{VecDeque, HashMap, HashSet}};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Elf {
    x: i32,  // current x
    y: i32,  // current y
}

fn elf_is_nearby(elves: &HashSet<Elf>, elf: &Elf) -> bool {
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            if elves.contains(&Elf { x: elf.x + i, y: elf.y + j }) {
                return true;
            }
        }
    }
    false
}

fn main() {
    let mut elves = read_to_string("./data.txt")
        .expect("File does not exist!")
        .lines()
        .enumerate()
        .fold(HashSet::new(), |mut acc, (row, line)| {
            for (col, c) in line.trim().chars().enumerate() {
                if c.eq(&'#') {
                    acc.insert(Elf { x: col as i32, y: -(row as i32) });
                }
            }
            acc
        });

    let mut directions = VecDeque::from(vec!["N", "S", "W", "E"]);

    /*
    From the problem definition of Day 23, AoC 2023:
    
    "During the first half of each round, each Elf considers the eight positions adjacent to themself. If no other Elves are in one of those eight positions, the Elf does not do anything during this round. Otherwise, the Elf looks in each of four directions in the following order and proposes moving one step in the first valid direction:

    If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
    If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
    If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
    If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step."
    */
    for round in 0.. {
        let mut proposed_moves = HashMap::new();
        let mut dups = HashMap::new();
        let mut has_nearby = false;
        for elf in &elves {
            if elf_is_nearby(&elves, elf) {
                has_nearby = true;
                let mut proposed_move = None;
                'dir: for direction in &directions {
                    let (px, py) = match direction {
                        &"N" | &"S" => {
                            for i in -1..=1 {
                                if elves.contains(&Elf { x: elf.x + i, y: elf.y + ('Q' as i32 - direction.chars().next().unwrap() as i32).signum() }) {
                                    continue 'dir;
                                }
                            }
                            (elf.x, elf.y + ('Q' as i32 - direction.chars().next().unwrap() as i32).signum())
                        },
                        &"E" | &"W" => {
                            for i in -1..=1 {
                                if elves.contains(&Elf { x: elf.x + ('Q' as i32 - direction.chars().next().unwrap() as i32).signum(), y: elf.y + i }) {
                                    continue 'dir;
                                }
                            }
                            (elf.x + ('Q' as i32 - direction.chars().next().unwrap() as i32).signum(), elf.y)
                        },
                        _ => panic!("Invalid direction"),
                    };
                    proposed_move = Some((px, py));
                    break;
                }
                if let Some((px, py)) = proposed_move {
                    proposed_moves.insert((elf.x, elf.y), (px, py));
                    dups.entry((px, py)).and_modify(|counter| *counter += 1).or_insert(1);
                }
            }
        }

        if !has_nearby {
            println!("Part 2: {}", round + 1);
            break;
        }

        // Second half of the round, elves are moving to the proposed positions
        let mut new_elves = HashSet::new();
        for elf in &elves {
            if let Some((px, py)) = proposed_moves.get(&(elf.x, elf.y)) {
                if dups.get(&(*px, *py)).unwrap() == &1 {
                    new_elves.insert(Elf { x: *px, y: *py });
                } else {
                    new_elves.insert(Elf { x: elf.x, y: elf.y });
                }
            } else {
                new_elves.insert(Elf { x: elf.x, y: elf.y });
            }
        }

        elves = new_elves;

        let d = directions.pop_front().unwrap();
        directions.push_back(d);

        if round == 9 {
            // Find the smallest rectange that contains all the elves
            let (min_x, max_x, min_y, max_y) = elves.iter().fold((i32::MAX, i32::MIN, i32::MAX, i32::MIN), |(min_x, max_x, min_y, max_y), elf| {
                (min_x.min(elf.x), max_x.max(elf.x), min_y.min(elf.y), max_y.max(elf.y))
            });
            
            println!("Part 1: {}", (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i32);
        }
    }

}
