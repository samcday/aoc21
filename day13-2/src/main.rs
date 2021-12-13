use std::fmt::{Display, Formatter, Write};
use std::io::{BufRead, Error};

struct Paper {
    points: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Display for Paper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                f.write_char(
                    if self.points.contains(&(x, y)) { '#' } else { '.' }
                )?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Paper {
    fn add_point(&mut self, x: usize, y: usize) {
        self.points.push((x, y));
        self.width = std::cmp::max(self.width, x + 1);
        self.height = std::cmp::max(self.height, y + 1);
    }

    fn fold_x(&mut self, x: usize) {
        if self.width % 2 != 1 {
            panic!("Folding along x={} when width {} is not odd!", x, self.width);
        }

        let reflect_x = self.width / 2;
        for (x, _) in self.points.iter_mut() {
            if *x > reflect_x {
                *x = reflect_x - (*x - reflect_x);
            }
        }
        self.width = reflect_x;
        println!("Finished folding along x={}", reflect_x);
    }

    fn fold_y(&mut self, y: usize) {
        if self.height % 2 != 1 {
            panic!("Folding along y={} when height {} is not odd!", y, self.height);
        }

        let reflect_y = self.height / 2;
        for (_, y) in self.points.iter_mut() {
            if *y > reflect_y {
                *y = reflect_y - (*y - reflect_y);
            }
        }
        self.height = reflect_y;
        println!("Finished folding along y={}", reflect_y);
    }
}

fn main() -> Result<(), Error> {
    let mut paper = Paper{points: vec![], width: 0, height: 0};
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        let mut split = line.split(",").map(|x| x.parse::<usize>().unwrap());
        let x = split.next().unwrap();
        let y = split.next().unwrap();

        paper.add_point(x, y);
    }

    for line in stdin.lock().lines() {
        let line = line?;
        assert!(line.starts_with("fold along "));
        match line.chars().nth(11).unwrap() {
            'x' => paper.fold_x(1),
            'y' => paper.fold_y(1),
            c => panic!("unexpected fold axis {}", c),
        }
    }

    println!("Final folded result: \n{}", paper);

    Ok(())
}
