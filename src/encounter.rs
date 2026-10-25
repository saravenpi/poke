use crate::config::Config;
use crate::lang::{self, Language};
use crate::models::{Encounter, Species, TrainerState};
use crate::species;
use crate::storage;

/// Determines if an encounter should be shiny: 1/4096, 1/1024 with the charm, or a configured 1/`rate`.
pub fn is_shiny(charm: bool, rate_override: Option<u32>) -> bool {
    let denominator = rate_override.unwrap_or(if charm { 1024 } else { 4096 });
    rand::random_range(0..denominator.max(1)) == 0
}

/// Calculates max HP from base HP and level using official formula.
pub fn calculate_hp(base_hp: u16, level: u8) -> u16 {
    let raw = (2 * (base_hp as u32) * (level as u32) / 100) + (level as u32) + 10;
    raw.min(999) as u16
}

/// Creates a new fresh encounter instance for a given species.
pub fn create_encounter(species: &Species, charm: bool, shiny_rate: Option<u32>) -> Encounter {
    let level = rand::random_range(3..=45);
    let max_hp = calculate_hp(species.base_hp, level);
    Encounter {
        species_id: species.id,
        name: species.name.clone(),
        slug: species.slug.clone(),
        level,
        max_hp,
        current_hp: max_hp,
        shiny: is_shiny(charm, shiny_rate),
        spawned_at: storage::current_epoch_secs(),
        razz_fed: false,
        turns: 0,
    }
}

/// Retrieves the active encounter or rolls a new one if expired or forced.
pub fn get_or_spawn_encounter(
    state: &mut TrainerState,
    config: &Config,
    force: bool,
    only_gen: Option<u8>,
) -> Result<Encounter, String> {
    let now = storage::current_epoch_secs();
    let fresh = |a: &&Encounter| now.saturating_sub(a.spawned_at) < config.encounter_timeout_secs;
    if let Some(active) = state.active_encounter.as_ref().filter(|a| !force && fresh(a)) {
        return Ok(active.clone());
    }

    let max_gen = config.generation_limit()?;
    let species = species::roll_encounter(max_gen, only_gen)
        .ok_or_else(|| format!("No Pokemon of that generation within your limit (gen 1-{}).", max_gen))?;
    let encounter = create_encounter(species, state.inventory.shiny_charm, config.shiny_rate);
    state.active_encounter = Some(encounter.clone());
    *state.seen.entry(species.id).or_insert(0) += 1;
    storage::save_state(state)?;
    Ok(encounter)
}

/// Formats encounter description for display in terminal.
pub fn format_encounter(enc: &Encounter, lang: Language) -> String {
    let shiny_tag = if enc.shiny { " ✨ SHINY ✨" } else { "" };
    let species = species::find_by_query(&enc.slug);
    let type_info = match species {
        Some(s) if !s.type2.is_empty() => format!("[{} / {}]", s.type1, s.type2),
        Some(s) => format!("[{}]", s.type1),
        None => String::new(),
    };
    format!(
        "A wild {}{} (Lv. {}) appeared! {}\nUse 'poke catch' to throw a Poké Ball, or 'poke run'.",
        lang::pokemon_name(enc.species_id, lang), shiny_tag, enc.level, type_info
    )
}
