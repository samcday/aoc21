use std::io::BufRead;

#[derive(Debug)]
struct BingoBoard([Option<u8>; 25]);

impl BingoBoard {
    fn row(&self, y: usize) -> &[Option<u8>] {
        &self.0[y * 5..y*5+5]
    }

    fn col(&self, x: usize) -> [Option<u8>; 5] {
        let mut col = [None; 5];
        let mut pos = x;
        for i in 0..5 {
            col[i] = self.0[pos];
            pos += 5;
        }
        col
    }

    fn bingo(&self) -> bool {
        for y in 0..5 {
            if self.row(y).iter().all(|x| x.is_none()) {
                return true;
            }
        }

        for x in 0..5 {
            if self.col(x).iter().all(|x| x.is_none()) {
                return true;
            }
        }

        return false;
    }

    fn remove(&mut self, num: u8) {
        let mut remove_idx = vec![];
        for (idx, board_num) in self.0.iter().enumerate() {
            if Some(num) == *board_num {
                remove_idx.push(idx);
            }
        }
        for idx in remove_idx {
            self.0[idx] = None;
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock().lines();

    let bingo_numbers = stdin.next()
        .expect("bingo numbers").expect("bingo numbers")
        .split(",")
        .map(|x| x.parse::<u8>().expect("parse bingo number"))
        .collect::<Vec<u8>>();

    println!("bingo_numbers {:?}", bingo_numbers);

    let mut boards = Vec::new();
    while let Some(empty) = stdin.next() {
        assert_eq!(empty.unwrap().len(), 0);
        let board = BingoBoard((0..5).flat_map(|y| {
            stdin.next().expect("bingo board row").unwrap()
                .split_whitespace()
                .map(|x| Some(x.parse::<u8>().expect("bingo board number")))
                .collect::<Vec<Option<u8>>>()
        }).collect::<Vec<Option<u8>>>().try_into().unwrap());
        boards.push(board);
    }

    let mut last_win = None;

    for num in bingo_numbers {
        for mut board in boards.iter_mut() {
            board.remove(num);
            if board.bingo() {
                let sum = board.0.iter()
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap() as u32)
                    .sum::<u32>();
                last_win = Some(sum * (num as u32));
            }
        }
        boards.retain(|x| !x.bingo());
    }

    println!("Last bingo: {}", last_win.unwrap());
}
