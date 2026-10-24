use rust_embed::RustEmbed;

pub mod bag;
pub mod catch;
pub mod cli;
pub mod daily;
pub mod dex;
pub mod encounter;
pub mod models;
pub mod species;
pub mod sprites;
pub mod storage;

/// Embedded asset bundle containing all Pokemon Gen 8 sprites.
#[derive(RustEmbed)]
#[folder = "data/pokemon-gen8"]
pub struct Data;
