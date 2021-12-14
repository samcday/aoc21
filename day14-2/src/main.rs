use std::collections::HashMap;
use std::io::BufRead;

const STEPS: usize = 40;

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let template = lines.next().expect("read template").unwrap();
    assert_eq!(0, lines.next().unwrap().unwrap().len());

    let mut insertion_rules = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        let mut split = line.split(" -> ");
        let pair = split.next().unwrap().to_string();
        let insertion = split.next().unwrap().to_string().chars().nth(0).unwrap();
        let new_pairs = [
            format!("{}{}", pair.chars().nth(0).unwrap(), insertion),
            format!("{}{}", insertion, pair.chars().nth(1).unwrap()),
        ];
        insertion_rules.insert(pair, new_pairs);
    }

    let mut polymer_chain = HashMap::new();
    for idx in 1..template.len() {
        *polymer_chain.entry(&template[idx-1..idx+1]).or_insert(0u64) += 1;
    }

    for step in 1..=STEPS {
        let old_chain = polymer_chain.clone();
        polymer_chain.clear();
        for (pair, count) in old_chain {
            for new_pair in &insertion_rules[pair] {
                *polymer_chain.entry(&new_pair).or_insert(0) += count;
            }
        }
    }

    let mut elements = HashMap::new();
    for (pair, count) in polymer_chain.iter() {
        for c in pair.chars() {
            *elements.entry(c).or_insert(0) += count;
        }
    }
    for (k, v) in elements.iter_mut() {
        *v = ((*v as f64) / 2.0).round() as u64;
    }


    let mut elements_sorted = elements.iter().map(|x| (*x.0, *x.1)).collect::<Vec<(char, u64)>>();

    elements_sorted.sort_by(|x, y| x.1.cmp(&y.1));
    println!("Result: {}", elements_sorted.iter().last().unwrap().1 - elements_sorted.iter().next().unwrap().1);
}
