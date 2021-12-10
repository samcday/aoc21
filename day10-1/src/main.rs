use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();

    let mut illegal = vec![];

    for line in stdin.lock().lines() {
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
                        illegal.push(char);
                    }
                }
            }
        }
    }

    let score: u32 = illegal.iter().map(|x| match x {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }).sum();
    println!("{:?} score={}", illegal, score);
}
