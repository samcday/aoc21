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
            println!("processing {}", candidate);
            let cost: i32 = positions.iter()
                .map(|x| {
                    let mut cost = 0;
                    for n in (0..=((x - candidate).abs())) {
                        cost += n;
                    }
                    cost
                }).sum();
            (candidate, cost)
        }).collect::<Vec<(i32, i32)>>();

    println!("{:?}", moves);

    let cheapest = moves.iter().min_by(|x, y| x.1.cmp(&y.1)).unwrap();
    println!("cheapest move is to {}, costing {}", cheapest.0, cheapest.1);
}
