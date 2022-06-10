use std::{
    fmt::Display,
    ops::{Add, AddAssign},
};

use crate::dice_set::DiceSet;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Score(pub u16);

impl Add<&Score> for Score {
    type Output = Score;

    fn add(self, rhs: &Score) -> Self::Output {
        Score(self.0 + rhs.0)
    }
}

impl AddAssign<&Score> for Score {
    fn add_assign(&mut self, rhs: &Score) {
        self.0 += rhs.0;
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub fn score_dice_set(ds: &DiceSet) -> (u8, Score) {
    let mut count = [0_u8; 6];

    for die in ds.dice() {
        count[*die as usize - 1] += 1;
    }

    let mut score: u16 = 0;

    if count == [1, 1, 1, 1, 1, 1] {
        // Straight
        return (6, Score(1500));
    }

    if count.contains(&6) {
        // 6 of a kind
        return (6, Score(3000));
    }

    if count.iter().filter(|&x| x == &3).count() == 2 {
        // 2 triplets
        return (6, Score(2500));
    }

    if count.iter().filter(|&x| x == &2).count() == 3 {
        // 3 pairs
        return (6, Score(1500));
    }

    if let Some(x) = count.iter().position(|&x| x == 5) {
        // 5 of a kind
        score += 2000;
        count[x] = 0;
    } else if let Some(x) = count.iter().position(|&x| x == 4) {
        // 4 of a kind
        score += 1000;
        count[x] = 0;

        if count.contains(&2) {
            return (6, Score(1500));
        }
    } else if let Some(x) = count.iter().position(|&x| x == 3) {
        // There can be only one three of a kind becuase we already did 2 triples.
        // 3 of a kind
        score += if x == 0 { 300 } else { (x + 1) as u16 * 100 };
        count[x] = 0;
    }

    score += count[0] as u16 * 100;
    count[0] = 0;
    score += count[4] as u16 * 50;
    count[4] = 0;

    let dice_left = count.iter().sum::<u8>();

    (if dice_left == 0 { 6 } else { dice_left }, Score(score))
}

pub fn should_color(ds: &DiceSet) -> [bool; 6] {
    let mut count = [0_u8; 6];
    let mut color = [true, false, false, false, true, false];

    for die in ds.dice() {
        count[*die as usize - 1] += 1;
    }

    if count == [1, 1, 1, 1, 1, 1] {
        // Straight
        return [true; 6];
    }

    if count.contains(&6) {
        // 6 of a kind
        return [true; 6];
    }

    if count.iter().filter(|&x| x == &3).count() == 2 {
        // 2 triplets
        return [true; 6];
    }

    if count.iter().filter(|&x| x == &2).count() == 3 {
        // 3 pairs
        return [true; 6];
    }

    if let Some(x) = count.iter().position(|&x| x == 5) {
        // 5 of a kind
        color[x] = true;
    } else if let Some(x) = count.iter().position(|&x| x == 4) {
        // 4 of a kind
        color[x] = true;

        if count.contains(&2) {
            return [true; 6];
        }
    } else if let Some(x) = count.iter().position(|&x| x == 3) {
        // There can be only one three of a kind becuase we already did 2 triples.
        // 3 of a kind
        color[x] = true;
    }

    color
}
