use crate::econ::{self, DAILY_MILESTONES, DAILY_POKE_BALLS};
use crate::models::TrainerState;
use crate::storage;

/// Returns the current day index since epoch to compute day deltas safely.
pub fn current_day_number() -> u64 {
    storage::current_epoch_secs() / 86400
}

/// Checks and awards the daily login bonus if not claimed today.
pub fn check_daily(state: &mut TrainerState) -> Option<String> {
    let today = current_day_number();
    let prev: u64 = state.last_daily.parse().unwrap_or(0);
    if prev == today {
        return None;
    }

    state.streak = if prev + 1 == today { state.streak + 1 } else { 1 };
    state.last_daily = today.to_string();

    let cash = econ::daily_cash(state.streak);
    state.pokedollars = state.pokedollars.saturating_add(cash);
    state.inventory.add(crate::ball::BallType::Poke, DAILY_POKE_BALLS);

    let bonus: String = DAILY_MILESTONES
        .iter()
        .filter(|(period, _, _)| state.streak.is_multiple_of(*period))
        .map(|(_, ball, count)| {
            state.inventory.add(*ball, *count);
            format!(" +{} {}{}!", count, ball.name(), if *count > 1 { "s" } else { "" })
        })
        .collect();

    Some(format!(
        "Daily bonus claimed: +{} ₽, +{} Poké Balls!{} (Streak: {} days)",
        cash, DAILY_POKE_BALLS, bonus, state.streak
    ))
}
