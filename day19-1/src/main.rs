// I'm not ashamed to admit this implementation is a rip-off of this code:
// https://github.com/LinAGKar/advent-of-code-2021-rust/blob/main/day19a/src/main.rs
// I spent 8+ hours today on what I now deem a "failed" solution.
// I think if I had have persevered for a little longer I might have managed to solve it that way.
// But then I would have hunted down the person who designed AoC and blown my brains out all over
// them. So.

use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use itertools::{Itertools, max};

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

#[derive(Debug, Clone)]
struct Scanner {
    beacons: HashSet<Loc>,
    axis: i32,
    correction: (i32, i32, i32),
}

impl Scanner {
    fn new() -> Self {
        Self { beacons: HashSet::new(), axis: 0, correction: (0, 0, 0) }
    }

    fn change_axis(&mut self) {
        match self.axis {
            0|1|2|3 => {
                self.beacons = self.beacons.iter().map(|beacon| {
                    Loc::new(beacon.z, beacon.y, -beacon.x)
                }).collect::<HashSet<Loc>>();
            }
            4 => {
                self.beacons = self.beacons.iter().map(|beacon| {
                    Loc::new(beacon.x, -beacon.z, beacon.y)
                }).collect::<HashSet<Loc>>();
            }
            5 => {
                self.beacons = self.beacons.iter().map(|beacon| {
                    Loc::new(beacon.x, -beacon.y, -beacon.z)
                }).collect::<HashSet<Loc>>();
            }
            _ => unreachable!(),
        }
        self.axis += 1;
    }

    fn rotate_axis(&mut self) {
        self.beacons = self.beacons.iter().map(|beacon| {
            Loc::new(-beacon.y, beacon.x, beacon.z)
        }).collect::<HashSet<Loc>>();
    }

    fn all_match(&mut self, other: &HashSet<Loc>) -> bool {
        let mut distances = HashMap::new();

        for b1 in other {
            for b2 in &self.beacons {
                let direction = (b2.x - b1.x, b2.y - b1.y, b2.z - b1.z);
                let entry = distances.entry(direction.clone()).or_insert(0);
                *entry += 1;
                if *entry == 4 {
                    self.correction = direction;
                    return true;
                }
            }
        }
        // println!("bummer. {:?}", distances);
        false
    }

    fn align(&mut self) {
        self.beacons = self.beacons.iter().map(|beacon| {
            Loc::new(beacon.x - self.correction.0, beacon.y - self.correction.1, beacon.z - self.correction.2)
        }).collect::<HashSet<Loc>>();
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

        scanner.as_mut().unwrap().beacons.insert(Loc::new(x, y, z));
    }

    if scanner.is_some() {
        scanners.push(scanner.unwrap());
    }

    scanners
}

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<String>>();

    let mut scanners = parse_input(&lines);

    let mut all_beacons = HashSet::new();
    let scanner_0 = scanners.remove(0);
    all_beacons.extend(scanner_0.beacons.iter().cloned());

    let mut solved_scanners = vec![scanner_0];

    while !scanners.is_empty() {
        let mut scanner = scanners.remove(0);

        let mut matched = false;

        'axis: for _ in 0..6 {
            scanner.change_axis();
            for _ in 0..4 {
                scanner.rotate_axis();
                if scanner.all_match(&all_beacons) {
                    scanner.align();
                    all_beacons.extend(scanner.beacons.iter().cloned());
                    matched = true;
                    solved_scanners.push(scanner.clone());
                    break 'axis;
                }
            }
        }

        if !matched {
            println!("failed");
            scanner.axis = 0;
            scanners.push(scanner);
        }
    }

    let mut max_manhattan = 0;
    for scanners in solved_scanners.iter().permutations(2) {
        let (scanner1, scanner2) = (scanners[0], scanners[1]);
        let manhattan_distance = (scanner2.correction.0 - scanner1.correction.0).abs() +
            (scanner2.correction.1 - scanner1.correction.1).abs() +
            (scanner2.correction.2 - scanner1.correction.2).abs();
        max_manhattan = std::cmp::max(max_manhattan, manhattan_distance);
    }

    println!("{} {}", all_beacons.len(), max_manhattan);
}
