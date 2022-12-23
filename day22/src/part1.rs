use std::fs::read_to_string;
use regex::Regex;
use ndarray::{Array2, s};

pub fn main() {
    let f = read_to_string("./data.txt")
        .expect("File does not exist!");
    let data = f
        .split("\r\n\r\n")
        .collect::<Vec<&str>>();
    
    let (height, width) = (200, 150); // (12, 16)

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
    let (mut r, mut c, mut facing) = (0, data[0].find('.').unwrap(), "R");

    captures.for_each(|cap| {
        let mut steps = cap.name("steps").unwrap().as_str().parse::<usize>().unwrap();
        let direction = cap.name("direction").unwrap().as_str();

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
                if move_ind + dx < 0 && arr.last().unwrap().eq(&'.') {
                    move_ind = arr.len() as i32 - 1;
                }
                else if move_ind + dx >= arr.len() as i32 && arr.first().unwrap().eq(&'.') {
                    move_ind = 0;
                }
                else {
                    break;
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
    });

    println!("Part 1: {}", 1000 * (r + 1) + 4 * (c + 1) + vec!["R", "D", "L", "U"].iter().position(|&x| x == facing).unwrap());
}
