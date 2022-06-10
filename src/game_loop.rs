use std::ops::Index;

use colored::Colorize;

use crate::{
    dice_set::DiceSet,
    game_player::{GamePlayer, GameState, ResetAction, RollAction},
    score::Score,
    terminal::clear_terminal,
};

const WINNING_SCORE: Score = Score(10000);

pub fn run_game(print: bool, players: Vec<Box<dyn GamePlayer>>) {
    if players.len() < 2 {
        panic!("Too few players!");
    }

    let num_players = players.len();
    let p_names: Vec<&str> = players.iter().map(|p| p.name()).collect();
    let mut gs = GameState {
        scores: vec![Score(0); num_players],
        current_pot: Score(0),
        current_player: 0_usize,
        dice_left: 6,
    };

    fn print_scores(players: &[&str], gs: &GameState) {
        println!("Here are the current scores:");
        for (i, player) in players.iter().enumerate() {
            println!("{:<10} {}", player, gs.scores[i].to_string().green());
        }
    }

    fn print_pot(gs: &GameState) {
        println!(
            "The pot is {} and there {} {} {} remaining.",
            gs.current_pot.to_string().green(),
            if gs.dice_left == 1 { "is" } else { "are" },
            gs.dice_left.to_string().green(),
            if gs.dice_left == 1 { "die" } else { "dice" },
        )
    }

    fn inc_player(gs: &mut GameState) {
        gs.current_player += 1;
        if gs.current_player >= gs.scores.len() {
            gs.current_player = 0;
        }
    }

    while gs.scores.iter().all(|s| s < &WINNING_SCORE) {
        let current_player = &players[gs.current_player];

        if print {
            clear_terminal();
            println!("It's {}'s turn!", current_player.name().green());
            print_scores(&p_names, &gs);
            println!();
        }

        if gs.current_pot != Score(0) && gs.dice_left != 6 {
            if print {
                print_pot(&gs);
                println!(
                    "{}: Will you keep these dice or reset?",
                    current_player.name()
                )
            }

            match current_player.keep_or_reset(&gs) {
                ResetAction::Keep => {
                    println!(
                        "{}",
                        format!("{} wants to keep these dice.", current_player.name()).blue()
                    );
                }
                ResetAction::Reset => {
                    println!(
                        "{}",
                        format!("{} is going to start over.", current_player.name()).blue()
                    );
                    gs.current_pot = Score(0);
                    gs.dice_left = 6;
                }
            }

            if print {
                if !current_player.is_human() {
                    current_player.confirm()
                }
                clear_terminal();
            }
        }

        let mut first_loop = true;

        loop {
            let roll = DiceSet::new_rand(gs.dice_left);
            let (new_dice_left, score) = roll.score();

            if print {
                if !first_loop {
                    clear_terminal();
                }
                first_loop = false;
                println!("{} rolls {}!", current_player.name(), roll);
                if score == Score(0) {
                    println!("{}", "Farkle!".red())
                } else {
                    println!(
                        "+{}! The pot is now {} and {} {}! {} if you stop now.",
                        score.to_string().green(),
                        (gs.current_pot + &score).to_string().green(),
                        new_dice_left.to_string().green(),
                        if new_dice_left == 1 {
                            "die remains"
                        } else {
                            "dice remain"
                        },
                        (gs.scores[gs.current_player] + &gs.current_pot + &score)
                            .to_string()
                            .green()
                    )
                }
            }

            if score == Score(0) {
                gs.current_pot = Score(0);
                gs.dice_left = 6;
                inc_player(&mut gs);

                current_player.confirm();
                break;
            } else {
                gs.current_pot += &score;
                gs.dice_left = new_dice_left as usize;
            }

            if gs.current_pot < Score(500) && gs.scores[gs.current_player] == Score(0) {
                // The player isn't currently on the board so they have to play again.
                if print {
                    println!("{} isn't on the board yet so they have to roll again.", {
                        current_player.name()
                    })
                }

                current_player.confirm();
                continue;
            }

            let action = current_player.choose_action(&gs);

            if print {
                match action {
                    RollAction::Roll => println!(
                        "{}",
                        format!("{} wants to roll again!", current_player.name()).blue()
                    ),
                    RollAction::Stop => println!(
                        "{}",
                        format!(
                            "{} is going to stop there and get {} points!",
                            current_player.name(),
                            gs.current_pot
                        )
                        .blue()
                    ),
                }
                if !current_player.is_human() {
                    current_player.confirm()
                }
            }

            match action {
                RollAction::Roll => continue,
                RollAction::Stop => {
                    gs.scores[gs.current_player] += &gs.current_pot;
                    inc_player(&mut gs);
                    break;
                }
            }
        }
    }

    if print {
        clear_terminal();
        println!(
            "The winner is {}!",
            p_names[gs.scores.iter().position(|s| s > &Score(10000)).unwrap()]
        );
        println!();
        print_scores(&p_names, &gs);
    }
}
