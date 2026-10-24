use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::TrainerState;

/// Returns the current day index since epoch to compute day deltas safely.
pub fn current_day_number() -> u64 {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    secs / 86400
}

/// Checks and awards daily login bonus if not claimed today.
pub fn check_daily(state: &mut TrainerState) -> Option<String> {
    let today = current_day_number().to_string();
    if state.last_daily == today {
        return None;
    }

    let prev_day: u64 = state.last_daily.parse().unwrap_or(0);
    let curr_day: u64 = current_day_number();

    if curr_day == prev_day + 1 {
        state.streak += 1;
    } else {
        state.streak = 1;
    }
    state.last_daily = today;

    let base_reward = 250 + (state.streak * 25);
    state.pokedollars += base_reward;
    state.inventory.poke_balls += 5;

    let mut bonus_note = String::new();
    if state.streak % 7 == 0 {
        state.inventory.ultra_balls += 1;
        bonus_note = " +1 Ultra Ball!".to_string();
    } else if state.streak % 3 == 0 {
        state.inventory.great_balls += 2;
        bonus_note = " +2 Great Balls!".to_string();
    }

    Some(format!(
        "Daily bonus claimed: +{} ₽, +5 Poké Balls!{} (Streak: {} days)",
        base_reward, bonus_note, state.streak
    ))
}
