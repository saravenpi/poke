use rust_embed::RustEmbed;

pub mod anim;
pub mod bag;
pub mod ball;
pub mod capture;
pub mod catch;
pub mod cli;
pub mod config;
pub mod daily;
pub mod dex;
pub mod econ;
pub mod encounter;
pub mod models;
pub mod species;
pub mod sprites;
pub mod storage;

/// Embedded asset bundle containing all Pokemon Gen 8 sprites.
#[derive(RustEmbed)]
#[folder = "data/pokemon-gen8"]
pub struct Data;
