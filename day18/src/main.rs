use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    sides: i32,
}

impl Point {
    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            sides: 6,
        }
    }
}

fn main() {
    let mut points = read_to_string("./data.txt")
        .expect("File does not exist!")
        .lines()
        .map(|line| line.trim().split(",").collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .iter()
        .map(|line| Point {
            x: line[0].parse().unwrap(),
            y: line[1].parse().unwrap(),
            z: line[2].parse().unwrap(),
            sides: 6,
        })
        .collect::<Vec<_>>();

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            update_sides_if_close_by(&mut points, i, j);
        }
    }

    let part1 = points.iter().fold(0, |acc, p| acc + p.sides);
    println!("Part 1: {}", part1);

    points.iter_mut().for_each(|p| p.sides = 6);

    // Generate a vector of 3d points that are 1 unit away from (0, 0, 0)
    // not including diagonals
    let mut points_1_away = Vec::new();
    for x in -1i32..2i32 {
        for y in -1i32..2i32 {
            for z in -1i32..2i32 {
                if x.abs() + y.abs() + z.abs() == 1 {
                    points_1_away.push(Point { x, y, z, sides: 6 });
                }
            }
        }
    }

    let points_set = points.iter().map(|p| p.clone()).collect::<HashSet<Point>>();

    let mut num_sides_enclosed = 0;
    let mut visited_enclosed = HashSet::new();
    for i in 0..points.len() {
        // Pick the side of the point and check if it is enclosed
        for j in 0..points_1_away.len() {
            let new_point = points[i].add(&points_1_away[j]);
            let mut visited = HashSet::new();
            num_sides_enclosed = if !visited_enclosed.contains(&new_point)
                && !points_set.contains(&new_point)
                && is_enclosed(
                    &new_point,
                    &points_set,
                    &points,
                    &points_1_away,
                    0,
                    &mut visited,
                ) {
                visited_enclosed.extend(visited);
                num_sides_enclosed + 1
            } else if visited_enclosed.contains(&new_point) {
                num_sides_enclosed + 1
            } else {
                num_sides_enclosed
            }
        }
    }

    println!("Part 2: {}", part1 - num_sides_enclosed);
}

fn is_enclosed(
    p:             &Point,
    points_set:    &HashSet<Point>,
    points:        &Vec<Point>,
    points_1_away: &Vec<Point>,
    count:         i32,
    visited:       &mut HashSet<Point>,
) -> bool {
    visited.insert(p.clone());
    let mut _count = count;
    if count == 1000 {
        return false;
    }
    for i in 0..points_1_away.len() {
        let new_point = p.add(&points_1_away[i]);
        if !points_set.contains(&new_point) && !visited.contains(&new_point) {
            if !is_enclosed(
                &new_point,
                &points_set,
                &points,
                &points_1_away,
                _count + 1,
                visited,
            ) {
                return false;
            }
        }
    }
    true
}

fn update_sides_if_close_by(points: &mut Vec<Point>, i: usize, j: usize) {
    let p1 = &points[i];
    let p2 = &points[j];
    if (p1.x - p2.x).abs() + (p1.y - p2.y).abs() + (p1.z - p2.z).abs() <= 1 {
        points[i].sides -= 1;
        points[j].sides -= 1;
    }
}
