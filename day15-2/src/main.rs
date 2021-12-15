use std::io::BufRead;

const START: (usize, usize) = (0, 0);

fn main() {
    let stdin = std::io::stdin();

    let mut grid_template = vec![];
    for line in stdin.lock().lines() {
        grid_template.push(line.unwrap().chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>());
    }

    let template_w = grid_template[0].len();
    let template_h = grid_template.len();

    let mut grid = vec![];
    for y in 0..template_h * 5 {
        let template_y = y % template_h;
        let y_incr = (y / template_h) as u32;

        let mut row = vec![];
        for x in 0..template_w * 5 {
            let template_x = x % template_w;
            let mut cell = grid_template[template_y][template_x] + y_incr + (x / template_w) as u32;
            if cell > 9 {
                cell = cell - 9;
            }
            row.push(cell);
        }
        grid.push(row);
    }

    let goal = (grid[0].len() - 1, grid.len() - 1);

    fn heuristic(grid: &Vec<Vec<u32>>, mut x1: usize, mut y1: usize, x2: usize, y2: usize) -> u32 {
        return (x1 as i32 - x2 as i32).abs() as u32 + (y1 as i32 - y2 as i32).abs() as u32;
    }

    let result = pathfinding::prelude::astar(&START, |&(x, y)| {
        let mut successors = vec![];

        if x > 0 {
            successors.push(((x - 1, y), grid[y][x - 1]));
        }
        if x < grid[y].len() - 1 {
            successors.push(((x + 1, y), grid[y][x + 1]));
        }
        if y > 0 {
            successors.push(((x, y - 1), grid[y - 1][x]));
        }
        if y < grid.len() - 1 {
            successors.push(((x, y + 1), grid[y + 1][x]));
        }

        successors
    }, |&(x, y)| heuristic(&grid, x, y, goal.0, goal.1), |p| *p == goal);

    let result = result.unwrap();
    let (points, cost) = result;
    println!("{} {:?}", cost, points);

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if points.contains(&(x, y)) {
                print!("{}", grid[y][x]);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

