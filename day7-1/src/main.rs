use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();

    let positions = stdin.lock().lines()
        .next().expect("read line").unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().expect("parse num"))
        .collect::<Vec<i32>>();

    let moves = (0..=*positions.iter().max().unwrap())
        .map(|candidate| {
            let cost: i32 = positions.iter()
                .map(|x| (x - candidate).abs()).sum();
            (candidate, cost)
        }).collect::<Vec<(i32, i32)>>();

    let cheapest = moves.iter().min_by(|x, y| x.1.cmp(&y.1)).unwrap();
    println!("cheapest move is to {}, costing {}", cheapest.0, cheapest.1);
}
