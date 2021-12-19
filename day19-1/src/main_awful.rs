use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Hash, Eq, Ord, PartialOrd)]
struct Loc {
    x: i32,
    y: i32,
    z: i32,
}

impl Loc {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn distance(&self, other: &Self) -> f32 {
        (
            (other.x as f32 - self.x as f32).powf(2.) +
            (other.y as f32 - self.y as f32).powf(2.) +
            (other.z as f32 - self.z as f32).powf(2.)
        ).sqrt()
    }
}

impl Display for Loc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    reports: Vec<Loc>,
}

impl Scanner {
    fn new() -> Self {
        Self { reports: vec![] }
    }

    fn visualize(&self) -> String {
        let mut s = String::new();

        let x1 = std::cmp::min(0, self.reports.iter().map(|l| l.x).min().unwrap());
        let x2 = std::cmp::max(0, self.reports.iter().map(|l| l.x).max().unwrap());
        let y1 = std::cmp::min(0, self.reports.iter().map(|l| l.y).min().unwrap());
        let y2 = std::cmp::max(0, self.reports.iter().map(|l| l.y).max().unwrap());

        for y in (y1..=y2).rev() {
            for x in x1..=x2 {
                s.push(if x == 0 && y == 0 {
                    'S'
                } else if self.reports.contains(&Loc::new(x, y, 0)) {
                    'B'
                } else {
                    '.'
                });
            }
            s.push('\n')
        }

        s
    }

    fn probe_distances(&self) -> HashSet<(Loc, Loc, String)> {
        let mut distances = HashSet::new();

        for perm in self.reports.iter().permutations(2) {
            let (loc1, loc2) = (perm[0], perm[1]);
            let (loc1, loc2) = (
                std::cmp::min(loc1, loc2).clone(),
                std::cmp::max(loc1, loc2).clone(),
            );
            let distance = loc1.distance(&loc2);
            distances.insert((loc1, loc2, distance.to_string()));
        }
        distances
    }
}

fn parse_input(lines: &Vec<String>) -> Vec<Scanner> {
    let mut scanners = vec![];
    let mut scanner = None;

    for line in lines {
        if line.starts_with("--- scanner ") {
            let prev = scanner.replace(Scanner::new());
            if prev.is_some() {
                scanners.push(prev.unwrap());
            }
            continue;
        }
        if line.is_empty() {
            continue;
        }
        if !scanner.is_some() {
            panic!("malformed input");
        }

        let mut split = line.split(",");

        let x = split.next().unwrap().parse::<i32>().unwrap();
        let y = split.next().unwrap().parse::<i32>().unwrap();
        let mut z = 0;
        if let Some(v) = split.next() {
            z = v.parse::<i32>().unwrap();
        }

        scanner.as_mut().unwrap().reports.push(Loc::new(x, y, z));
    }

    if scanner.is_some() {
        scanners.push(scanner.unwrap());
    }

    scanners
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::{Loc, parse_input};

    #[test]
    fn first_example() {
        let lines = indoc! {"
            --- scanner 0 ---
            0,2
            4,1
            3,3

            --- scanner 1 ---
            -1,-1
            -5,0
            -2,1
        "}.split("\n").map(|x| x.to_string()).collect::<Vec<String>>();

        let scanners = parse_input(&lines);
        assert_eq!(scanners.len(), 2);

        assert_eq!(scanners[0].reports.len(), 3);
        assert_eq!(scanners[0].reports[0], Loc::new(0, 2, 0));
        assert_eq!(scanners[0].reports[1], Loc::new(4, 1, 0));
        assert_eq!(scanners[0].reports[2], Loc::new(3, 3, 0));

        assert_eq!(scanners[1].reports.len(), 3);
        assert_eq!(scanners[1].reports[0], Loc::new(-1, -1, 0));
        assert_eq!(scanners[1].reports[1], Loc::new(-5, 0, 0));
        assert_eq!(scanners[1].reports[2], Loc::new(-2, 1, 0));

        let expected_visual = indoc!{"
            ...B.
            B....
            ....B
            S....
        "};
        assert_eq!(scanners[0].visualize(), expected_visual);

        let expected_visual = indoc!{"
            ...B..
            B....S
            ....B.
        "};
        assert_eq!(scanners[1].visualize(), expected_visual);
    }

    #[test]
    fn test_loc_distance() {
        let loc1 = Loc::new(0,0,0);
        let loc2 = Loc::new(686,422,578);
        assert_eq!(loc1.distance(&loc2), 991.344542);
    }
}

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<String>>();

    let scanners = parse_input(&lines);

    // I need to:
    // Use the distance heuristic to determine if two scanners overlap.
    // They overlap if there's at least 12 locations in common.
    // Once it's determined two scanners overlap, a common coordinate plane needs to be established.
    // When that's established, the actual overlapping beacons can be determined and counted up.

    let mut scanner_distances = HashMap::new();
    let mut distance_pairs = HashMap::new();
    for (idx, scanner) in scanners.iter().enumerate() {
        for (loc1, loc2, distance) in scanner.probe_distances().iter() {
            // let entry = common_distances.entry(distance.to_string()).or_insert(HashMap::new());
            // let entry = entry.entry(idx).or_insert(HashSet::new());
            // entry.insert(loc1.clone());
            // entry.insert(loc2.clone());
            scanner_distances.entry(idx).or_insert(HashSet::new()).insert(distance.clone());
            let k = (idx, distance.clone());
            assert!(!distance_pairs.contains_key(&k));
            distance_pairs.insert(k, (loc1.clone(), loc2.clone()));
        }
    }

    let mut scanner_transforms: HashMap<usize, (usize, i32, i32, i32)> = HashMap::new();

    // let mut checked_pairs = HashSet::new();
    'scanner: for scanner_pair in (0..scanners.len()).permutations(2) {
        if scanner_pair[1] == 0 {
            continue;
        }

        let (scanner1, scanner2) = (scanner_pair[0], scanner_pair[1]);

        // let (scanner1, scanner2) = pair;

        if scanner_transforms.contains_key(&scanner2) {
            continue;
        }

        let common_distances = scanner_distances[&scanner1].intersection(&scanner_distances[&scanner2]).cloned().collect::<Vec<String>>();

        let mut scanner1_locs = HashSet::new();
        let mut scanner2_locs = HashSet::new();
        for distance in &common_distances {
            let v = &distance_pairs[&(scanner1, distance.clone())];
            scanner1_locs.insert(v.0.clone());
            scanner1_locs.insert(v.1.clone());

            let v = &distance_pairs[&(scanner2, distance.clone())];
            scanner2_locs.insert(v.0.clone());
            scanner2_locs.insert(v.1.clone());
        }

        if scanner1_locs.len() >= 12 {
            // Take a pair of locations with matching distance from each of the pair of scanners.
            let distance = common_distances.first().unwrap();

            let scanner1_pair = &distance_pairs[&(scanner1, distance.clone())];
            let scanner2_pair = &distance_pairs[&(scanner2, distance.clone())];

            // We now try all permutations of subtracting the scanner2 pair from scanner1.
            // That is, we take the permutations of the individual locations (we know the distances
            // match, but we don't know which end is which). For each of these, we further permute
            // the 24 different ways the axes can be fucked up.
            // We test if a permutation is correct by applying it to all remaining points in
            // scanner2 and seeing if the set completely matches the points in scanner1.

            // There's only supposed to be 24 of these, but I couldn't be fucked figuring out
            // why. There's significantly more possible permutations. The ones that shouldn't
            // belong here will just never result in a valid computed result, so whatever.
            fn axis_permutations(loc: &Loc, num: usize) -> Loc {
                match num {
                    0 => Loc::new( loc.x,  loc.y,  loc.z),
                    1 => Loc::new( loc.x, -loc.y,  loc.z),
                    2 => Loc::new( loc.x, -loc.y, -loc.z),
                    3 => Loc::new( loc.x,  loc.y, -loc.z),
                    4 => Loc::new(-loc.x,  loc.y,  loc.z),
                    5 => Loc::new(-loc.x, -loc.y,  loc.z),
                    6 => Loc::new(-loc.x, -loc.y, -loc.z),
                    7 => Loc::new(-loc.x,  loc.y, -loc.z),
                    8 => Loc::new( loc.x,  loc.z,  loc.y),
                    9 => Loc::new( loc.x, -loc.z,  loc.y),
                    10 => Loc::new( loc.x, -loc.z, -loc.y),
                    11 => Loc::new( loc.x,  loc.z, -loc.y),
                    12 => Loc::new(-loc.x,  loc.z,  loc.y),
                    13 => Loc::new(-loc.x, -loc.z,  loc.y),
                    14 => Loc::new(-loc.x, -loc.z, -loc.y),
                    15 => Loc::new(-loc.x,  loc.z, -loc.y),
                    16 => Loc::new( loc.y,  loc.x,  loc.z),
                    17 => Loc::new( loc.y, -loc.x,  loc.z),
                    18 => Loc::new( loc.y, -loc.x, -loc.z),
                    19 => Loc::new( loc.y,  loc.x, -loc.z),
                    20 => Loc::new(-loc.y,  loc.x,  loc.z),
                    21 => Loc::new(-loc.y, -loc.x,  loc.z),
                    22 => Loc::new(-loc.y, -loc.x, -loc.z),
                    23 => Loc::new(-loc.y,  loc.x, -loc.z),
                    24 => Loc::new( loc.y,  loc.z,  loc.x),
                    25 => Loc::new( loc.y, -loc.z,  loc.x),
                    26 => Loc::new( loc.y, -loc.z, -loc.x),
                    27 => Loc::new( loc.y,  loc.z, -loc.x),
                    28 => Loc::new(-loc.y,  loc.z,  loc.x),
                    29 => Loc::new(-loc.y, -loc.z,  loc.x),
                    30 => Loc::new(-loc.y, -loc.z, -loc.x),
                    31 => Loc::new(-loc.y,  loc.z, -loc.x),
                    32 => Loc::new( loc.z,  loc.x,  loc.y),
                    33 => Loc::new( loc.z, -loc.x,  loc.y),
                    34 => Loc::new( loc.z, -loc.x, -loc.y),
                    35 => Loc::new( loc.z,  loc.x, -loc.y),
                    36 => Loc::new(-loc.z,  loc.x,  loc.y),
                    37 => Loc::new(-loc.z, -loc.x,  loc.y),
                    38 => Loc::new(-loc.z, -loc.x, -loc.y),
                    39 => Loc::new(-loc.z,  loc.x, -loc.y),
                    40 => Loc::new( loc.z,  loc.y,  loc.z),
                    41 => Loc::new( loc.z, -loc.y,  loc.z),
                    42 => Loc::new( loc.z, -loc.y, -loc.z),
                    43 => Loc::new( loc.z,  loc.y, -loc.z),
                    44 => Loc::new(-loc.z,  loc.y,  loc.z),
                    45 => Loc::new(-loc.z, -loc.y,  loc.z),
                    46 => Loc::new(-loc.z, -loc.y, -loc.z),
                    47 => Loc::new(-loc.z,  loc.y, -loc.z),
                    _ => unreachable!()
                }
            }

            let pair_combos = [
                (scanner1_pair.0.clone(), scanner2_pair.0.clone()),
                (scanner1_pair.0.clone(), scanner2_pair.1.clone()),
                (scanner1_pair.1.clone(), scanner2_pair.0.clone()),
                (scanner1_pair.1.clone(), scanner2_pair.1.clone()),
            ];

            for (l, r) in pair_combos {
                for axis_permute_num in 0..48 {
                    let r = axis_permutations(&r, axis_permute_num);

                    let x = l.x - r.x;
                    let y = l.y - r.y;
                    let z = l.z - r.z;

                    let mut hail_mary = HashSet::new();
                    for loc in &scanner2_locs {
                        let mut sweet_jesus = axis_permutations(loc, axis_permute_num);
                        sweet_jesus.x += x;
                        sweet_jesus.y += y;
                        sweet_jesus.z += z;
                        hail_mary.insert(sweet_jesus);
                    }

                    if hail_mary.intersection(&scanner1_locs).count() == 12 {
                        scanner_transforms.insert(scanner2, (scanner1, x, y, z));
                        println!("Scanner {} solved relative to {}: {} {} {}", scanner2, scanner1, x, y, z);
                        continue 'scanner;
                    }
                }
            }
        }
    }

    for scanner_num in 1..scanners.len() {
        if !scanner_transforms.contains_key(&scanner_num) {
            panic!("your solution sucks bruh! {} is missing", scanner_num);
        }
    }

    let mut final_map = HashMap::new();

    for loc in &scanners[0].reports {
        *final_map.entry(loc.clone()).or_insert(0) += 1;
    }

    for idx in 1..scanners.len() {
        let mut transform_x = 0;
        let mut transform_y = 0;
        let mut transform_z = 0;

        let mut scanner_transform = &scanner_transforms[&idx];
        loop {
            // println!("lol cunt: {}");
            transform_x += scanner_transform.1;
            transform_y += scanner_transform.2;
            transform_z += scanner_transform.3;
            if scanner_transform.0 == 0 {
                break;
            }
            scanner_transform = &scanner_transforms[&scanner_transform.0];
        }

        println!("Transforming scanner {} by {},{},{}", idx, transform_x, transform_y, transform_z);

        for loc in &scanners[idx].reports {
            let transformed_loc = Loc::new(loc.x + transform_x, loc.y + transform_y, loc.z + transform_z);
            *final_map.entry(transformed_loc.clone()).or_insert(0) += 1;
        }
    }
    //
    // for (k, v) in final_map {
    //     println!("{} = {}", k, v);
    // }

    // final_map.retain(|k, v| *v > 1);
    //
    // println!("pls god: {}", final_map.len());
}
