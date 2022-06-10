use std::io;

use crate::game_player::{GamePlayer, GameState, ResetAction, RollAction};

pub struct Robbie;

impl GamePlayer for Robbie {
    fn choose_action(&self, _game_state: &GameState) -> RollAction {
        if rand::random() {
            RollAction::Roll
        } else {
            RollAction::Stop
        }
    }

    fn keep_or_reset(&self, _game_state: &GameState) -> ResetAction {
        if rand::random() {
            ResetAction::Keep
        } else {
            ResetAction::Reset
        }
    }

    fn confirm(&self) {
        let mut buff = String::new();
        let _ = io::stdin().read_line(&mut buff);
    }

    fn name(&self) -> &str {
        "Robbie"
    }

    fn is_human(&self) -> bool {
        false
    }
}
