use std::io::BufRead;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();

    let heightmap = stdin.lock()
        .lines()
        .map(|x| x.unwrap().chars().map(|x| x.to_digit(10).unwrap() as usize).collect())
        .collect::<Vec<Vec<usize>>>();

    let width = heightmap[0].len();
    let height = heightmap.len();
    println!("heightmap: {:?} w={} h={}", heightmap, width, height);

    let mut lowest_points = vec![];
    let mut risk_level = 0;
    for y in 0..height {
        for (x, val) in heightmap[y].iter().copied().enumerate() {
            if x > 0 && heightmap[y][x - 1] <= val {
                continue;
            }
            if x < width - 1 && heightmap[y][x + 1] <= val {
                continue;
            }
            if y > 0 && heightmap[y - 1][x] <= val {
                continue;
            }
            if (y < height - 1) && heightmap[y + 1][x] <= val {
                continue;
            }
            lowest_points.push(val);
            risk_level += val + 1;
        }
    }

    println!("lowest points={:?} risk={}", lowest_points, risk_level);

    Ok(())
}
