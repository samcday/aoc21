use std::fmt::{Display, Formatter, Write};
use std::io::BufRead;

const STEPS: usize = 100;

#[derive(Debug)]
struct Octopi(Vec<Vec<u32>>);

impl Octopi {
    fn step(&mut self) -> Vec<(usize, usize)> {
        let mut flashes = vec![];

        for y in 0..self.0.len() {
            for x in 0..self.0[y].len() {
                flashes = self.adv_energy(x, y, flashes);
            }
        }

        flashes
    }

    fn adv_energy(&mut self, x: usize, y: usize, mut flashes: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        if flashes.contains(&(x, y)) {
            return flashes;
        }

        self.0[y][x] += 1;
        if self.0[y][x] > 9 {
            self.0[y][x] = 0;
            flashes.push((x, y));

            let x1 = x.saturating_sub(1);
            let x2 = std::cmp::min(self.0[y].len() - 1, x +1);
            let y1 = y.saturating_sub(1);
            let y2 = std::cmp::min(self.0.len() - 1, y + 1);

            for y in y1..=y2 {
                for x in x1..=x2 {
                    flashes = self.adv_energy(x, y, flashes);
                }
            }
        }

        flashes
    }
}

impl Display for Octopi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.iter()
            .map(|row| row.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""))
            .collect::<Vec<String>>().join("\n"))
    }
}

fn main() {
    let stdin = std::io::stdin();

    let mut octopi = Octopi(stdin.lock().lines()
        .map(|x| x.expect("read line")
            .chars()
            .map(|c| c.to_digit(10).expect("parse digit"))
            .collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>());


    println!("Before any steps:\n{}", octopi);

    let mut flash_count = 0;
    for step in 1..=STEPS {
        let flashes = octopi.step();
        flash_count += flashes.len();
        println!("\nAfter step {}:\n{} (flashes at: {:?})", step, octopi, flashes);
    }

    println!("Total flashes {}", flash_count);
}
