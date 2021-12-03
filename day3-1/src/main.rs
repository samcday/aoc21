use std::fmt::Error;
use std::io::BufRead;

fn main() -> Result<(), Error> {
    let values = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.expect("read line"))
        .collect::<Vec<String>>();

    let number_len = values[0].len();
    let mut bit_counts = (0..number_len).map(|x| 0).collect::<Vec<u32>>();
    for value in values.iter().cloned() {
        if value.len() != number_len {
            panic!("bad value {} not same length as others", value);
        }

        for (i, bit) in value.chars().enumerate() {
            bit_counts[i] += bit.to_digit(10).expect("parse digit");
        }
    }

    let mut gamma = vec!();
    let mut epsilon = vec!();
    let quorum = (&values.len() / 2) as u32;

    for bit in bit_counts.iter().enumerate() {
        gamma.push(if *bit > quorum { '1' } else { '0' });
        epsilon.push(if *bit > quorum { '0' } else { '1' });
    }

    let gamma_str = gamma.iter().collect::<String>();
    let epsilon_str = epsilon.iter().collect::<String>();

    println!("gamma: {}", gamma_str);
    println!("epsilon: {}", epsilon_str);

    let power =
        u32::from_str_radix(gamma_str.as_str(), 2).unwrap() *
            u32::from_str_radix(epsilon_str.as_str(), 2).unwrap();

    println!("{}", power);
    Ok(())
}
