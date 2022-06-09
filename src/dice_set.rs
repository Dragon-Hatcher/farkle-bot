use std::{fmt::Display, str::FromStr};

use rand::prelude::*;

use crate::score::{score_dice_set, Score};

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
    pub fn new_rand() -> DiceSet {
        let mut rng = rand::thread_rng();
        let mut new_dice = [0; DICE_IN_DICE_SET];
        for x in &mut new_dice {
            *x = rng.gen_range(1..=6);
        }

        DiceSet {
            dice: new_dice,
            active_dice: 6,
        }
    }

    pub fn dice(&self) -> &[u8] {
        &self.dice[..self.active_dice]
    }

    pub fn score(&mut self) -> (u8, Score) {
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

        for die in self.dice() {
            f.write_fmt(format_args!("{} ", DIE_CHARS[*die as usize - 1]))?;
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
        //     .flatten()
        //     .collect::<Vec<u8>>();
        // if nums.len() > 6 {
        //     return Err(());
        // }
        // all_nums[..].clone_from_slice(nums.as_ref());
        Ok(DiceSet {
            dice: all_nums,
            active_dice: total,
        })
    }
}
