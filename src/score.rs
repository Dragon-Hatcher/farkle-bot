use std::fmt::Display;

use crate::dice_set::DiceSet;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Score(u16);

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
        return (0, Score(1500));
    }

    if count.contains(&6) {
        // 6 of a kind
        return (0, Score(3000));
    }

    if count.iter().filter(|&x| x == &3).count() == 2 {
        // 2 triplets
        return (0, Score(2500));
    }

    if count.iter().filter(|&x| x == &2).count() == 3 {
        // 3 pairs
        return (0, Score(1500));
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
            return (0, Score(1500));
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

    (count.iter().sum::<u8>(), Score(score))
}

// #[cfg(test)]
// mod test {
//     use super::{score_dice_set, Score};

//     #[test]
//     fn ones_and_fives() {
//         assert_eq!(score_dice_set(&mut [1, 1, 5, 3, 6, 4].into()), Score(250));
//         assert_eq!(score_dice_set(&mut [5, 1, 2, 3, 4, 6].into()), Score(150));
//     }

//     #[test]
//     fn big_scores() {
//         // 6 of a kind
//         assert_eq!(score_dice_set(&mut [2, 2, 2, 2, 2, 2].into()), Score(3000));
//         assert_eq!(score_dice_set(&mut [1, 1, 1, 1, 1, 1].into()), Score(3000));

//         // 5 of a kind
//         assert_eq!(score_dice_set(&mut [4, 4, 4, 4, 4, 6].into()), Score(2000));
//         assert_eq!(score_dice_set(&mut [2, 2, 2, 2, 2, 5].into()), Score(2050));

//         // 4 of a kind
//         assert_eq!(score_dice_set(&mut [2, 2, 2, 2, 1, 5].into()), Score(1150));
//         assert_eq!(score_dice_set(&mut [6, 6, 6, 6, 1, 1].into()), Score(1500));
//         assert_eq!(score_dice_set(&mut [6, 6, 6, 6, 2, 2].into()), Score(1500));

//         // straight
//         assert_eq!(score_dice_set(&mut [1, 2, 3, 4, 5, 6].into()), Score(1500));
//         assert_eq!(score_dice_set(&mut [6, 3, 2, 5, 4, 1].into()), Score(1500));

//         // 3 pairs
//         assert_eq!(score_dice_set(&mut [1, 1, 2, 2, 3, 3].into()), Score(1500));
//         assert_eq!(score_dice_set(&mut [6, 3, 4, 6, 4, 3].into()), Score(1500));

//         // 2 triples
//         assert_eq!(score_dice_set(&mut [5, 5, 5, 6, 6, 6].into()), Score(2500));
//         assert_eq!(score_dice_set(&mut [1, 2, 2, 1, 2, 1].into()), Score(2500));
//     }

//     #[test]
//     fn triples() {
//         assert_eq!(score_dice_set(&mut [2, 3, 2, 4, 2, 6].into()), Score(200));
//         assert_eq!(score_dice_set(&mut [1, 1, 1, 2, 3, 4].into()), Score(300));
//     }
// }
