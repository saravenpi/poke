use crate::models::{Encounter, Species, TrainerState};
use crate::species;
use crate::storage;

const ENCOUNTER_TIMEOUT_SECS: u64 = 600;

/// Determines if an encounter should be shiny based on base rates and charm.
pub fn is_shiny(charm: bool) -> bool {
    if let Ok(val) = std::env::var("POKE_SHINY_RATE") {
        if let Ok(rate) = val.parse::<u32>() {
            return 0 == rand::random_range(0..rate.max(1));
        }
    }
    let denominator = if charm { 1024 } else { 4096 };
    0 == rand::random_range(0..denominator)
}

/// Calculates max HP from base HP and level using official formula.
pub fn calculate_hp(base_hp: u16, level: u8) -> u16 {
    let raw = (2 * (base_hp as u32) * (level as u32) / 100) + (level as u32) + 10;
    raw.min(999) as u16
}

/// Creates a new fresh encounter instance for a given species.
pub fn create_encounter(species: &Species, charm: bool) -> Encounter {
    let level = rand::random_range(3..=45);
    let max_hp = calculate_hp(species.base_hp, level);
    Encounter {
        species_id: species.id,
        name: species.name.clone(),
        slug: species.slug.clone(),
        level,
        max_hp,
        current_hp: max_hp,
        shiny: is_shiny(charm),
        spawned_at: storage::current_epoch_secs(),
        razz_fed: false,
    }
}

/// Retrieves active encounter or rolls a new one if expired or forced.
pub fn get_or_spawn_encounter(state: &mut TrainerState, force: bool, gen: Option<u8>) -> Encounter {
    let now = storage::current_epoch_secs();
    if !force {
        if let Some(ref active) = state.active_encounter {
            if now.saturating_sub(active.spawned_at) < ENCOUNTER_TIMEOUT_SECS {
                return active.clone();
            }
        }
    }

    let species = species::roll_encounter(gen);
    let encounter = create_encounter(species, state.inventory.shiny_charm);
    state.active_encounter = Some(encounter.clone());
    *state.seen.entry(species.id).or_insert(0) += 1;
    let _ = storage::save_state(state);
    encounter
}

/// Formats encounter description for display in terminal.
pub fn format_encounter(enc: &Encounter) -> String {
    let shiny_tag = if enc.shiny { " ✨ SHINY ✨" } else { "" };
    let species = species::find_by_query(&enc.slug);
    let type_info = match species {
        Some(s) if !s.type2.is_empty() => format!("[{} / {}]", s.type1, s.type2),
        Some(s) => format!("[{}]", s.type1),
        None => String::new(),
    };
    format!(
        "A wild {}{} (Lv. {}) appeared! {}\nUse 'poke catch' to throw a Poké Ball, or 'poke run'.",
        enc.name, shiny_tag, enc.level, type_info
    )
}
