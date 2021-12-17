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

fn run_probe(probe: Probe, target_area: &((i32, i32), (i32, i32))) -> (bool, Vec<(i32, i32)>) {
    let mut probe_steps = HashSet::new();
    let mut target_hit = false;

    for step in probe {
        let (x, y) = step.0;
        let (x_vel, y_vel) = step.0;

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

    let mut total_viable_options = 0;
    let max_x_vel = target_area.1.0;

    for x_vel in 1..=max_x_vel {
        for y_vel in -100..100 {
            let (hit, steps) = run_probe(Probe{x: 0, y: 0, x_vel, y_vel}, &target_area);
            if !hit {
                continue;
            }
            println!("hit: {},{}", x_vel, y_vel);
            total_viable_options += 1;
        }
    }

    println!("total: {}", total_viable_options);
}
