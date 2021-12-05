use std::cmp::{max, min};
use std::fmt::Error;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug)]
struct Coord {
    x: u32,
    y: u32,
}

impl FromStr for Coord {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<u32> = s
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        Ok(Coord{x: coords[0], y: coords[1]})
    }
}

#[derive(Debug)]
struct LineSegment {
    c1: Coord,
    c2: Coord,
}

impl FromStr for LineSegment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(" -> ");
        Ok(LineSegment{
            c1: it.next().unwrap().parse()?,
            c2: it.next().unwrap().parse()?,
        })
    }
}

fn main() {
    let stdin = std::io::stdin();

    let lineSegments: Vec<LineSegment> = stdin.lock().lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();

    const GRID_W: usize = 1000;
    const GRID_H: usize = 1000;
    let mut grid = [[0; GRID_W]; GRID_H];

    for seg in lineSegments {
        let x1 = min(seg.c1.x, seg.c2.x) as usize;
        let x2 = max(seg.c1.x, seg.c2.x) as usize;
        let y1 = min(seg.c1.y, seg.c2.y) as usize;
        let y2 = max(seg.c1.y, seg.c2.y) as usize;

        if x1 != x2 && y1 != y2 {
            continue;
        }

        println!("{:?} {:?} {} {} {} {}", seg.c1, seg.c2, x1, x2, y1, y2);

        for y in y1..=y2 {
            for x in x1..=x2 {
                grid[y][x] += 1;
            }
        }
    }

    let mut overlaps = 0;
    for y in 0..GRID_H {
        for x in 0..GRID_W {
            print!("{}", if grid[y][x] > 0 { grid[y][x].to_string() } else { ".".to_string() });
            if grid[y][x] > 1 {
                overlaps += 1;
            }
        }
        println!();
    }
    println!("Total overlaps: {}", overlaps);
}
