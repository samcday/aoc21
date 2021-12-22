use std::collections::HashSet;
use std::io::BufRead;

/*
Clearly, I can't track the individual lit status of each cube. I have to work in ranges.
The tricky part is "deforming" these cuboid ranges.

i.e if I have a cuboid range of (0, 4) (0, 4) (0, 4) that has been lit, if I then turn off the
range (1,2) (1,2) (1,2), I would end up with 6 new lit ranges that are all adjacent to the newly
unlit area.

In other words, this is yet another exercise that isn't really testing coding abilities, but memory
of highschool math concepts. Sigh.
 */

fn main() {
    let stdin = std::io::stdin();

    let mut lit_ranges: Vec<(i64, i64, i64, i64, i64, i64)> = vec![];

    'line: for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut split = line.split(" ");
        let on = split.next().unwrap() == "on";
        let coords = split.next().unwrap().split(",");

        let mut parsed_ranges = ((0, 0), (0, 0), (0, 0));

        let (
            mut new_lower_x, mut new_upper_x,
            mut new_lower_y, mut new_upper_y,
            mut new_lower_z, mut new_upper_z
        ) = (0, 0, 0, 0, 0, 0);

        for coord in coords {
            let mut split = coord.split("=");
            let axis = split.next().unwrap();
            assert_eq!(axis.len(), 1);
            let axis = axis.chars().next().unwrap();

            let mut range = split.next().unwrap().split("..");
            let mut lower = range.next().unwrap().parse::<i64>().unwrap();
            let mut upper = range.next().unwrap().parse::<i64>().unwrap();

            match axis {
                'x' => { new_lower_x = lower; new_upper_x = upper },
                'y' => { new_lower_y = lower; new_upper_y = upper },
                'z' => { new_lower_z = lower; new_upper_z = upper },
                _ => unreachable!(),
            }
        }

        lit_ranges = lit_ranges.iter()
            .flat_map(|&range| {
                let (lit_lower_x, lit_upper_x, lit_lower_y, lit_upper_y, lit_lower_z, lit_upper_z) = range;

                // Skip if this new range does not intersect this existing range at all.
                if new_upper_x < lit_lower_x ||
                    new_lower_x > lit_upper_x ||
                    new_upper_y < lit_lower_y ||
                    new_lower_y > lit_upper_y ||
                    new_upper_z < lit_lower_z ||
                    new_lower_z > lit_upper_z {
                    return vec![range];
                }

                let mut new_ranges = vec![];

                let max_lower_x = std::cmp::max(lit_lower_x, new_lower_x);
                let min_upper_x = std::cmp::min(lit_upper_x, new_upper_x);
                let max_lower_y = std::cmp::max(lit_lower_y, new_lower_y);
                let min_upper_y = std::cmp::min(lit_upper_y, new_upper_y);

                if lit_upper_x > new_upper_x {
                    new_ranges.push((new_upper_x + 1, lit_upper_x, lit_lower_y, lit_upper_y, lit_lower_z, lit_upper_z));
                }
                if lit_lower_x < new_lower_x {
                    new_ranges.push((lit_lower_x, new_lower_x - 1, lit_lower_y, lit_upper_y, lit_lower_z, lit_upper_z));
                }
                if lit_upper_y > new_upper_y {
                    new_ranges.push((max_lower_x, min_upper_x, new_upper_y + 1, lit_upper_y, lit_lower_z, lit_upper_z));
                }
                if lit_lower_y < new_lower_y {
                    new_ranges.push((max_lower_x, min_upper_x, lit_lower_y, new_lower_y - 1, lit_lower_z, lit_upper_z))
                }
                if lit_upper_z > new_upper_z {
                    new_ranges.push((max_lower_x, min_upper_x, max_lower_y, min_upper_y, new_upper_z + 1, lit_upper_z));
                }
                if lit_lower_z < new_lower_z {
                    new_ranges.push((max_lower_x, min_upper_x, max_lower_y, min_upper_y, lit_lower_z, new_lower_z - 1));
                }

                new_ranges

            })
            .collect::<Vec<_>>();
        if on {
            lit_ranges.push((new_lower_x, new_upper_x, new_lower_y, new_upper_y, new_lower_z, new_upper_z));
        }
    }

    let mut total_lit: u64 = 0;
    for (lower_x, upper_x, lower_y, upper_y, lower_z, upper_z) in lit_ranges {
        let lit = (upper_x - lower_x + 1) * (upper_y - lower_y + 1) * (upper_z - lower_z + 1);
        total_lit += lit as u64;
    }
    println!("Total lit cubes: {}", total_lit);
}
