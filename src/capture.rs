use crate::ball::BallType;
use crate::models::{Encounter, Species};

/// Result of the four (or one, if critical) breakout checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Throw {
    pub caught: bool,
    pub shakes: u8,
    pub critical: bool,
}

/// Gen 8 modified catch rate X: HP, species rate, ball bonus, low-level bonus and berry.
pub fn catch_value(enc: &Encounter, species: &Species, ball: BallType, owned: bool) -> f64 {
    let m = f64::from(enc.max_hp.max(1));
    let h = f64::from(enc.current_hp).clamp(1.0, m);
    let hp = (3.0 * m - 2.0 * h) / (3.0 * m);
    let low_level = if enc.level < 21 { (30.0 - f64::from(enc.level)) / 10.0 } else { 1.0 };
    let berry = if enc.razz_fed { 1.5 } else { 1.0 };
    hp * f64::from(species.catch_rate) * ball.multiplier(enc, species, owned) * low_level * berry
}

/// Per-check hold chance Y / 65536 for a catch value.
pub fn hold_chance(x: f64) -> f64 {
    let y = (65536.0 / (255.0 / x.clamp(1.0, 255.0)).powf(0.1875)).floor();
    y.min(65535.0) / 65536.0
}

/// Probability of a successful normal (four check) capture.
pub fn capture_chance(x: f64) -> f64 {
    if x >= 255.0 {
        return 1.0;
    }
    hold_chance(x).powi(4)
}

/// Critical capture chance, scaled by how many species the trainer has caught.
pub fn critical_chance(x: f64, species_caught: usize) -> f64 {
    let p = match species_caught {
        0..=30 => 0.0,
        31..=150 => 0.5,
        151..=300 => 1.0,
        301..=450 => 1.5,
        451..=600 => 2.0,
        _ => 2.5,
    };
    (x.min(255.0) * p / 6.0).floor() / 256.0
}

/// Rolls the breakout checks for a catch value.
pub fn roll_throw(ball: BallType, x: f64, species_caught: usize) -> Throw {
    if ball == BallType::Master || x >= 255.0 {
        return Throw { caught: true, shakes: 3, critical: false };
    }
    let hold = hold_chance(x);
    if rand::random::<f64>() < critical_chance(x, species_caught) {
        let caught = rand::random::<f64>() < hold;
        return Throw { caught, shakes: if caught { 1 } else { 0 }, critical: true };
    }
    let passed = (0..4).take_while(|_| rand::random::<f64>() < hold).count() as u8;
    Throw { caught: passed == 4, shakes: passed.min(3), critical: false }
}
