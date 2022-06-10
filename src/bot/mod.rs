use std::io;
use std::sync::atomic::Ordering;

use atomic_float::AtomicF32;
use indicatif::ProgressBar;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::{
    dice_set::DiceSet,
    game_player::{GamePlayer, GameState, ResetAction, RollAction},
    score::Score,
};

pub struct Robbie;

#[derive(Debug, Clone)]
pub struct SimpleGameState {
    pub cur_score: Score,
    pub pot: Score,
    pub dice_left: usize,
}

impl From<&GameState> for SimpleGameState {
    fn from(gs: &GameState) -> Self {
        SimpleGameState {
            cur_score: gs.scores[gs.current_player],
            pot: gs.current_pot,
            dice_left: gs.dice_left,
        }
    }
}

impl Robbie {
    pub fn choose_action_simple(gs: SimpleGameState) -> RollAction {
        let score = gs.cur_score;
        let value_of_stopping = 10000.0 - gs.cur_score.0 as f32 - gs.pot.0 as f32;
        let value_of_rolling = Self::recursive_eval_continuing(gs, 100, true);

        println!("Value of stopping: {}", 10000.0 - score.0 as f32 - value_of_stopping);
        println!("Value of rolling:  {}", 10000.0 - score.0 as f32 - value_of_rolling);

        if value_of_stopping < value_of_rolling {
            RollAction::Stop
        } else {
            RollAction::Roll
        }
    }

    pub fn choose_should_reset_simple(gs: SimpleGameState) -> ResetAction {
        let score = gs.cur_score;
        let value_of_keep = Self::recursive_eval_continuing(gs, 100, true);
        let value_of_reset = Self::recursive_eval_continuing(
            SimpleGameState {
                cur_score: score,
                pot: Score(0),
                dice_left: 6,
            },
            100,
            true,
        );

        println!("Value of keeping:   {}", 10000.0 - score.0 as f32 - value_of_keep);
        println!("Value of resetting: {}", 10000.0 - score.0 as f32 - value_of_reset);

        if value_of_keep < value_of_reset {
            ResetAction::Keep
        } else {
            ResetAction::Reset
        }
    }

    fn eval_situation(gs: SimpleGameState, iters: u32, log: bool) -> f32 {
        if gs.cur_score + &gs.pot >= Score(10000) {
            return 0.0;
        }

        let value_of_stopping = 10000.0 - gs.cur_score.0 as f32 - gs.pot.0 as f32;
        let value_of_rolling = Self::recursive_eval_continuing(gs, iters, log);

        f32::min(value_of_rolling, value_of_stopping)
    }

    fn recursive_eval_continuing(gs: SimpleGameState, iters: u32, log: bool) -> f32 {
        let total_value = AtomicF32::new(0.0);

        let bar = if log {
            Some(ProgressBar::new(iters as u64))
        } else {
            None
        };
        (0..iters).into_par_iter().for_each(|_| {
            let roll = DiceSet::new_rand(gs.dice_left);
            let (new_dice_left, score) = roll.score();
            if score != Score(0) {
                let new_gs = SimpleGameState {
                    cur_score: gs.cur_score,
                    pot: gs.pot + &score,
                    dice_left: new_dice_left as usize,
                };
                let eval = Self::eval_situation(new_gs, iters / 2, false);
                let _ = total_value
                    .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + eval));
            } else {
                let _ = total_value.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| {
                    Some(x + 10000.0 - gs.cur_score.0 as f32)
                });
            }

            if let Some(ref bar) = bar {
                bar.inc(1)
            }
        });
        if let Some(bar) = bar {
            bar.finish_and_clear();
        }

        total_value.into_inner() / iters as f32
    }
}

impl GamePlayer for Robbie {
    fn choose_action(&self, game_state: &GameState) -> RollAction {
        Self::choose_action_simple(game_state.into())
    }

    fn keep_or_reset(&self, game_state: &GameState) -> ResetAction {
        Self::choose_should_reset_simple(game_state.into())
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
