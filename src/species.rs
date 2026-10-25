use std::io::Cursor;
use std::sync::OnceLock;

use crate::models::{Species, LATEST_GEN};

static SPECIES_CACHE: OnceLock<Vec<Species>> = OnceLock::new();

/// Returns the complete list of all 905 Pokemon species.
pub fn all_species() -> &'static [Species] {
    SPECIES_CACHE.get_or_init(|| {
        const CSV_DATA: &str = include_str!("../data/species.csv");
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(Cursor::new(CSV_DATA));
        let mut list = Vec::with_capacity(905);
        for record in reader.records().flatten() {
            if record.len() < 10 {
                continue;
            }
            list.push(Species {
                id: record[0].parse().unwrap_or(1),
                name: record[1].to_string(),
                slug: record[2].to_string(),
                gen: record[3].parse().unwrap_or(1),
                type1: record[4].to_string(),
                type2: record[5].to_string(),
                catch_rate: record[6].parse().unwrap_or(45),
                base_hp: record[7].parse().unwrap_or(45),
                rarity: record[8].to_string(),
                weight: record[9].parse().unwrap_or(10),
            });
        }
        list
    })
}

/// Finds a species by ID or case-insensitive name/slug.
pub fn find_by_query(query: &str) -> Option<&'static Species> {
    let trimmed = query.trim();
    if let Ok(id) = trimmed.parse::<usize>() {
        return all_species().iter().find(|s| s.id == id);
    }
    let lower = trimmed.to_lowercase().replace(' ', "-");
    all_species()
        .iter()
        .find(|s| s.slug == lower || s.name.to_lowercase() == lower)
}

/// Resolves a generation number (1-8) or region name (`unova`) to a generation.
pub fn parse_generation(raw: &str) -> Option<u8> {
    let norm = raw.trim().to_lowercase();
    let by_region = ["kanto", "johto", "hoenn", "sinnoh", "unova", "kalos", "alola", "galar"]
        .iter()
        .position(|r| *r == norm)
        .map(|i| i as u8 + 1);
    by_region.or_else(|| norm.parse().ok()).filter(|g| (1..=LATEST_GEN).contains(g))
}

/// Selects a wild Pokemon by apparition weight, up to `max_gen` (or exactly `only_gen`).
pub fn roll_encounter(max_gen: u8, only_gen: Option<u8>) -> Option<&'static Species> {
    let pool: Vec<&Species> = all_species()
        .iter()
        .filter(|s| s.gen <= max_gen && only_gen.is_none_or(|g| s.gen == g))
        .collect();
    let total_weight: u32 = pool.iter().map(|s| s.weight).sum();
    if total_weight == 0 {
        return None;
    }
    let mut roll = rand::random_range(0..total_weight);
    pool.into_iter().find(|s| {
        let hit = roll < s.weight;
        roll = roll.saturating_sub(s.weight);
        hit
    })
}
