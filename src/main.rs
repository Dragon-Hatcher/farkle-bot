use std::io::{self, Write};

use bot::Robbie;
use game_loop::run_game;
use human_player::HumanPlayer;

use crate::{game_player::GamePlayer, terminal::clear_terminal};

mod bot;
mod dice_set;
mod game_loop;
mod game_player;
mod human_player;
mod score;
mod terminal;

fn main() {
    clear_terminal();

    let mut names = String::new();
    print!("Who's playing? ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut names)
        .expect("Non UTF8? Really?");
    let names = names.split(',').map(|s| s.trim().into());
    let mut players: Vec<Box<dyn GamePlayer>> = names
        .map(|n| Box::new(HumanPlayer::new(n)) as Box<dyn GamePlayer>)
        .collect();
    players.push(Box::new(Robbie));

    run_game(true, players);
}
