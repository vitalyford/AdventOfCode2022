use std::fs::read_to_string;
use ndarray::Array2;

struct Block {
    shape: Array2<char>,
    r :i32, // bottom-left row position in the room
    c: i32, // bottom-left col position in the room
}

fn main() {
    let size = (10000, 7);
    let input = read_to_string("./data.txt").expect("File does not exist!");

    let moves = input.chars().flat_map(|c| {
        match c {
            '>' => vec![(0,  1), (-1, 0)],
             _  => vec![(0, -1), (-1, 0)]
        }
    })
    .collect::<Vec<(i32, i32)>>();

    let mut room = Array2::from_shape_vec(size, vec![' '; size.0 * size.1]).unwrap();

    let mut blocks = vec![
        Block { shape: Array2::from_shape_vec((1, 4), vec!['#'; 4]).unwrap(), r: 0, c: 0, },
        Block { shape: Array2::from_shape_vec((3, 3), vec![' ', '#', ' ', '#', '#', '#', ' ', '#', ' ']).unwrap(), r: 0, c: 0, },
        Block { shape: Array2::from_shape_vec((3, 3), vec![' ', ' ', '#', ' ', ' ', '#', '#', '#', '#']).unwrap(), r: 0, c: 0, },
        Block { shape: Array2::from_shape_vec((4, 1), vec!['#'; 4]).unwrap(), r: 0, c: 0, },
        Block { shape: Array2::from_shape_vec((2, 2), vec!['#'; 4]).unwrap(), r: 0, c: 0, },
    ];

    let mut height_diffs = Vec::new();
    height_diffs.push(0);
    let mut height  = 0;
    let mut rock_count = 0;

    let mut i_move  = 0;
    let mut i_block = 0;
    'blocks: loop {
        i_block %= blocks.len();
        let mut b = &mut blocks[i_block];
        b.c = 2;
        b.r = height + 3;

        'moves: loop {
            i_move %= moves.len();
            let m = moves[i_move];

            let is_movable = can_move_then_move(&mut b, m, &room);
            
            if !is_movable {
                // Check if we need to rest
                if m == (-1, 0) {
                    land_block(&b, &mut room);
                    // 5000 should be enough to find a repeating pattern
                    if rock_count == 5000 {
                        break 'blocks;
                    }
                    if rock_count == 2022 {
                        println!("Part 1: {}", height);
                    }
                    let prev_height = height;
                    height = std::cmp::max(height, b.r + b.shape.nrows() as i32);
                    height_diffs.push(height - prev_height);
                    rock_count += 1;
                    break 'moves;
                }
            }
            i_move += 1;
        }
        i_move += 1;
        i_block += 1;
    }

    let (longest_slice, first_occur_index) = longest_repeating_slice(&height_diffs);

    let total_rocks: i64 = 1000000000000;
    let total_height: i64 = 
        height_diffs[1..first_occur_index].iter().sum::<i32>() as i64 
            + (1 + total_rocks - first_occur_index as i64) / longest_slice.len() as i64 * longest_slice.iter().sum::<i32>() as i64
            + longest_slice[0..((1 + total_rocks - first_occur_index as i64) % longest_slice.len() as i64) as usize].iter().sum::<i32>() as i64;

    println!("Part 2: {}", total_height);
}

// Returns the longest repeating slice in the vector
// and the index of the its first occurrence
fn longest_repeating_slice(vec: &[i32]) -> (&[i32], usize) {
    let mut longest_slice = &vec[0..0];
    let mut index = 0;
    for i in 0..vec.len() {
        for j in i+1..vec.len() {
            if j+j-i < vec.len() && vec[i..j] == vec[j..j+j-i] {
                if j-i > longest_slice.len() {
                    longest_slice = &vec[i..j];
                    index = i;
                }
            }
        }
    }
    (longest_slice, index)
}

fn land_block(b: &Block, room: &mut Array2<char>) {
    for b_row in 0..b.shape.nrows() {
        for b_col in 0..b.shape.ncols() {
            if b.shape.get((b_row, b_col)).unwrap().eq(&'#') {
                let dr = b.r - (b_row as i32) + (b.shape.nrows() as i32) - 1;
                let dc = b.c + (b_col as i32);
                room[(dr as usize, dc as usize)] = '#';
            }
        }
    }
}

fn can_move_then_move(b: &mut Block, m: (i32, i32), room: &Array2<char>) -> bool {
    for b_row in 0..b.shape.nrows() {
        for b_col in 0..b.shape.ncols() {
            if b.shape.get((b_row, b_col)).unwrap().eq(&'#') {
                let dr = b.r - (b_row as i32) + (b.shape.nrows() as i32) - 1 + m.0;
                let dc = b.c + (b_col as i32) + m.1;
                // Out of bounds in the room
                if dr < 0 || dc < 0 || dr >= room.nrows() as i32 || dc >= room.ncols() as i32 {
                    return false;
                }
                // Collision
                if room.get((dr as usize, dc as usize)).unwrap().eq(&'#') {
                    return false;
                }
            }
        }
    }
    b.r += m.0;
    b.c += m.1;
    true
}