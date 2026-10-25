use crate::ball::BallType;
use crate::econ;
use crate::models::TrainerState;
use crate::species;
use crate::sprites;
use crate::storage;

/// Computes high-level Pokédex completion statistics.
pub fn dex_summary(state: &TrainerState) -> String {
    let all = species::all_species();
    let total_species = all.len();
    let seen_count = state.seen.len();
    let caught_count = state.caught.len();
    let shinies_in_box = state.storage_box.iter().filter(|p| p.shiny).count();
    let seen_pct = (seen_count as f32 / total_species as f32) * 100.0;
    let caught_pct = (caught_count as f32 / total_species as f32) * 100.0;

    format!(
        "=== Pokédex Overview ===\n\
         Seen:         {}/{} ({:.1}%)\n\
         Caught:       {}/{} ({:.1}%)\n\
         Box Storage:  {} Pokémon ({} Shiny)\n\
         Streak:       {} days\n\
         Wallet:       {} ₽",
        seen_count, total_species, seen_pct,
        caught_count, total_species, caught_pct,
        state.storage_box.len(), shinies_in_box,
        state.streak, state.pokedollars
    )
}

/// Displays detailed Pokédex information and sprite for a specific Pokemon.
pub fn inspect_pokemon(query: &str, state: &TrainerState) -> Result<String, String> {
    let species = species::find_by_query(query)
        .ok_or_else(|| format!("Pokemon '{}' not found in National Dex.", query))?;

    let seen = state.seen.get(&species.id).copied().unwrap_or(0);
    let caught = state.caught.get(&species.id).copied().unwrap_or(0);
    let has_shiny = state
        .storage_box
        .iter()
        .any(|p| p.species_id == species.id && p.shiny);

    let sprite = sprites::render_sprite(&species.slug, has_shiny).unwrap_or_default();
    let type_str = if species.type2.is_empty() {
        species.type1.clone()
    } else {
        format!("{} / {}", species.type1, species.type2)
    };

    Ok(format!(
        "{}\n\
         No. {:03} {}\n\
         Type:       {}\n\
         Rarity:     {}\n\
         Generation: Gen {}\n\
         Catch Rate: {}\n\
         Base HP:    {}\n\
         Seen:       {} times\n\
         Caught:     {} times",
        sprite, species.id, species.name,
        type_str, species.rarity, species.gen,
        species.catch_rate, species.base_hp,
        seen, caught
    ))
}

/// Lists all Pokémon currently stored in the trainer box.
pub fn list_box(state: &TrainerState) -> String {
    if state.storage_box.is_empty() {
        return "Storage box is empty. Go catch some wild Pokémon!".to_string();
    }
    let mut out = format!("=== Storage Box ({} Pokémon) ===\n", state.storage_box.len());
    for (i, p) in state.storage_box.iter().enumerate() {
        let shiny = if p.shiny { "✨ " } else { "" };
        out.push_str(&format!(
            "[{}] #{:03} {}{} (Lv. {}) - Caught with {}\n",
            i + 1, p.species_id, shiny, p.name, p.level, p.ball_used
        ));
    }
    out.push_str("\nUse 'poke release <number>' to send to Professor Oak for ₽.");
    out
}

/// Releases a stored Pokémon in exchange for a cash reward.
pub fn release_pokemon(state: &mut TrainerState, index: usize) -> Result<String, String> {
    if index == 0 || index > state.storage_box.len() {
        return Err(format!("Invalid box index. Valid range: 1..{}", state.storage_box.len()));
    }
    let removed = state.storage_box.remove(index - 1);
    let reward = econ::release_reward(removed.level, removed.shiny);
    state.pokedollars = state.pokedollars.saturating_add(reward);
    storage::save_state(state)?;
    Ok(format!(
        "Sent {} (Lv. {}) to Professor Oak. Received {} ₽!",
        removed.name, removed.level, reward
    ))
}

/// Generates a single-line status summary for shell prompt integration.
pub fn status_line(state: &TrainerState) -> String {
    let inv = &state.inventory;
    format!(
        "{} ₽ | ⚪{} 🔵{} 🟡{} | Dex: {}/{}",
        state.pokedollars,
        inv.count(BallType::Poke),
        inv.count(BallType::Great),
        inv.count(BallType::Ultra),
        state.caught.len(),
        species::all_species().len()
    )
}
