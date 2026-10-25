use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use crate::ball::BallType;

/// Newest generation available in the sprite and species data.
pub const LATEST_GEN: u8 = 8;

/// Metadata for a single Pokemon species.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Species {
    pub id: usize,
    pub name: String,
    pub slug: String,
    pub gen: u8,
    pub type1: String,
    pub type2: String,
    pub catch_rate: u16,
    pub base_hp: u16,
    pub rarity: String,
    pub weight: u32,
}

/// Represents an ongoing wild encounter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Encounter {
    pub species_id: usize,
    pub name: String,
    pub slug: String,
    pub level: u8,
    pub max_hp: u16,
    pub current_hp: u16,
    pub shiny: bool,
    pub spawned_at: u64,
    pub razz_fed: bool,
    #[serde(default)]
    pub turns: u32,
}

/// An individual captured Pokemon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturedPokemon {
    pub uuid: String,
    pub species_id: usize,
    pub name: String,
    pub slug: String,
    pub level: u8,
    pub shiny: bool,
    pub caught_at: u64,
    pub ball_used: String,
}

/// Trainer inventory for balls, berries, and charms.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Inventory {
    pub balls: BTreeMap<BallType, u32>,
    pub razz_berries: u32,
    pub shiny_charm: bool,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            balls: BTreeMap::from([(BallType::Poke, 15), (BallType::Great, 5), (BallType::Ultra, 1)]),
            razz_berries: 3,
            shiny_charm: false,
        }
    }
}

impl Inventory {
    /// Number of balls of one type in the bag.
    pub fn count(&self, ball: BallType) -> u32 {
        self.balls.get(&ball).copied().unwrap_or(0)
    }

    /// Adds balls to the bag.
    pub fn add(&mut self, ball: BallType, qty: u32) {
        let slot = self.balls.entry(ball).or_insert(0);
        *slot = slot.saturating_add(qty);
    }

    /// Removes one ball, returning false when none are left.
    pub fn take(&mut self, ball: BallType) -> bool {
        match self.balls.get_mut(&ball) {
            Some(n) if *n > 0 => {
                *n -= 1;
                true
            }
            _ => false,
        }
    }
}

/// Complete trainer profile and save data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TrainerState {
    pub pokedollars: u32,
    pub inventory: Inventory,
    pub seen: HashMap<usize, u32>,
    pub caught: HashMap<usize, u32>,
    pub storage_box: Vec<CapturedPokemon>,
    pub active_encounter: Option<Encounter>,
    pub last_daily: String,
    pub streak: u32,
}

impl Default for TrainerState {
    fn default() -> Self {
        Self {
            pokedollars: 500,
            inventory: Inventory::default(),
            seen: HashMap::new(),
            caught: HashMap::new(),
            storage_box: Vec::new(),
            active_encounter: None,
            last_daily: String::new(),
            streak: 0,
        }
    }
}
