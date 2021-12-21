use std::io::BufRead;
use memoize::memoize;

struct Player {
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
        Player {board_pos: start_pos, score: 0}
    }).collect::<Vec<Player>>();

    assert_eq!(players.len(), 2);

    let player2 = players.pop().unwrap();
    let player1 = players.pop().unwrap();

    // Recurse into INFINITE UNIVERSES. It's too early in the morning for this kind of existential
    // shit, honestly.
    #[memoize]
    fn quantum_shenanigans(p1_pos: u32, p1_score: u32, p2_pos: u32, p2_score: u32) -> (usize, usize) {
        // Recursive terminal cases / quantum collapse.
        if p1_score >= 21 {
            return (1, 0);
        } else if p2_score >= 21 {
            return (0, 1);
        }

        let (mut p1_wins, mut p2_wins) = (0, 0);

        for (die_1, die_2, die_3) in itertools::iproduct!([1, 2, 3], [1, 2, 3], [1, 2, 3]) {
            let dice_value = die_1 + die_2 + die_3;

            let mut new_pos = p1_pos + dice_value;
            while new_pos > 10 {
                new_pos -= 10;
            }

            let (moar_p2_wins, moar_p1_wins) = quantum_shenanigans(p2_pos, p2_score, new_pos, p1_score + new_pos);
            p1_wins += moar_p1_wins;
            p2_wins += moar_p2_wins;
        }

        (p1_wins, p2_wins)
    }

    let (player1_wins, player2_wins) = quantum_shenanigans(player1.board_pos, player1.score, player2.board_pos, player2.score);
    println!("Player 1 wins in {} universes. Player 2 wins in {} universes.", player1_wins, player2_wins);
}
