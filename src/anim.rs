use std::io::{self, IsTerminal, Write};
use std::thread::sleep;
use std::time::Duration;

use crate::ball::BallType;
use crate::config::Config;
use crate::lang;
use crate::catch::{Attempt, End};

const WOBBLE: [&str; 4] = ["◐", "◓", "◑", "◒"];
const TRAVEL_STEPS: usize = 12;

/// Redraws the current terminal line, then waits.
fn frame(line: &str, ms: u64) {
    print!("\r\x1b[2K{}", line);
    let _ = io::stdout().flush();
    sleep(Duration::from_millis(ms));
}

/// Colors a glyph with the ball's color.
fn tint(ball: BallType, glyph: &str) -> String {
    format!("\x1b[{}m{}\x1b[0m", ball.color(), glyph)
}

/// Flies the ball across the screen, then lets it land.
fn throw_ball(ball: BallType) {
    for step in 0..TRAVEL_STEPS {
        frame(&format!("{}{}", " ".repeat(step * 2), tint(ball, "●")), 55);
    }
    frame(&format!("{}{}", " ".repeat(TRAVEL_STEPS * 2), tint(ball, "◎")), 700);
}

/// Wobbles the ball once per shake, adding a dot each time.
fn shake_ball(ball: BallType, shakes: u8) {
    let pad = " ".repeat(TRAVEL_STEPS * 2);
    for n in 1..=shakes {
        for glyph in WOBBLE {
            frame(&format!("{}{} {}", pad, tint(ball, glyph), ".".repeat(n as usize)), 110);
        }
        frame(&format!("{}{} {}", pad, tint(ball, "●"), ".".repeat(n as usize)), 750);
    }
    frame(&format!("{}{}", pad, tint(ball, "●")), 900);
    println!();
}

/// Plays the throw animation when attached to a terminal, plain text otherwise.
fn animate(attempt: &Attempt, enabled: bool) {
    if !enabled || !io::stdout().is_terminal() {
        println!("{}", "*shake* ".repeat(attempt.throw.shakes as usize).trim_end());
        return;
    }
    throw_ball(attempt.ball);
    if attempt.throw.critical {
        frame("A critical capture!", 900);
        println!();
    }
    shake_ball(attempt.ball, attempt.throw.shakes);
}

/// Animates a throw and prints how it ended.
pub fn play(attempt: &Attempt, config: &Config) {
    println!("You threw a {}!", attempt.ball.name());
    animate(attempt, config.animation);
    let name = lang::pokemon_name(attempt.species_id, config.language);
    match &attempt.end {
        End::Caught { reward, new_entry, .. } => {
            println!("*click*! Gotcha! {} was caught!", name);
            if *new_entry {
                println!("New Pokédex entry: {}!", name);
            }
            println!("+{} ₽ earned.", reward);
        }
        End::Escaped => {
            println!("Oh no! {} broke free!", name);
            println!("Use 'poke catch [ball]' to try again or 'poke berry'.");
        }
        End::Fled => println!("Oh no! {} broke free and fled into the tall grass!", name),
    }
}
