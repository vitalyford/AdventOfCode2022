use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32, // going right is positive
    y: i32, // going down is positive
    val: char,
}

fn main() {
    let map = read_to_string("./data.txt")
        .expect("File does not exist!")
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut blizs = HashSet::new();
    let me = Point { x: 1, y: 0, val: 'E' };

    for (row, v) in map.iter().enumerate() {
        for (col, c) in v.iter().enumerate() {
            if c.ne(&'.') && c.ne(&'#') {
                blizs.insert(Point { x: col as i32, y: row as i32, val: *c });
            }
        }
    }

    println!("Part 1: {}", search(me.clone(), Point { x: map[0].len() as i32 - 2, y: map.len() as i32 - 1, val: 'E'}, &map, &blizs, 1));
    println!("Part 2: {}", search(me.clone(), Point { x: map[0].len() as i32 - 2, y: map.len() as i32 - 1, val: 'E'}, &map, &blizs, 3));
}

fn search(init_start: Point, init_end: Point, map: &Vec<Vec<char>>, blizzards: &HashSet<Point>, times: usize) -> i32 {
    let mut steps = 0;
    let ways = vec![(init_start.clone(), init_end.clone()), (init_end.clone(), init_start.clone()), (init_start.clone(), init_end.clone())];
    
    let mut blizs = blizzards.clone();
    for i in 0..times {
        let (start, end) = ways[i].clone();

        let mut paths = HashSet::new();
        paths.insert(start.clone());

        loop {
            let mut new_paths = HashSet::new();
            // Move each blizzard
            let mut new_blizs = HashSet::new();
            for b in blizs.iter() {
                match b.val {
                    '>' => {
                        if b.x + 1 != map[0].len() as i32 - 1 {
                            new_blizs.insert(Point { x: b.x + 1, y: b.y, val: '>' });
                        } else {
                            new_blizs.insert(Point { x: 1, y: b.y, val: '>' });
                        }
                    }
                    '<' => {
                        if b.x != 1 {
                            new_blizs.insert(Point { x: b.x - 1, y: b.y, val: '<' });
                        } else {
                            new_blizs.insert(Point { x: map[0].len() as i32 - 2, y: b.y, val: '<' });
                        }
                    }
                    'v' => {
                        if b.y + 1 != map.len() as i32 - 1 {
                            new_blizs.insert(Point { x: b.x, y: b.y + 1, val: 'v' });
                        } else {
                            new_blizs.insert(Point { x: b.x, y: 1, val: 'v' });
                        }
                    }
                    '^' => {
                        if b.y != 1 {
                            new_blizs.insert(Point { x: b.x, y: b.y - 1, val: '^' });
                        } else {
                            new_blizs.insert(Point { x: b.x, y: map.len() as i32 - 2, val: '^' });
                        }
                    }
                    _ => panic!("Invalid blizzard!"),
                }
            }
            steps += 1;
            blizs = new_blizs;

            let mut quit = false;
            for path in paths.iter() {
                [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)]
                    .iter()
                    .for_each(|(x, y)| {
                        if ['>', '<', 'v', '^'].iter().all(|d| {
                            !blizs.contains(&Point { x: path.x + *x, y: path.y + *y, val: *d })
                        }) {
                            if (path.x + *x >= 1
                                && path.x + *x < map[0].len() as i32 - 1
                                && path.y + *y >= 1
                                && path.y + *y < map.len() as i32 - 1)
                                || (path.x + *x == 1 && path.y + *y == 0)
                                || (path.x + *x == map[0].len() as i32 - 2
                                    && path.y + *y == map.len() as i32 - 1)
                            {
                                if path.x + *x == end.x && path.y + *y == end.y {
                                    quit = true;
                                }
                                new_paths.insert(Point { x: path.x + *x, y: path.y + *y, val: path.val });
                            }
                        }
                    });
            }
            if quit {
                break;
            }
            paths = new_paths;
        }
    }
    steps
}
