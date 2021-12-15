use std::io::BufRead;

const START: (usize, usize) = (0, 0);

// #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
// struct Pos(usize, usize);

fn main() {
    let stdin = std::io::stdin();

    let mut grid = vec![];
    for line in stdin.lock().lines() {
        grid.push(line.unwrap().chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>());
    }

    let goal = (grid[0].len() - 1, grid.len() - 1);

    fn heuristic(grid: &Vec<Vec<u32>>, mut x1: usize, mut y1: usize, x2: usize, y2: usize) -> u32 {
        return (x1 as i32 - x2 as i32).abs() as u32 + (y1 as i32 - y2 as i32).abs() as u32;
        //
        // let mut cost = 0;
        //
        // loop {
        //     if x1 < x2 {
        //         x1 += 1;
        //         cost += grid[y1][x1];
        //     }
        //     if y1 < y2 {
        //         y1 += 1;
        //         cost += grid[y1][x1];
        //     }
        //     if x1 == x2 && y1 == y2 {
        //         return cost;
        //     }
        // }
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


// #[derive(Eq, PartialEq, Debug)]
// struct Node {
//     pos: (usize, usize),
//     score: u32,
// }
//
// impl PartialOrd<Self> for Node {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.score.cmp(&other.score).reverse())
//     }
// }
//
// impl Ord for Node {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.score.cmp(&other.score).reverse()
//     }
// }
// fn fucked() {
//     let stdin = std::io::stdin();
//
//     let mut grid = vec![];
//
//     for line in stdin.lock().lines() {
//         grid.push(line.unwrap().chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>());
//     }
//
//     let goal = (grid[0].len() - 1, grid.len() - 1);
//
//     fn cost_heuristic(grid: &Vec<Vec<u32>>, mut from: (usize, usize), to: (usize, usize)) -> u32 {
//         let mut cost = 0;
//
//         let (mut x1, mut y1) = from;
//         let (mut x2, mut y2) = to;
//         loop {
//             if x1 < x2 {
//                 x1 += 1;
//             }
//             if y1 < y2 {
//                 y1 += 1;
//             }
//             cost += grid[y1][x1];
//             if x1 == x2 && y1 == y2 {
//                 return cost;
//             }
//         }
//     }
//
//     // A*
//     let mut open_set = BinaryHeap::new();
//
//     open_set.push(Node{pos: START, score: cost_heuristic(&grid, START, goal)});
//
//     let mut came_from = HashMap::new();
//
//     let mut g_score: HashMap<(usize, usize), u32> = HashMap::new();
//     g_score.insert(START,  0);
//
//     while !open_set.is_empty() {
//         let current = open_set.pop().unwrap();
//
//         println!("curr: {:?}", &current);
//
//         if current.pos == goal {
//             let mut pos_nav = Some(current.pos);
//             let mut final_path = vec![];
//             while let Some(pos) = pos_nav {
//                 final_path.push((pos, grid[pos.1][pos.0]));
//                 pos_nav = came_from.get(&pos).cloned();
//             }
//             final_path.reverse();
//             println!("YAY! {:?} {:?}", current, final_path);
//             println!("\n\n\n{:?}", g_score);
//             return;
//         }
//
//         let (x, y) = current.pos;
//
//         let neighbors = [
//             if x > 0 { Some((x - 1, y)) } else { None },
//             if x < grid[y].len() - 1 { Some((x + 1, y)) } else { None },
//             if y > 0 { Some((x, y - 1)) } else { None },
//             if y < grid.len() - 1 { Some((x, y + 1)) } else { None },
//         ];
//
//         for neighbor in neighbors {
//             if neighbor.is_none() {
//                 continue;
//             }
//             let neighbor = neighbor.unwrap();
//             let (neighbor_x, neighbor_y) = neighbor;
//             let tentative_score = current.score + grid[neighbor_y][neighbor_x];
//
//             if tentative_score < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
//                 came_from.insert(neighbor, current.pos);
//             }
//             g_score.insert(neighbor, tentative_score);
//             open_set.push(Node{pos: neighbor, score: current.score + cost_heuristic(&grid, neighbor, goal)})
//         }
//     }
// }
