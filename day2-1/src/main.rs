use std::fmt::Error;
use std::io::BufRead;

#[derive(PartialEq, Copy, Clone, Debug)]
enum Dir {
    UP,
    DOWN,
    FORWARD,
}

fn main() -> Result<(), Error> {
    let values = std::io::stdin()
        .lock()
        .lines()
        .map(|mut x| {
            let mut split = x.as_mut().expect("read line").split_whitespace();

            let dir = match split.next() {
                Some("up") => Dir::UP,
                Some("down") => Dir::DOWN,
                Some("forward") => Dir::FORWARD,
                _ => panic!("unexpected direction")
            };

            let amount = split.next()
                .expect("number after direction")
                .parse::<i32>()
                .expect("number parse");
            (dir, amount)
        })
        .collect::<Vec<(Dir, i32)>>();

    let mut horiz = 0;
    let mut depth = 0;

    for (dir, amount) in values {
        match dir {
            Dir::UP => depth -= amount,
            Dir::DOWN => depth += amount,
            Dir::FORWARD => horiz += amount,
        }
    }
    println!("{}", depth * horiz);
    Ok(())
}
