use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use std::io::{BufRead, Read};
use std::str::FromStr;

#[derive(Debug)]
struct EnhancementAlgorithm([bool; 512]);

impl FromStr for EnhancementAlgorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(s.len(), 512);

        let mut alg = [false; 512];
        for (idx, char) in s.chars().enumerate() {
            if char == '#' {
                alg[idx] = true;
            } else if char != '.' {
                return Err(format!("unexpected alg char at index {}: {}", char, idx));
            }
        }
        Ok(EnhancementAlgorithm(alg))
    }
}

impl Display for EnhancementAlgorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for v in self.0.iter() {
            f.write_char(if *v { '#' } else {'.'})?
        }
        Ok(())
    }
}

impl EnhancementAlgorithm {
    fn enhance(&self, image: Image, round: usize) -> Image {
        let mut pixels = HashSet::new();

        let mut x1 = image.x1 - 1;
        let mut x2 = image.x2 + 1;
        let mut y1 = image.y1 - 1;
        let mut y2 = image.y2 + 1;

        let mut infinite_grid_lit = false;
        if self.0[0] {
            infinite_grid_lit = round % 2 == 1;
        }

        for y in y1..=y2 {
            for x in x1..=x2 {
                let window = image.pixel_window(x, y, infinite_grid_lit);
                let lit = self.0[window as usize];
                if lit {
                    pixels.insert((x, y));
                }
            }
        }

        Image{ pixels, x1, x2, y1, y2 }
    }
}

struct Image {
    pixels: HashSet<(i32, i32)>,
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

impl FromStr for Image {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pixels = HashSet::new();
        let mut x1 = i32::MAX;
        let mut x2 = 0;
        let mut y1 = i32::MAX;
        let mut y2 = 0;

        for (y, line) in s.lines().filter(|x| !x.is_empty()).enumerate() {
            for (x, pix) in line.chars().enumerate() {
                let (x, y) = (x as i32, y as i32);

                if pix == '#' {
                    pixels.insert((x, y));
                    x1 = std::cmp::min(x1, x);
                    x2 = std::cmp::max(x2, x);
                    y1 = std::cmp::min(y1, y);
                    y2 = std::cmp::max(y2, y);
                } else if pix != '.' {
                    return Err("Malformed input".to_string());
                }
            }
        }

        Ok(Image{pixels, x1, x2, y1, y2})
    }
}

impl Image {
    fn pixel_window(&self, x: i32, y: i32, default: bool) -> u16 {
        let mut result = 0;

        let mut idx = 0;
        for y_pos in y-1..=y+1 {
            for x_pos in x-1..=x+1 {
                let lit = if y_pos < self.y1 || x_pos < self.x1 || x_pos > self.x2 || y_pos > self.y2 {
                    default
                } else {
                    self.pixels.contains(&(x_pos, y_pos))
                };

                if lit {
                    result |= 1 << 8 - idx;
                }
                idx += 1;
            }
        }
        result
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in self.y1..=self.y2 {
            for x in self.x1..=self.x2 {
                f.write_char(if self.pixels.contains(&(x, y)) { '#' } else { '.' })?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

fn main() {
    let stdin = std::io::stdin();

    // Parse enhancement algorithm.
    let mut buf = String::new();
    stdin.lock().read_line(&mut buf).unwrap();
    let enhancement_algorithm = buf.trim().parse::<EnhancementAlgorithm>().unwrap();

    buf.clear();
    stdin.lock().read_to_string(&mut buf).unwrap();
    let mut image = buf.parse::<Image>().unwrap();

    println!("Original image:\n{}", image);

    for round in 0..50 {
        image = enhancement_algorithm.enhance(image, round);
        if round == 2 {
            println!("Part 1: {}", image.pixels.len());
        }
        // println!("Zoom! Enhance!\n{}", image);
    }

    println!("Final: {}", image);
    println!("Lit pixels: {}", image.pixels.len());
}
