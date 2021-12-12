use std::collections::HashMap;
use std::io::{BufRead, Error};

#[derive(Debug)]
struct Cave {
    label: String,
    is_big: bool,
    connected_to: Vec<String>,
}

impl Cave {
    fn new<T: ToString>(label: &T) -> Cave {
        let label = label.to_string();
        let is_big = label.chars().all(|x| x.is_uppercase());
        Cave{
            label,
            is_big,
            connected_to: vec![],
        }
    }
}

fn main() -> Result<(), Error> {
    let stdin = std::io::stdin();
    let mut cave_system = HashMap::new();

    for line in stdin.lock().lines() {
        let line = line?;
        let mut split = line.split("-").map(|x| x.to_string()).collect::<Vec<String>>();
        let right = split.pop().unwrap();
        let left = split.pop().unwrap();

        if !cave_system.contains_key(&left) {
            cave_system.insert(left.clone(), Cave::new(&left));
        }

        if !cave_system.contains_key(&right) {
            cave_system.insert(right.clone(), Cave::new(&right));
        }

        cave_system.get_mut(&left).unwrap().connected_to.push(right.clone());
        cave_system.get_mut(&right).unwrap().connected_to.push(left.clone());
    }

    fn calculate_route(cave_system: &HashMap<String, Cave>, current: String, mut path: Vec<String>) -> Vec<Vec<String>> {
        let cave = cave_system.get(&current).unwrap();
        println!("calculate_route: {} {:?}", current.clone(), path.clone());
        if current == "end" {
            return vec![path];
        }

        if !cave.is_big {
            let count = path.iter().filter(|x| **x == current).count();
            if count == 2 {
                println!("skipping smol cave {} because it's already present twice", cave.label);
                return vec![];
            }

            for (_, small_cave) in cave_system.iter().filter(|(_, x)| x.label != current && !x.is_big) {
                if count == 1 && path.iter().filter(|x| **x == small_cave.label).count() > 1 {
                    println!("skipping smol cave {} because {} is already visited twice", cave.label, small_cave.label);
                    return vec![];
                }
            }
        }

        path.push(current.clone());

        let mut permutations = vec![];

        'zzz: for connection in &cave.connected_to {
            if connection == "start" {
                continue;
            }

            let connected_cave = cave_system.get(connection).unwrap();

            // This is a small cave, check that:
            // We've not visited it more than twice, and.
            // We've not visited any other small cave more than once.
            if !connected_cave.is_big {
                println!("{} ain't big", connected_cave.label);
                let count = path.iter().filter(|x| *x == connection).count();
                if count == 2 {
                    println!("skipping smol cave {} because it's already present twice", connected_cave.label);
                    continue;
                }

                for (_, small_cave) in cave_system.iter().filter(|(_, x)| x.label != current && !x.is_big) {
                    if count == 1 && path.iter().filter(|x| **x == small_cave.label).count() > 1 {
                        println!("skip");
                        continue 'zzz;
                    }
                }
            }

            permutations.extend(calculate_route(cave_system, connection.clone(), path.clone()));
        }

        permutations
    }

    let mut permutations = calculate_route(&cave_system, "start".to_string(), vec![]);
    permutations.sort();

    println!("Caves: {:?}\nPaths:\n{}\nTotal paths: {}", cave_system, permutations.iter().map(|x| x.join(",")).collect::<Vec<String>>().join("\n"), permutations.len());
    Ok(())
}
