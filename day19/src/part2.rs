use std::{fs::read_to_string, collections::HashMap};
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct RoboCost {
    ore: u32,
    clay: u32,
    obsidian: u32,
}

pub fn main() {
    let re = Regex::new(r"Blueprint (?P<id>[0-9]*): Each ore robot costs (?P<ore_robot>[0-9]*) ore. Each clay robot costs (?P<clay_robot>[0-9]*) ore. Each obsidian robot costs (?P<obs_ore_robot>[0-9]*) ore and (?P<obs_clay_robot>[0-9]*) clay. Each geode robot costs (?P<geo_ore_robot>[0-9]*) ore and (?P<geo_obs_robot>[0-9]*) obsidian.").unwrap();

    let data = read_to_string("./data.txt").expect("File does not exist!");
    let captures = re.captures_iter(data.as_str());

    let mut blueprints = Vec::new();

    captures.for_each(|c| {
        let mut robots = HashMap::new();
        robots.insert("ore_robot".to_string(), RoboCost { ore: c["ore_robot"].parse().unwrap(), clay: 0, obsidian: 0 });
        robots.insert("clay_robot".to_string(), RoboCost { ore: c["clay_robot"].parse().unwrap(), clay: 0, obsidian: 0 });
        robots.insert("obsidian_robot".to_string(), RoboCost { ore: c["obs_ore_robot"].parse().unwrap(), clay: c["obs_clay_robot"].parse().unwrap(), obsidian: 0 });
        robots.insert("geode_robot".to_string(), RoboCost { ore: c["geo_ore_robot"].parse().unwrap(), clay: 0, obsidian: c["geo_obs_robot"].parse().unwrap() });

        blueprints.push(robots);
    });

    blueprints = blueprints[..3].to_vec();

    let robofleet = HashMap::from([
        ("ore_robot".to_string(), 1u32),
        ("clay_robot".to_string(), 0u32),
        ("obsidian_robot".to_string(), 0u32),
        ("geode_robot".to_string(), 0u32),
    ]);

    let resources = HashMap::from([
        ("ore".to_string(), 0u32),
        ("clay".to_string(), 0u32),
        ("obsidian".to_string(), 0u32),
        ("geode".to_string(), 0u32),
    ]);

    let max_geode = vec![0; 33];
    let mut sum = 1;

    for id in 0..blueprints.len() {
        let max_needed = blueprints[id].iter().map(|(_, v)| {
            HashMap::from([
                ("ore_robot".to_string(), v.ore),
                ("clay_robot".to_string(), v.clay),
                ("obsidian_robot".to_string(), v.obsidian),
            ])
        }).fold(HashMap::new(), |mut acc, x| {
            for (k, v) in x {
                let ks = k.clone();
                acc.insert(k, std::cmp::max(acc.get(&ks).unwrap_or(&0), &v).clone());
            }
            acc
        });

        let ql = quality_level(id as u32 + 1, &mut resources.clone(), robofleet.clone(), &blueprints[id], 1, &mut max_geode.clone(), &max_needed);
        println!("Quality level for blueprint {} is {}", id + 1, ql);
        sum *= ql;
    }

    println!("Part 2: {}", sum);
}

fn quality_level(id: u32, resources: &mut HashMap<String, u32>, robofleet: HashMap<String, u32>, blueprint: &HashMap<String, RoboCost>, minute: u32, max_geode: &mut Vec<u32>, max_needed: &HashMap<String, u32>) -> u32 {
    if minute == 33 {
        // println!("the end >> {:?} {:?} {:?}", id, resources, robofleet);
        return resources["geode"];
    }

    max_geode[minute as usize - 1] = std::cmp::max(max_geode[minute as usize - 1], resources["geode"]);
    let mut ql_after_spending = 0;
    
    // Trying to build robots
    for robot in ["ore_robot".to_string(), "clay_robot".to_string(), "obsidian_robot".to_string(), "geode_robot".to_string()] {
        let cost = blueprint.get(&robot).unwrap();

        // If we will never have enough resources to build a robot during collection time, don't build it
        if (cost.ore > 0 && robofleet[&"ore_robot".to_string()] == 0) || (cost.clay > 0 && robofleet[&"clay_robot".to_string()] == 0) || (cost.obsidian > 0 && robofleet[&"obsidian_robot".to_string()] == 0) {
            continue;
        }

        // If we have enough robots for a specific resource, don't make more
        if robot.ne("geode_robot") && (robofleet[&robot] >= max_needed[&robot] 
            || resources[&robot.replace("_robot", "")] + robofleet[&robot] * (32 - minute) >= (32 - minute) * max_needed[&robot]) {
            continue;
        }

        let mut new_resources = resources.clone();
        let mut new_robofleet = robofleet.clone();
        let mut curr_minute = minute;

        while new_resources["ore"] < cost.ore || new_resources["clay"] < cost.clay || new_resources["obsidian"] < cost.obsidian {
            collect_resources(&mut new_resources, &new_robofleet);
            curr_minute += 1;
            max_geode[curr_minute as usize - 1] = std::cmp::max(max_geode[curr_minute as usize - 1], new_resources["geode"]);
            
            if curr_minute == 33 {
                return std::cmp::max(new_resources["geode"], ql_after_spending);
            }
        }

        // println!("{:?} {:?} {:?}", new_resources, new_robofleet, curr_minute);

        collect_resources(&mut new_resources, &new_robofleet);
        curr_minute += 1;

        // Build the robot
        for (resource, c) in [("ore".to_string(), cost.ore), ("clay".to_string(), cost.clay), ("obsidian".to_string(), cost.obsidian)] {
            new_resources.insert(resource.clone(), new_resources.get(&resource).unwrap() - c);
        }

        new_robofleet.insert(robot.clone(), new_robofleet.get(&robot).unwrap() + 1);
        if max_geode[curr_minute as usize - 1] <= new_resources["geode"] {
            ql_after_spending = std::cmp::max(quality_level(id, &mut new_resources, new_robofleet, blueprint, curr_minute, max_geode, max_needed), ql_after_spending);
        }
    }
    ql_after_spending
}

fn collect_resources(resources: &mut HashMap<String, u32>, robofleet: &HashMap<String, u32>) {
    for r in ["ore".to_string(), "clay".to_string(), "obsidian".to_string(), "geode".to_string()] {
        resources.insert(r.clone(), resources.get(&r).unwrap() + robofleet[&(r + "_robot")]);
    }
}
