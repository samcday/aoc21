use std::fmt::Error;
use std::io::BufRead;

fn main() -> Result<(), Error> {
    let values = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.expect("read line").parse::<i32>().expect("parse number"))
        .collect::<Vec<i32>>();

    let mut prev = i32::MAX;
    let mut occurrences = 0;
    for value in values.windows(3) {
        let sum = value.iter().sum();
        if sum > prev {
            occurrences += 1;
        }
        prev = sum;
    }
    println!("{}", occurrences);
    Ok(())
}
