use crate::score::Score;

pub enum RollAction {
    Roll,
    Stop,
}

pub enum ResetAction {
    Keep,
    Reset,
}

pub struct GameState {
    pub scores: Vec<Score>,
    pub current_pot: Score,
    pub current_player: usize,
    pub dice_left: usize,
}
pub trait GamePlayer {
    fn choose_action(&self, game_state: &GameState) -> RollAction;

    fn keep_or_reset(&self, game_state: &GameState) -> ResetAction;

    fn confirm(&self);

    fn name(&self) -> &str;

    fn is_human(&self) -> bool;
}
