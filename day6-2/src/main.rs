use std::io::BufRead;

const DAYS: usize = 256;

fn main() {
    let stdin = std::io::stdin();

    // When the lanternfishies outnumber the atoms in the galaxy, we must count them as cohorts.
    let mut lanternfishy_cohorts = [0u64; 9];

    let mut lanternfishies = stdin.lock().lines().next().expect("read line").unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().expect("lanternfish age"))
        .for_each(|x| {
            lanternfishy_cohorts[x] += 1;
        });

    for day in 1..=DAYS {
        let day_0_fishies = lanternfishy_cohorts[0];
        for day in 1..=8 {
            lanternfishy_cohorts[day - 1] = lanternfishy_cohorts[day];
        }
        lanternfishy_cohorts[8] = day_0_fishies;
        lanternfishy_cohorts[6] += day_0_fishies;

        println!("After {:02} days: {} ({} total)",
            day,
             (0..8).map(|age| format!("{}: {}", age, lanternfishy_cohorts[age])).collect::<Vec<String>>().join(" "),
            lanternfishy_cohorts.iter().sum::<u64>());
    }
}
