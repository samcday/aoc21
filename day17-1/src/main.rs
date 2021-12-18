use std::collections::HashSet;
use std::io::BufRead;

struct Probe {
    x: i32,
    y: i32,
    x_vel: i32,
    y_vel: i32,
}

impl Iterator for Probe {
    type Item = ((i32, i32), (i32, i32));

    fn next(&mut self) -> Option<Self::Item> {
        self.x += self.x_vel;
        self.y += self.y_vel;
        if self.x_vel > 0 {
            self.x_vel -= 1;
        } else if self.x_vel < 0 {
            self.x_vel += 1;
        }
        self.y_vel -= 1;

        Some(((self.x, self.y), (self.x_vel, self.y_vel)))
    }
}

fn map_probe(probe: Probe, target_area: &((i32, i32), (i32, i32))) {
    let (_, probe_steps) = run_probe(probe, target_area);

    let probe_min_x = probe_steps.iter().map(|&(x, _)| x).min().unwrap();
    let probe_max_x = probe_steps.iter().map(|&(x, _)| x).max().unwrap();
    let probe_min_y = probe_steps.iter().map(|&(_, y)| y).min().unwrap();
    let probe_max_y = probe_steps.iter().map(|&(_, y)| y).max().unwrap();

    let x1 = std::cmp::min(0, std::cmp::min(target_area.0.0, probe_min_x));
    let x2 = std::cmp::max(0, std::cmp::max(target_area.1.0, probe_max_x));
    let y1 = std::cmp::min(0, std::cmp::min(target_area.0.1, probe_min_y));
    let y2 = std::cmp::max(0, std::cmp::max(target_area.1.1, probe_max_y));

    println!("{},{} {},{}", x1, y1, x2, y2);
    for y in (y1..=y2).rev() {
        for x in x1..=x2 {
            if x == 0 && y == 0 {
                print!("S");
            }
            else if probe_steps.contains(&(x, y)) {
                print!("#");
            } else if x >= target_area.0.0 && x <= target_area.1.0 && y >= target_area.0.1 && y <= target_area.1.1 {
                print!("T");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn run_probe(probe: Probe, target_area: &((i32, i32), (i32, i32))) -> (bool, Vec<(i32, i32)>) {
    let mut probe_steps = HashSet::new();
    let mut target_hit = false;

    for step in probe {
        let (x, y) = step.0;
        let (x_vel, y_vel) = step.0;

        // println!("Step: {:?}", step);

        if x_vel == 0 && (x < target_area.0.0 || x > target_area.1.0) {
            break;
        }

        if y_vel < 0 && y < target_area.0.1 {
            break;
        }

        probe_steps.insert(step.0);

        if x >= target_area.0.0 && x <= target_area.1.0 && y >= target_area.0.1 && y <= target_area.1.1 {
            target_hit = true;
            break;
        }
    }

    (target_hit, probe_steps.iter().cloned().collect())
}

fn main() {
    let stdin = std::io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let line = line.strip_suffix("\n").unwrap();

    assert!(line.starts_with("target area: x="));
    let line = line.strip_prefix("target area: x=").unwrap();
    let mut split = line.split(", ");
    let mut x_range = split.next().unwrap().split("..");

    let mut y_range = split.next().unwrap().strip_prefix("y=").unwrap().split("..");

    let target_area = {
        let x1 = x_range.next().unwrap().parse::<i32>().unwrap();
        let y1 = y_range.next().unwrap().parse::<i32>().unwrap();
        let x2 = x_range.next().unwrap().parse::<i32>().unwrap();
        let y2 = y_range.next().unwrap().parse::<i32>().unwrap();
        (
            (
                std::cmp::min(x1, x2),
                std::cmp::min(y1, y2),
            ),
            (
                std::cmp::max(x1, x2),
                std::cmp::max(y1, y2),
            ),
        )
    };

    let max_x_vel = target_area.0.0;
    let mut highest_y_pos = 0;

    for x_vel in 1..max_x_vel {
        println!("{} {}", x_vel, max_x_vel);
        for y_vel in 0..100 {
            let (hit, steps) = run_probe(Probe{x: 0, y: 0, x_vel, y_vel}, &target_area);
            if !hit {
                continue;
            }
            highest_y_pos = std::cmp::max(highest_y_pos, steps.iter().map(|&(x, y)| y).max().unwrap());
        }
    }

    println!("Highest y_pos: {}", highest_y_pos);

    // map_probe(Probe{x: 0, y: 0, x_vel: 7, y_vel: 2}, &target_area);
    // map_probe(Probe{x: 0, y: 0, x_vel: 6, y_vel: 3}, &target_area);
    // map_probe(Probe{x: 0, y: 0, x_vel: 9, y_vel: 0}, &target_area);
    // map_probe(Probe{x: 0, y: 0, x_vel: 17, y_vel: -4}, &target_area);
    // map_probe(Probe{x: 0, y: 0, x_vel: 6, y_vel: 9}, &target_area);
}
