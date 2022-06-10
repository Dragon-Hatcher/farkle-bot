use std::io::{self, stdout, Write};

use crate::game_player::{GamePlayer, ResetAction, RollAction};

pub struct HumanPlayer {
    name: String,
}

impl HumanPlayer {
    pub fn new(name: String) -> HumanPlayer {
        HumanPlayer { name }
    }
}

impl GamePlayer for HumanPlayer {
    fn choose_action(&self, _game_state: &crate::game_player::GameState) -> RollAction {
        let mut input = String::new();
        loop {
            print!("Roll or stop? ");
            let _ = stdout().flush();
            let _ = io::stdin().read_line(&mut input);
            match input.trim().to_lowercase().as_str() {
                "roll" => return RollAction::Roll,
                "stop" => return RollAction::Stop,
                _ => {}
            }
        }
    }

    fn keep_or_reset(&self, _game_state: &crate::game_player::GameState) -> ResetAction {
        let mut input = String::new();
        loop {
            print!("Keep the dice or reset? ");
            let _ = stdout().flush();
            let _ = io::stdin().read_line(&mut input);
            match input.trim().to_lowercase().as_str() {
                "keep" => return ResetAction::Keep,
                "reset" => return ResetAction::Reset,
                _ => {}
            }
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_human(&self) -> bool {
        true
    }

    fn confirm(&self) {
        let mut buff = String::new();
        let _ = io::stdin().read_line(&mut buff);
    }
}
