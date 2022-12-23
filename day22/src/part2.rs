use std::fs::read_to_string;
use regex::Regex;
use ndarray::{Array2, s};

pub fn main() {
    let f = read_to_string("./data.txt")
        .expect("File does not exist!");
    let data = f
        .split("\r\n\r\n")
        .collect::<Vec<&str>>();
    
    let (height, width) = (200, 150);

    let grid: Array2<char> = Array2::from_shape_vec(
        (height, width),
        data[0]
            .split("\r\n")
            .flat_map(|x| {
                let mut v = vec![' '; width];
                v[0..x.len()].copy_from_slice(x.chars().collect::<Vec<char>>().as_slice());
                v
            })
            .collect::<Vec<char>>()
    ).unwrap();

    let row_bounds = grid.axis_iter(ndarray::Axis(0)).map(|row| {
        (
            row.iter().position(|&x| x == '#' || x == '.').unwrap(),
            row.iter().rposition(|&x| x == '#' || x == '.').unwrap()
        )
    }).collect::<Vec<(usize, usize)>>();

    // find the bounds of the grid's columns
    let col_bounds = grid.axis_iter(ndarray::Axis(1)).map(|col| {
        (
            col.iter().position(|&x| x == '#' || x == '.').unwrap(),
            col.iter().rposition(|&x| x == '#' || x == '.').unwrap()
        )
    }).collect::<Vec<(usize, usize)>>();

    let re = Regex::new(r"(?P<steps>\d+)(?P<direction>[RL\s]{1})").unwrap();

    let captures = re.captures_iter(data[1]);

    // starting row and column
    let (mut r, mut c, mut facing) = (0, 50, "R");//data[0].find('.').unwrap()

    captures.for_each(|cap| {
        let mut steps = cap.name("steps").unwrap().as_str().parse::<usize>().unwrap();
        let direction = cap.name("direction").unwrap().as_str();

        'a: loop {
            let (slice, dx, mut move_ind) = match facing {
                "R" | "L" => (grid.slice(s![r, row_bounds[r].0..=row_bounds[r].1]), (facing.chars().next().unwrap() as i32 - 'O' as i32).signum(), (c - row_bounds[r].0) as i32),
                "U" | "D" => (grid.slice(s![col_bounds[c].0..=col_bounds[c].1, c]), ('O' as i32 - facing.chars().next().unwrap() as i32).signum(), (r - col_bounds[c].0) as i32),
                _ => panic!("Invalid direction {}!", facing),
            };

            let arr = slice.to_vec();
            while steps > 0 {
                if move_ind + dx >= 0 && move_ind + dx < arr.len() as i32 {
                    if arr[(move_ind + dx) as usize].eq(&'#') {
                        break;
                    }
                    move_ind += dx;
                } else {
                    if move_ind + dx < 0 {
                        if facing.eq("L") {
                            // For 1 going left (top row): going right on 4 (bottom row)
                            if r < 50 && grid[[149 - r, 0]] == '.' {
                                r = 149 - r;
                                c = 0;
                                facing = "R";
                                steps -= 1;
                                continue 'a;
                            }
                            // For 3 going left (0, 0): going down on 4 (0, 0)
                            else if 50 <= r && r < 100 && grid[[100, r - 50]] == '.' {
                                c = r - 50;
                                r = 100;
                                facing = "D";
                                steps -= 1;
                                continue 'a;
                            }
                            // For 4 going left (top row): going right on 1 (bottom row)
                            else if 100 <= r && r < 150 && grid[[149 - r, 50]] == '.' {
                                r = 149 - r;
                                c = 50;
                                facing = "R";
                                steps -= 1;
                                continue 'a;
                            }
                            // For 6 going left (top row): going down on 1 (top-left corner)
                            else if 150 <= r && r < 200 && grid[[0, r - 100]] == '.' {
                                c = r - 100;
                                r = 0;
                                facing = "D";
                                steps -= 1;
                                continue 'a;
                            }
                            else {
                                break;
                            }
                        }
                        if facing.eq("U") {
                            // For 1 going up (0, 0): going right on 6 (0, 0)
                            if r < 50 && c < 100 && grid[[100 + c, 0]] == '.' {
                                r = 100 + c;
                                c = 0;
                                facing = "R";
                                steps -= 1;
                                continue 'a;
                            }
                            // For 2 going up (0, 0): going up on 6 (bottom-left corner)
                            else if r < 50 && 100 <= c && c < 150 && grid[[199, c - 100]] == '.' {
                                r = 199;
                                c = c - 100;
                                facing = "U";
                                steps -= 1;
                                continue 'a;
                            }
                            // For 4 going up (0, 0): going right on 3 (top row)
                            else if 100 <= r && r < 150 && c < 50 && grid[[50 + c, 50]] == '.' {
                                r = 50 + c;
                                c = 50;
                                facing = "R";
                                steps -= 1;
                                continue 'a;
                            }
                            else {
                                break;
                            }
                        }
                    }
                    else if move_ind + dx >= arr.len() as i32 {
                        if facing.eq("R") {
                            // For 2 going right (top row): going left on 5 (bottom row)
                            if r < 50 && c >= 100 && grid[[149 - r, 99]] == '.' {
                                r = 149 - r;
                                c = 99;
                                facing = "L";
                                steps -= 1;
                                continue 'a;
                            }
                            // For 3 going right (top row): going up on 2 (bottom-left corner)
                            else if 50 <= r && r < 100 && grid[[49, r + 50]] == '.' {
                                c = r + 50;
                                r = 49;
                                facing = "U";
                                steps -= 1;
                                continue 'a;
                            }
                            // For 5 going right (top row): going left on 2 (bottom row)
                            else if 100 <= r && r < 150 && grid[[149 - r, 149]] == '.' {
                                r = 149 - r;
                                c = 149;
                                facing = "L";
                                steps -= 1;
                                continue 'a;
                            }
                            // For 6 going right (top row): going up on 5 (bottom-left corner)
                            else if 150 <= r && r < 200 && grid[[149, r - 100]] == '.' {
                                c = r - 100;
                                r = 149;
                                facing = "U";
                                steps -= 1;
                                continue 'a;
                            }
                            else {
                                break;
                            }
                        }
                        if facing.eq("D") {
                            // For 2 going down (bottom-left corner): going left on 3 (top-right corner)
                            if r < 50 && c >= 100 && grid[[c - 50, 99]] == '.' {
                                r = c - 50;
                                c = 99;
                                facing = "L";
                                steps -= 1;
                                continue 'a;
                            }
                            // For 5 going down (bottom-left corner): going left on 6 (top row)
                            else if 100 <= r && r < 150 && c >= 50 && grid[[c + 100, 49]] == '.' {
                                r = c + 100;
                                c = 49;
                                facing = "L";
                                steps -= 1;
                                continue 'a;
                            }
                            // For 6 going down (top-left corner): going down on 2 (top-left corner)
                            else if 150 <= r && r < 200 && grid[[0, c + 100]] == '.' {
                                c = c + 100;
                                r = 0;
                                facing = "D";
                                steps -= 1;
                                continue 'a;
                            }
                            else {
                                break;
                            }
                        }
                    }
                }
                steps -= 1;
            }

            match facing {
                "R" | "L" => c = (move_ind + row_bounds[r].0 as i32) as usize,
                "U" | "D" => r = (move_ind + col_bounds[c].0 as i32) as usize,
                _ => panic!("Invalid direction {}!", facing),
            }
    
            match direction {
                "R" => facing = match facing {
                    "R" => "D",
                    "D" => "L",
                    "L" => "U",
                    "U" => "R",
                    _ => panic!("Invalid direction {}!", facing),
                },
                "L" => facing = match facing {
                    "R" => "U",
                    "U" => "L",
                    "L" => "D",
                    "D" => "R",
                    _ => panic!("Invalid direction {}!", facing),
                },
                _ => (),
            }

            break 'a;
        }
    });

    println!("Part 2: {}", 1000 * (r + 1) + 4 * (c + 1) + vec!["R", "D", "L", "U"].iter().position(|&x| x == facing).unwrap());
}

/*
The unwrapped cube:

 12
 3
45
6

For 1 going up (0, 0): going right on 6 (0, 0)
For 4 going up (0, 0): going right on 3 (top row)
For 2 going up (0, 0): going up on 6 (bottom-left corner)
For 3 going left (0, 0): going down on 4 (0, 0)
For 6 going down (top-left corner): going down on 2 (top-left corner)
For 3 going right (top row): going up on 2 (bottom-left corner)
For 1 going left (top row): going right on 4 (bottom row)
For 4 going left (top row): going right on 1 (bottom row)
For 6 going left (top row): going down on 1 (top-left corner)
For 6 going right (top row): going up on 5 (bottom-left corner)
For 2 going right (top row): going left on 5 (bottom row)
For 5 going right (top row): going left on 2 (bottom row)
For 5 going down (bottom-left corner): going left on 6 (top row)
For 2 going down (bottom-left corner): going left on 3 (top-right corner)


*/