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
        println!("{:?} {:?}", seg.c1, seg.c2);

        let mut x = seg.c1.x as usize;
        let mut y = seg.c1.y as usize;
        let x_end = seg.c2.x as usize;
        let y_end = seg.c2.y as usize;

        grid[y][x] += 1;
        while y != y_end || x != x_end {
            // if y1 == y2 && x1 == x2 {
            //     break;
            // }
            if x < x_end {
                x += 1;
            } else if x > x_end {
                x -= 1;
            }

            if y < y_end {
                y += 1;
            } else if y > y_end {
                y -= 1;
            }
            grid[y][x] += 1;
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
