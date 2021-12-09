use std::collections::HashMap;
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
            lowest_points.push((x, y));
        }
    }

    println!("lowest points={:?}", lowest_points);

    let mut marked = HashMap::new();

    fn flood_fill(heightmap: &Vec<Vec<usize>>, marked: &mut HashMap<(usize, usize), ()>, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut basin = vec![];
        let width = heightmap[0].len();
        let height = heightmap.len();

        if heightmap[y][x] == 9 {
            return basin;
        }
        if marked.contains_key(&(x, y)) {
            return basin;
        }

        basin.push((x, y));
        marked.insert((x, y), ());

        if x > 0 {
            basin.extend(&flood_fill(heightmap, marked, x - 1, y));
        }
        if x < width - 1 {
            basin.extend(&flood_fill(heightmap, marked, x + 1, y));
        }
        if y > 0 {
            basin.extend(&flood_fill(heightmap, marked, x, y - 1));
        }
        if y < height - 1 {
            basin.extend(&flood_fill(heightmap, marked, x, y + 1));
        }

        return basin;
    }

    let mut basins = vec![];
    for (x, y) in lowest_points {
        basins.push(flood_fill(&heightmap, &mut marked, x, y));
    }

    basins.sort_by(|x1, x2| x2.len().cmp(&x1.len()));

    for y in 0..height {
        for x in 0..width {
            if basins.iter().any(|basin| basin.iter().find(|(x1, y1)| *x1 == x && *y1 == y).is_some()) {
                print!("{}", heightmap[y][x]);
            }
            else {
                print!(" ");
            }
        }
        println!();
    }

    println!("Result: {:?}", basins.iter().take(3)
        .map(|x| x.len()).fold(1usize, |acc, x| acc * x));

    Ok(())
}
