use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let ops = read_to_string("./data.txt")
        .expect("File does not exist!")
        .lines()
        .fold(HashMap::new(), |mut map, line| {
            let mut parts = line.split(": ");
            map.entry(parts.next().unwrap().to_string())
                .or_insert(parts.next().unwrap().to_string());
            map
        });

    let mut calculated = HashMap::new();
    let mut need_humn = HashSet::new();

    println!(
        "Part 1: {}",
        calc("root".to_string(), &ops, &mut calculated, &mut need_humn)
    );
    need_humn.insert("humn".to_string());

    let (first, _, second) = parse_into_tuple(ops[&"root".to_string()].clone());

    let (val, parent) = if need_humn.contains(&first) {
        (calculated[&second], first.clone())
    } else {
        (calculated[&first], second.clone())
    };

    println!(
        "Part 2: {}",
        find_humn(parent, val, &ops, &calculated, &need_humn)
    );
}

fn calc(
    name: String,
    ops: &HashMap<String, String>,
    mut calculated: &mut HashMap<String, i64>,
    mut need_hum: &mut HashSet<String>,
) -> i64 {
    match ops[&name].parse::<i64>() {
        Ok(num) => num,
        Err(_) => {
            if calculated.contains_key(&name) {
                calculated[&name]
            } else {
                let (first, op, second) = ops[&name].split(" ").fold(
                    (String::new(), String::new(), String::new()),
                    |mut tuple, part| {
                        if tuple.0.is_empty() {
                            tuple.0 = part.to_string();
                        } else if tuple.1.is_empty() {
                            tuple.1 = part.to_string();
                        } else {
                            tuple.2 = part.to_string();
                        }
                        tuple
                    },
                );
                if !calculated.contains_key(&first) {
                    let val = calc(first.clone(), &ops, &mut calculated, &mut need_hum);
                    calculated.insert(first.clone(), val);
                }
                if !calculated.contains_key(&second) {
                    let val = calc(second.clone(), &ops, &mut calculated, &mut need_hum);
                    calculated.insert(second.clone(), val);
                }
                if first.eq("humn")
                    || second.eq("humn")
                    || need_hum.contains(&first)
                    || need_hum.contains(&second)
                {
                    need_hum.insert(name.clone());
                }
                match op.as_str() {
                    "+" => calculated[&first] + calculated[&second],
                    "*" => calculated[&first] * calculated[&second],
                    "-" => calculated[&first] - calculated[&second],
                    "/" => calculated[&first] / calculated[&second],
                    _ => panic!("Unknown operator!"),
                }
            }
        }
    }
}

fn find_humn(
    parent: String,
    val: i64,
    ops: &HashMap<String, String>,
    calculated: &HashMap<String, i64>,
    need_humn: &HashSet<String>,
) -> i64 {
    if parent.eq("humn") {
        val
    } else {
        let (first, op, second) = parse_into_tuple(ops[&parent].clone());
        if need_humn.contains(&first) {
            find_humn(
                first.clone(),
                match op.as_str() {
                    "+" => val - calculated[&second],
                    "*" => val / calculated[&second],
                    "-" => val + calculated[&second],
                    "/" => val * calculated[&second],
                    _ => panic!("Unknown operator!"),
                },
                &ops,
                &calculated,
                &need_humn,
            )
        } else {
            // if need_humn.contains(&second)
            find_humn(
                second.clone(),
                match op.as_str() {
                    "+" => val - calculated[&first],
                    "*" => val / calculated[&first],
                    "-" => calculated[&first] - val,
                    "/" => calculated[&first] / val,
                    _ => panic!("Unknown operator!"),
                },
                &ops,
                &calculated,
                &need_humn,
            )
        }
    }
}

fn parse_into_tuple(input: String) -> (String, String, String) {
    input.split(" ").fold(
        (String::new(), String::new(), String::new()),
        |mut tuple, part| {
            if tuple.0.is_empty() {
                tuple.0 = part.to_string();
            } else if tuple.1.is_empty() {
                tuple.1 = part.to_string();
            } else {
                tuple.2 = part.to_string();
            }
            tuple
        },
    )
}
