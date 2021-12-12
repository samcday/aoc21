use std::collections::HashMap;
use std::io::{BufRead, Error};

#[derive(Debug)]
struct Cave {
    is_big: bool,
    connected_to: Vec<String>,
}

impl Cave {
    fn new<T: ToString>(label: &T) -> Cave {
        let label = label.to_string();
        let is_big = label.chars().all(|x| x.is_uppercase());
        Cave{
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
        path.push(current.clone());


        let mut permutations = vec![];

        println!("calculate_route: {} {:?}", current.clone(), path.clone());

        for connection in &cave.connected_to {
            if connection == "end" {
                path.push("end".to_string());
                permutations.push(path.clone());
            }

            let connected_cave = cave_system.get(connection).unwrap();
            if path.contains(connection) && !connected_cave.is_big {
                continue;
            } else {
                permutations.extend(calculate_route(cave_system, connection.clone(), path.clone()));
            }
        }

        permutations
    }

    let permutations = calculate_route(&cave_system, "start".to_string(), vec![]);


    println!("Caves: {:?}\nPaths:\n{}\nTotal paths: {}", cave_system, permutations.iter().map(|x| x.join(",")).collect::<Vec<String>>().join("\n"), permutations.len());
    Ok(())
}
