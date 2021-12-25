use std::collections::HashMap;
use std::io::BufRead;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Cucumber {
    SOUTH,
    EAST,
}

fn print_map(map: &HashMap<(usize, usize), Cucumber>, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            print!("{}", match map.get(&(x, y)) {
                Some(Cucumber::EAST) => '>',
                Some(Cucumber::SOUTH) => 'v',
                None => '.',
            });
        }
        println!();
    }
}

fn main() {
    let stdin = std::io::stdin();

    let mut sea_floor = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in stdin.lock().lines().enumerate() {
        let line = line.unwrap();
        let mut chars = line.chars();
        for (x, char) in chars.enumerate() {
            if char == '.' {
                continue;
            }
            sea_floor.insert((x, y), match char {
                '>' => Cucumber::EAST,
                'v' => Cucumber::SOUTH,
                _ => panic!()
            });
            width = std::cmp::max(width, x + 1);
            height = std::cmp::max(height, y + 1);
        }
    }

    let mut step = 0;
    loop {
        let east_movement = sea_floor.iter().filter_map(|(&pos, &cucumber)| {
            if cucumber != Cucumber::EAST {
                return None;
            }

            let mut new_pos = pos;
            new_pos.0 +=  1;
            if new_pos.0 == width {
                new_pos.0 = 0;
            }
            if sea_floor.contains_key(&new_pos) {
                return None;
            }

            Some((pos, new_pos))
        }).collect::<Vec<_>>();

        let mut any_moved = east_movement.len() > 0;

        for (old_pos, new_pos) in east_movement {
            let cucumber = sea_floor.remove(&old_pos).unwrap();
            sea_floor.insert(new_pos, cucumber);
        }

        let south_movement = sea_floor.iter().filter_map(|(&pos, &cucumber)| {
            if cucumber != Cucumber::SOUTH {
                return None;
            }

            let mut new_pos = pos;
            new_pos.1 +=  1;
            if new_pos.1 == height {
                new_pos.1 = 0;
            }
            if sea_floor.contains_key(&new_pos) {
                return None;
            }

            Some((pos, new_pos))
        }).collect::<Vec<_>>();

        any_moved = any_moved || south_movement.len() > 0;

        for (old_pos, new_pos) in south_movement {
            let cucumber = sea_floor.remove(&old_pos).unwrap();
            sea_floor.insert(new_pos, cucumber);
        }

        step += 1;

        println!("After {} steps", step);
        print_map(&sea_floor, width, height);

        if !any_moved {
            break;
        }

    }

    println!("Settled after {} steps", step);
}
