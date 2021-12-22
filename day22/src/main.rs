use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();

    let mut on_cubes: HashSet<(i32, i32, i32)> = HashSet::new();

    'line: for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut split = line.split(" ");
        let on = split.next().unwrap() == "on";
        let coords = split.next().unwrap().split(",");

        let mut parsed_ranges = ((0, 0), (0, 0), (0, 0));
        for coord in coords {
            let mut split = coord.split("=");
            let axis = split.next().unwrap();
            assert_eq!(axis.len(), 1);
            let axis = axis.chars().next().unwrap();

            let mut range = split.next().unwrap().split("..");
            let mut lower = range.next().unwrap().parse::<i32>().unwrap();
            let mut upper = range.next().unwrap().parse::<i32>().unwrap();

            if (lower < -50 || lower > 50) && (upper < -50 || upper > 50) {
                continue 'line;
            }

            // lower = std::cmp::min(50, std::cmp::max(-50, lower));
            // upper = std::cmp::max(-50, std::cmp::min(50, upper));

            match axis {
                'x' => parsed_ranges.0 = (lower, upper),
                'y' => parsed_ranges.1 = (lower, upper),
                'z' => parsed_ranges.2 = (lower, upper),
                _ => unreachable!(),
            }
        }

        // println!("Hmm. {:?}", parsed_ranges);
        // continue;

        let mut cubes_on = 0;
        let mut cubes_off = 0;
        for x in parsed_ranges.0.0..=parsed_ranges.0.1 {
            for y in parsed_ranges.1.0..=parsed_ranges.1.1 {
                for z in parsed_ranges.2.0..=parsed_ranges.2.1 {
                    let tuple = (x, y, z);
                    if on && !on_cubes.contains(&tuple) {
                        // println!("Turning on cube {},{},{}", x, y, z);
                        on_cubes.insert(tuple);
                        cubes_on += 1;
                    } else if !on && on_cubes.contains(&tuple) {
                        // println!("Turning off cube {},{},{}", x, y, z);
                        on_cubes.remove(&tuple);
                        cubes_off += 1;
                    }
                }
            }
        }

        println!("Turned on {} cubes, turned off {} cubes\n", cubes_on, cubes_off);
    }

    println!("Total lit cubes: {}", on_cubes.len());
}
