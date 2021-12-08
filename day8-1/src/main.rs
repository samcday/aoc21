use std::collections::HashMap;
use std::io::BufRead;
use std::iter::Map;

fn main() {
    let stdin = std::io::stdin();

    let input: Vec<(Vec<String>, Vec<String>)> = stdin.lock().lines()
        .map(|line| {
            let line = line.unwrap();
            let mut it = line.split(" | ");
            (
                it.next().unwrap().split(" ").map(|x| x.to_string()).collect(),
                it.next().unwrap().split(" ").map(|x| x.to_string()).collect()
            )
        })
        .collect();

    let mut uniq_digit_count = 0;

    for (_, output) in input {
        for str in &output {
            match str.len() {
                2|3|4|7 => {
                    println!("uniq: {}", str);
                    uniq_digit_count += 1;
                },
                _ => {}
            }
        }
    }

    println!("Uniq digit occurrences: {}", uniq_digit_count);
}
