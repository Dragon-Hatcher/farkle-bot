use std::io::{self, Write};

use bot::Robbie;
use game_loop::run_game;
use human_player::HumanPlayer;

use crate::terminal::clear_terminal;

mod bot;
mod dice_set;
mod game_loop;
mod game_player;
mod human_player;
mod score;
mod terminal;

fn main() {
    clear_terminal();

    let mut name = String::new();
    print!("Who's playing? ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut name).expect("Non UTF8? Really?");
    name = name.trim().into();


    run_game(
        true,
        vec![Box::new(HumanPlayer::new(name)), Box::new(Robbie)],
    )
}
