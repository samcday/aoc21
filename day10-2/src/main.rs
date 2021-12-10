use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();

    let mut autocomplete_scores = vec![];

    'lines: for line in stdin.lock().lines() {
        let line = line.expect("read line");

        let mut stack = vec![];

        for char in line.chars() {
            match char {
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '(' => stack.push(')'),
                '<' => stack.push('>'),
                _ => {
                    let expected = stack.pop().unwrap();
                    if char != expected {
                        println!("{} - Expected {}, but found {} instead.", line, expected, char);
                        continue 'lines;
                    }
                }
            }
        }

        let mut score: i64 = 0;
        for item in stack.iter().rev() {
            score = score * 5 + match item {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            };
        }
        autocomplete_scores.push((stack.iter().rev().collect::<String>(), score));
    }

    autocomplete_scores.sort_by(|(_, score1), (_, score2)| score1.cmp(score2));

    println!("{:?}", autocomplete_scores);
    println!("middle score: {}", (autocomplete_scores[autocomplete_scores.len() / 2 as usize].1));
}
