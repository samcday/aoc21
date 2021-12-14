use std::collections::HashMap;
use std::io::BufRead;

const STEPS: usize = 10;

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let mut polymer = lines.next().expect("read template").unwrap();
    assert_eq!(0, lines.next().unwrap().unwrap().len());

    let mut insertion_rules = vec![];
    for line in lines {
        let line = line.unwrap();
        let mut split = line.split(" -> ");
        let pair = split.next().unwrap().to_string();
        let insertion = split.next().unwrap().to_string().chars().nth(0).unwrap();

        insertion_rules.push((pair, insertion));
    }

    println!("Template: {}", polymer);
    for step in 1..=STEPS {
        let mut new_polymer = String::new();
        for idx in 1..polymer.len() {
            new_polymer.push(polymer.chars().nth(idx - 1).unwrap());
            for (pair, insert) in insertion_rules.iter() {
                if polymer[(idx - 1..idx + 1)] == *pair {
                    new_polymer.push(*insert);
                }
            }
        }
        new_polymer.push(polymer.chars().last().unwrap());

        polymer = new_polymer;
        println!("After step {}: {}", step, polymer);
    }

    let mut elements = HashMap::new();
    for element in polymer.chars() {
        *elements.entry(element).or_insert(0) += 1;
    }

    let mut elements_sorted = elements.iter().map(|x| (*x.0, *x.1)).collect::<Vec<(char, i32)>>();

    elements_sorted.sort_by(|x, y| x.1.cmp(&y.1));
    println!("Result: {}", elements_sorted.iter().last().unwrap().1 - elements_sorted.iter().next().unwrap().1);
}
