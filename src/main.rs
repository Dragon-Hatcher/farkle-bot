use std::io::{stdin, stdout, Write};

use crate::dice_set::DiceSet;

mod dice_set;
mod score;

fn main() {
    loop {
        let mut i: DiceSet = read_dice_set("Enter a dice set: ");
        let (dice_left, score) = i.score();
        println!("+{} \t| {} dice left", score, dice_left);
    }
}

fn read_dice_set(message: &str) -> DiceSet {
    print!("{}", message);
    let _ = stdout().flush();
    let mut out = String::new();
    loop {
        stdin().read_line(&mut out).expect("Non UTF-8? Really?");
        let resp = out.parse::<DiceSet>();
        match resp {
            Ok(ds) => return ds,
            Err(_) => {
                print!("{}", message);
                let _ = stdout().flush();
            }
        }
    }
}
