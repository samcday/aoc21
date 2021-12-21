use std::io::BufRead;

enum Dice {
    DETERMINISTIC(u32),
}

impl Dice {
    fn roll(self) -> (Dice, u32) {
        match self {
            Dice::DETERMINISTIC(v) => {
                (Dice::DETERMINISTIC(if v + 1 <= 100 { v + 1 } else { 1 }), v)
            }
        }
    }
}

struct Player {
    num: u32,
    board_pos: u32,
    score: u32,
}

fn main() {
    let stdin = std::io::stdin();

    let mut players = stdin.lock().lines().map(|line| {
        let line = line.unwrap();
        let line = line.strip_prefix("Player ").unwrap();

        let mut split = line.splitn(2," ");
        let player_num = split.next().unwrap().parse::<u32>().unwrap();
        let line = split.next().unwrap();
        let line = line.strip_prefix("starting position: ").unwrap();
        let start_pos = line.parse::<u32>().unwrap();
        println!("player: {} start: {}", player_num, start_pos);
        Player {num: player_num, board_pos: start_pos, score: 0}
    }).collect::<Vec<Player>>();

    let mut dice = Dice::DETERMINISTIC(1);

    let mut total_dice_rolls = 0;
    let mut dice_rolls = vec![];

    'game: loop {
        for player in &mut players {
            dice_rolls.clear();
            for i in 0..3 {
                total_dice_rolls += 1;
                let (new_dice, num) = dice.roll();
                dice = new_dice;
                dice_rolls.push(num);
            }

            let sum: u32 = dice_rolls.iter().sum();
            player.board_pos = player.board_pos + sum;
            while player.board_pos > 10 {
                player.board_pos -= 10;
            }

            player.score += player.board_pos as u32;

            let won = player.score >= 1000;

            println!("Player {} rolls {} and moves to space {} for a {} score of {}",
                     player.num,
                     dice_rolls.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("+"),
                     player.board_pos,
                     if won { "final" } else { "total " },
                     player.score);

            if won {
                break 'game;
            }
        }
    }

    let loser = players.iter().find(|x| x.score < 1000).unwrap();
    println!("{} total dice rolls, loser score: {}, part 1 solution: {}",
        total_dice_rolls, loser.score, total_dice_rolls * loser.score);
}
