use serde::{Deserialize, Serialize};

use crate::models::{Encounter, Species};

/// Every ball the trainer can throw.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum BallType {
    Poke,
    Great,
    Ultra,
    Master,
    Net,
    Nest,
    Repeat,
    Quick,
    Timer,
}

/// All balls in shop display order.
pub const ALL_BALLS: [BallType; 9] = [
    BallType::Poke,
    BallType::Great,
    BallType::Ultra,
    BallType::Net,
    BallType::Nest,
    BallType::Repeat,
    BallType::Quick,
    BallType::Timer,
    BallType::Master,
];

impl BallType {
    /// Display name of the ball.
    pub fn name(self) -> &'static str {
        match self {
            Self::Poke => "Poké Ball",
            Self::Great => "Great Ball",
            Self::Ultra => "Ultra Ball",
            Self::Master => "Master Ball",
            Self::Net => "Net Ball",
            Self::Nest => "Nest Ball",
            Self::Repeat => "Repeat Ball",
            Self::Quick => "Quick Ball",
            Self::Timer => "Timer Ball",
        }
    }

    /// Short command keyword used by `catch` and `buy`.
    pub fn key(self) -> &'static str {
        match self {
            Self::Poke => "poke",
            Self::Great => "great",
            Self::Ultra => "ultra",
            Self::Master => "master",
            Self::Net => "net",
            Self::Nest => "nest",
            Self::Repeat => "repeat",
            Self::Quick => "quick",
            Self::Timer => "timer",
        }
    }

    /// Resolves a user keyword (`great`, `greatball`, `great-ball`) to a ball.
    pub fn parse(raw: &str) -> Option<Self> {
        let norm = raw.trim().to_lowercase().replace(['-', '_', ' '], "");
        let norm = norm.strip_suffix("ball").unwrap_or(&norm);
        ALL_BALLS.iter().copied().find(|b| b.key() == norm)
    }

    /// Shop price in Pokédollars.
    pub fn price(self) -> u32 {
        match self {
            Self::Poke => 200,
            Self::Great => 600,
            Self::Ultra => 800,
            Self::Master => 50_000,
            _ => 1_000,
        }
    }

    /// One-line description of when the ball shines.
    pub fn blurb(self) -> &'static str {
        match self {
            Self::Poke => "1x",
            Self::Great => "1.5x",
            Self::Ultra => "2x",
            Self::Net => "3.5x on Water and Bug types",
            Self::Nest => "up to 4x on low levels (below 30)",
            Self::Repeat => "3.5x on species you already own",
            Self::Quick => "5x on the first throw",
            Self::Timer => "grows each throw, up to 4x",
            Self::Master => "never fails",
        }
    }

    /// ANSI color code used by the throw animation.
    pub fn color(self) -> u8 {
        match self {
            Self::Poke => 31,
            Self::Great => 34,
            Self::Ultra => 33,
            Self::Master => 35,
            Self::Net => 36,
            Self::Nest => 32,
            Self::Repeat => 91,
            Self::Quick => 94,
            Self::Timer => 90,
        }
    }

    /// Gen 8 ball bonus (B) for this target.
    pub fn multiplier(self, enc: &Encounter, species: &Species, owned: bool) -> f64 {
        let level = f64::from(enc.level);
        match self {
            Self::Poke => 1.0,
            Self::Great => 1.5,
            Self::Ultra => 2.0,
            Self::Master => 255.0,
            Self::Net if is_type(species, &["Water", "Bug"]) => 3.5,
            Self::Nest if enc.level < 30 => (41.0 - level) / 10.0,
            Self::Repeat if owned => 3.5,
            Self::Quick if enc.turns == 0 => 5.0,
            Self::Timer => (1.0 + f64::from(enc.turns) * 1229.0 / 4096.0).min(4.0),
            _ => 1.0,
        }
    }
}

/// True when either of the species types is in the list.
fn is_type(species: &Species, types: &[&str]) -> bool {
    types.contains(&species.type1.as_str()) || types.contains(&species.type2.as_str())
}
