use crate::ball::BallType;

/// Streak milestones: every `period` days the daily bonus adds `count` balls.
pub const DAILY_MILESTONES: [(u32, BallType, u32); 3] =
    [(3, BallType::Great, 2), (7, BallType::Ultra, 1), (30, BallType::Master, 1)];

/// Poké Balls handed out with every daily bonus.
pub const DAILY_POKE_BALLS: u32 = 5;

/// Longest streak that still increases the daily cash.
const STREAK_CASH_CAP: u32 = 30;

/// Payout for a capture: harder species and higher levels pay more, new Pokédex entries and shinies pay extra.
pub fn capture_reward(catch_rate: u16, level: u8, shiny: bool, new_entry: bool) -> u32 {
    let rarity = 255u32.saturating_sub(u32::from(catch_rate)) * 3;
    let mut reward = 150 + u32::from(level) * 8 + rarity;
    if new_entry {
        reward += 500;
    }
    if shiny {
        reward += 2500;
    }
    reward
}

/// Payout for releasing a stored Pokémon to Professor Oak.
pub fn release_reward(level: u8, shiny: bool) -> u32 {
    25 + u32::from(level) * 5 + if shiny { 500 } else { 0 }
}

/// Daily cash, growing with the streak up to a cap.
pub fn daily_cash(streak: u32) -> u32 {
    200 + 20 * streak.min(STREAK_CASH_CAP)
}
