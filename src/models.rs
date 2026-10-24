use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

/// Supported Poke Ball types and multipliers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BallType {
    Poke,
    Great,
    Ultra,
    Master,
}

impl BallType {
    /// Returns multiplier for catch formula.
    pub fn multiplier(self) -> f32 {
        match self {
            Self::Poke => 1.0,
            Self::Great => 1.5,
            Self::Ultra => 2.0,
            Self::Master => 255.0,
        }
    }

    /// Returns display name of ball.
    pub fn name(self) -> &'static str {
        match self {
            Self::Poke => "Poké Ball",
            Self::Great => "Great Ball",
            Self::Ultra => "Ultra Ball",
            Self::Master => "Master Ball",
        }
    }

    /// Returns price in shop.
    pub fn price(self) -> u32 {
        match self {
            Self::Poke => 200,
            Self::Great => 600,
            Self::Ultra => 1200,
            Self::Master => 50000,
        }
    }
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

/// Trainer inventory for balls, berries, and badges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub poke_balls: u32,
    pub great_balls: u32,
    pub ultra_balls: u32,
    pub master_balls: u32,
    pub razz_berries: u32,
    pub shiny_charm: bool,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            poke_balls: 15,
            great_balls: 5,
            ultra_balls: 1,
            master_balls: 0,
            razz_berries: 3,
            shiny_charm: false,
        }
    }
}

/// Complete trainer profile and save data.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
