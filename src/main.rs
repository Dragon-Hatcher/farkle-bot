use std::io::{stdin, stdout, Write};

use bot::Robbie;
use game_loop::run_game;
use human_player::HumanPlayer;

mod dice_set;
mod game_loop;
mod game_player;
mod human_player;
mod score;
mod terminal;
mod bot;

fn main() {
    run_game(
        true,
        vec![
            Box::new(HumanPlayer::new("Alice".into())),
            Box::new(Robbie),
        ],
    )
    // loop {
    //     let mut i: DiceSet = read_dice_set("Enter a dice set: ");
    //     let (dice_left, score) = i.score();
    //     println!("+{} \t| {} dice left", score, dice_left);
    // }
}

// fn read_dice_set(message: &str) -> DiceSet {
//     print!("{}", message);
//     let _ = stdout().flush();
//     let mut out = String::new();
//     loop {
//         stdin().read_line(&mut out).expect("Non UTF-8? Really?");
//         let resp = out.parse::<DiceSet>();
//         match resp {
//             Ok(ds) => return ds,
//             Err(_) => {
//                 print!("{}", message);
//                 let _ = stdout().flush();
//             }
//         }
//     }
// }
