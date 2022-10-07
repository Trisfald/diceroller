use crate::number_expression::NumberExpression;
use clap::Parser;
use colored::*;
use rand::prelude::*;
use std::collections::BTreeMap;

mod number_expression;

#[derive(Parser)]
#[clap(version = "1.0", author = "Trisfald <trisfald@gmail.com>")]
struct Opts {
    /// Size of the dice pool.
    dice: NumberExpression,
    /// Difficulty of the roll.
    #[clap(default_value = "9")]
    difficulty: NumberExpression,
    /// Display only the final result.
    #[clap(short, long)]
    short: bool,
    /// Number of die's sides.
    #[clap(short, long, default_value = "12")]
    die: u8,
}

fn main() {
    let opts: Opts = Opts::parse();
    let difficulty = opts.difficulty;
    let die = opts.die;

    let mut rng = thread_rng();
    let mut successes = 0;
    let mut rolled_one = false;

    #[cfg(windows)]
    control::set_virtual_terminal(true).unwrap();

    // Roll loop.
    let mut success_history: Vec<u8> = Vec::new();
    for _ in 1..=opts.dice.into() {
        let result: u8 = rng.gen_range(1..=die);
        if result == 1 {
            rolled_one = true;
        }
        let success = if result >= difficulty.into() {
            successes += 1;
            success_history.push(result);
            true
        } else {
            false
        };
        if !opts.short {
            let color = if success {
                |x: ColoredString| x.green()
            } else {
                |x: ColoredString| x.red()
            };
            println!(
                "{} vs {}",
                format_args!("{: >2}", color(result.to_string().bold())),
                difficulty
            );
        }
    }

    // Result.
    if rolled_one && successes == 0 {
        println!("{}", "Critical failure!".bold().red());
    } else {
        let matches = get_matches(&success_history);
        let matches_str = match matches {
            1 => format!(" ({} match)", matches),
            i if i > 1 => format!(" ({} matches)", matches),
            _ => "".to_string(),
        };
        let color = if successes > 0 {
            |x: ColoredString| x.green()
        } else {
            |x: ColoredString| x.red()
        };
        println!(
            "{} {}{}",
            "Successes:".bold(),
            color(successes.to_string().bold()),
            matches_str.italic()
        );
    }
}

fn get_matches(successes: &[u8]) -> u8 {
    let mut counts = BTreeMap::new();
    for i in successes.iter() {
        *counts.entry(i).or_insert(0) += 1;
    }
    let mut matches = 0;
    for (_, count) in counts {
        if count > matches && count >= 2 {
            matches = count;
        }
    }
    if matches > 0 {
        matches - 1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches() {
        assert_eq!(get_matches(&[2, 3, 4]), 0);
        assert_eq!(get_matches(&[]), 0);
        assert_eq!(get_matches(&[1, 2, 1]), 1);
        assert_eq!(get_matches(&[1, 1, 3, 3, 1]), 2);
    }
}
