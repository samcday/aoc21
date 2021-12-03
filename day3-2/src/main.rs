use std::fmt::Error;
use std::io::BufRead;

fn main() -> Result<(), Error> {
    let values = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.expect("read line"))
        .collect::<Vec<String>>();

    let number_len = values[0].len();

    let mut oxygen_candidates = values.clone();
    let mut oxygen = 0;
    let mut oxygen_bits = vec!();
    for i in 0..number_len {
        let mut bit_count = 0;
        let quorum = (oxygen_candidates.len() as f32) / 2.0;
        for x in oxygen_candidates.clone() {
            if x.chars().nth(i).unwrap() == '1' {
                bit_count += 1;
            }
        }

        println!("oxygen bit {}, candidates: {:?}", i, oxygen_candidates);
        let most_common = if (bit_count as f32) >= quorum { '1' } else { '0' };
        oxygen_bits.push(most_common);
        println!("oxygen {} bit_count {} number len {} quorum:{} most common {}", i, bit_count, number_len, quorum, most_common);

        oxygen_candidates.retain(|x| x.chars().nth(i).unwrap() == most_common);
        println!("remaining: {:?}", oxygen_candidates);

        if oxygen_candidates.len() == 1 {
            // oxygen = i32::from_str_radix(&oxygen_bits.iter().collect::<String>(), 2).unwrap();
            oxygen = i32::from_str_radix(oxygen_candidates.first().unwrap(), 2).unwrap();
            break;
        }
        println!();
    }

    println!();
    let mut co2_candidates = values.clone();
    let mut co2 = 0;
    let mut co2_bits = vec!();
    for i in 0..number_len {
        let mut bit_count = 0;
        let quorum = (co2_candidates.len() as f32) / 2.0;
        for x in co2_candidates.clone() {
            if x.chars().nth(i).unwrap() == '1' {
                bit_count += 1;
            }
        }

        println!("co2 bit {}, candidates: {:?}", i, co2_candidates);
        let most_common = if (bit_count as f32) < quorum { '1' } else { '0' };
        co2_bits.push(most_common);
        println!("co2 {} bit_count {} number len {} quorum:{} most common {}", i, bit_count, number_len, quorum, most_common);

        co2_candidates.retain(|x| x.chars().nth(i).unwrap() == most_common);
        println!("remaining: {:?}", co2_candidates);

        if co2_candidates.len() == 1 {
            // co2 = i32::from_str_radix(&co2_bits.iter().collect::<String>(), 2).unwrap();
            co2 = i32::from_str_radix(co2_candidates.first().unwrap(), 2).unwrap();
            break;
        }
        println!();
    }

    println!("oxygen: {}, co2: {}", oxygen, co2);
    println!("{}", oxygen * co2);
    Ok(())
}
