use std::fmt::Error;
use std::io::BufRead;

fn main() -> Result<(), Error> {
    let stdin = std::io::stdin();
    let mut prev = u32::MAX;
    let mut occurrences = 0;
    stdin.lock().lines().for_each(|v| {
        if let Ok(x) = v {
            if let Ok(num) = x.parse::<u32>() {
                if num > prev {
                    occurrences += 1;
                }
                prev = num;
            }
        }
    });
    println!("{}", occurrences);
    Ok(())
}
