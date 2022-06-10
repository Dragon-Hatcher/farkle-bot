use bot::Robbie;
use game_loop::run_game;
use human_player::HumanPlayer;

mod bot;
mod dice_set;
mod game_loop;
mod game_player;
mod human_player;
mod score;
mod terminal;

fn main() {
    run_game(
        true,
        vec![Box::new(HumanPlayer::new("Alice".into())), Box::new(Robbie)],
    )
}
