use std::{fmt::Display, str::FromStr};

use colored::Colorize;
use rand::prelude::*;

use crate::score::{score_dice_set, Score, should_color};

// use rand::prelude::*;

// impl DiceValue {
//     fn new_rand() -> DiceValue {
//         DiceValue(rand::thread_rng().gen_range(1..=6))
//     }
// }

const DICE_IN_DICE_SET: usize = 6;

#[derive(Debug, Clone)]
pub struct DiceSet {
    dice: [u8; DICE_IN_DICE_SET],
    active_dice: usize,
}

impl DiceSet {
    pub fn new_rand(active_dice: usize) -> DiceSet {
        assert!(active_dice <= 6, "Too many dice!");

        let mut rng = rand::thread_rng();
        let mut new_dice = [0; DICE_IN_DICE_SET];
        for x in &mut new_dice {
            *x = rng.gen_range(1..=6);
        }

        DiceSet {
            dice: new_dice,
            active_dice,
        }
    }

    pub fn dice(&self) -> &[u8] {
        &self.dice[..self.active_dice]
    }

    pub fn score(&self) -> (u8, Score) {
        score_dice_set(self)
    }
}

impl From<[u8; 6]> for DiceSet {
    fn from(dice: [u8; 6]) -> Self {
        DiceSet {
            dice,
            active_dice: 6,
        }
    }
}

impl Display for DiceSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // const DIE_CHARS: [char; 6] = ['⚀', '⚁', '⚂', '⚃', '⚄', '⚅'];
        const DIE_CHARS: [char; 6] = ['1', '2', '3', '4', '5', '6'];
        let should_color = should_color(self);

        for die in self.dice() {
            let die_char = DIE_CHARS[*die as usize - 1];
            if should_color[*die as usize - 1] {
                f.write_fmt(format_args!("{} ", die_char.to_string().green()))?;
            } else {
                f.write_fmt(format_args!("{} ", die_char))?;
            }
        }

        Ok(())
    }
}

impl FromStr for DiceSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut all_nums = [0; 6];
        let nums = s.split_whitespace().map(str::parse::<u8>);
        let mut total = 0;

        for (i, num) in nums.enumerate() {
            if total > 5 {
                return Err(anyhow::anyhow!("Too many numbers."));
            }

            all_nums[i] = num?;
            total += 1;
        }

        Ok(DiceSet {
            dice: all_nums,
            active_dice: total,
        })
    }
}
