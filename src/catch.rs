use crate::ball::BallType;
use crate::capture::{self, Throw};
use crate::econ;
use crate::lang::{self, Language};
use crate::models::{CapturedPokemon, Encounter, Species, TrainerState};
use crate::species;
use crate::storage;

const FLEE_PERCENT: u32 = 25;

/// How a throw ended.
#[derive(Debug)]
pub enum End {
    Caught {
        pokemon: CapturedPokemon,
        reward: u32,
        new_entry: bool,
    },
    Escaped,
    Fled,
}

/// Full record of one ball throw, used to animate and report it.
#[derive(Debug)]
pub struct Attempt {
    pub ball: BallType,
    pub species_id: usize,
    pub throw: Throw,
    pub end: End,
}

/// Finalizes a capture and awards the payout.
fn record_capture(state: &mut TrainerState, enc: Encounter, species: &Species, ball: BallType) -> Result<End, String> {
    let new_entry = !state.caught.contains_key(&enc.species_id);
    let reward = econ::capture_reward(species.catch_rate, enc.level, enc.shiny, new_entry);
    state.pokedollars = state.pokedollars.saturating_add(reward);
    *state.caught.entry(enc.species_id).or_insert(0) += 1;
    let now = storage::current_epoch_secs();
    let pokemon = CapturedPokemon {
        uuid: format!("{}-{}-{}", now, enc.species_id, state.storage_box.len()),
        species_id: enc.species_id,
        name: enc.name,
        slug: enc.slug,
        level: enc.level,
        shiny: enc.shiny,
        caught_at: now,
        ball_used: ball.name().to_string(),
    };
    state.storage_box.push(pokemon.clone());
    state.active_encounter = None;
    storage::save_state(state)?;
    Ok(End::Caught { pokemon, reward, new_entry })
}

/// Handles a failed throw: the Pokémon may flee, unless a berry calmed it.
fn resolve_breakout(state: &mut TrainerState, mut enc: Encounter) -> Result<End, String> {
    let fled = !enc.razz_fed && rand::random_range(0..100) < FLEE_PERCENT;
    enc.razz_fed = false;
    enc.turns += 1;
    state.active_encounter = if fled { None } else { Some(enc) };
    storage::save_state(state)?;
    Ok(if fled { End::Fled } else { End::Escaped })
}

/// Executes a ball throw against the currently active encounter.
pub fn attempt_catch(state: &mut TrainerState, ball: BallType) -> Result<Attempt, String> {
    let enc = state
        .active_encounter
        .clone()
        .ok_or("No active wild Pokemon nearby. Run 'poke spawn' first!")?;
    let species = species::find_by_query(&enc.slug).ok_or("Unknown species in save data. Run 'poke run'.")?;
    if !state.inventory.take(ball) {
        return Err(format!("You have no {}s left! Visit 'poke shop'.", ball.name()));
    }

    let owned = state.caught.contains_key(&enc.species_id);
    let x = capture::catch_value(&enc, species, ball, owned);
    let throw = capture::roll_throw(ball, x, state.caught.len());
    let species_id = enc.species_id;
    let end = if throw.caught {
        record_capture(state, enc, species, ball)?
    } else {
        resolve_breakout(state, enc)?
    };
    Ok(Attempt { ball, species_id, throw, end })
}

/// Feeds a Razz Berry to calm the wild Pokemon.
pub fn feed_berry(state: &mut TrainerState, lang: Language) -> Result<String, String> {
    let enc = state.active_encounter.as_mut().ok_or("No active wild Pokemon to feed.")?;
    if state.inventory.razz_berries == 0 {
        return Err("You have no Razz Berries left! Visit 'poke shop'.".to_string());
    }
    state.inventory.razz_berries -= 1;
    enc.razz_fed = true;
    let name = lang::pokemon_name(enc.species_id, lang);
    storage::save_state(state)?;
    Ok(format!("Fed a Razz Berry to {}! It is calmer: catch chance x1.5, no fleeing next throw.", name))
}

/// Flees from the active encounter.
pub fn flee(state: &mut TrainerState) -> Result<String, String> {
    if state.active_encounter.take().is_none() {
        return Err("No active encounter to flee from.".to_string());
    }
    storage::save_state(state)?;
    Ok("Got away safely!".to_string())
}
