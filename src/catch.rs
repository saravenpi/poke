use crate::models::{BallType, CapturedPokemon, Encounter, Inventory, TrainerState};
use crate::species;
use crate::storage;

/// Outcome of a catch attempt.
#[derive(Debug)]
pub enum CatchOutcome {
    Caught {
        pokemon: CapturedPokemon,
        reward: u32,
    },
    Escaped {
        shakes: u8,
        slug: String,
    },
    Fled {
        shakes: u8,
        name: String,
    },
}

/// Consumes one ball of the given type from inventory.
pub fn consume_ball(inv: &mut Inventory, ball: BallType) -> bool {
    let slot = match ball {
        BallType::Poke => &mut inv.poke_balls,
        BallType::Great => &mut inv.great_balls,
        BallType::Ultra => &mut inv.ultra_balls,
        BallType::Master => &mut inv.master_balls,
    };
    if *slot == 0 {
        return false;
    }
    *slot -= 1;
    true
}

/// Computes the catch chance score using the authentic capture formula.
pub fn calculate_catch_rate(enc: &Encounter, ball: BallType, base_rate: u16) -> f32 {
    let m = enc.max_hp as f32;
    let h = enc.current_hp as f32;
    let hp_factor = (3.0 * m - 2.0 * h) / (3.0 * m);
    let berry_mult = if enc.razz_fed { 1.5 } else { 1.0 };
    hp_factor * (base_rate as f32) * ball.multiplier() * berry_mult
}

/// Finalizes capture state and awards payout.
fn record_capture(state: &mut TrainerState, enc: Encounter, ball: BallType) -> Result<CatchOutcome, String> {
    let reward = 100 + (enc.level as u32 * 10) + if enc.shiny { 5000 } else { 0 };
    state.pokedollars += reward;
    *state.caught.entry(enc.species_id).or_insert(0) += 1;
    let captured = CapturedPokemon {
        uuid: format!("{}-{}", storage::current_epoch_secs(), enc.species_id),
        species_id: enc.species_id,
        name: enc.name.clone(),
        slug: enc.slug.clone(),
        level: enc.level,
        shiny: enc.shiny,
        caught_at: storage::current_epoch_secs(),
        ball_used: ball.name().to_string(),
    };
    state.storage_box.push(captured.clone());
    state.active_encounter = None;
    storage::save_state(state).map_err(|e| e.to_string())?;
    Ok(CatchOutcome::Caught {
        pokemon: captured,
        reward,
    })
}

/// Handles failed catch shake mechanics and flee chances.
fn resolve_breakout(state: &mut TrainerState, mut enc: Encounter) -> Result<CatchOutcome, String> {
    let fled = !enc.razz_fed && (rand::random_range(0..100) < 25);
    enc.razz_fed = false;
    let shakes = rand::random_range(0..3);

    if fled {
        let name = enc.name.clone();
        state.active_encounter = None;
        storage::save_state(state).map_err(|e| e.to_string())?;
        Ok(CatchOutcome::Fled { shakes, name })
    } else {
        state.active_encounter = Some(enc.clone());
        storage::save_state(state).map_err(|e| e.to_string())?;
        Ok(CatchOutcome::Escaped {
            shakes,
            slug: enc.slug,
        })
    }
}

/// Executes a ball throw against the currently active encounter.
pub fn attempt_catch(state: &mut TrainerState, ball: BallType) -> Result<CatchOutcome, String> {
    let enc = state
        .active_encounter
        .clone()
        .ok_or_else(|| "No active wild Pokemon nearby. Run 'poke spawn' first!".to_string())?;

    if !consume_ball(&mut state.inventory, ball) {
        return Err(format!("You have no {}s left! Visit 'poke shop'.", ball.name()));
    }

    let species = species::find_by_query(&enc.slug);
    let base_rate = species.map(|s| s.catch_rate).unwrap_or(45);
    let score = calculate_catch_rate(&enc, ball, base_rate);

    let caught = if ball == BallType::Master || score >= 255.0 {
        true
    } else {
        let b = 65536.0 / (255.0 / score).powf(0.1875);
        let b_val = b.clamp(1.0, 65535.0) as u32;
        (0..4).all(|_| rand::random_range(0..65536) < b_val)
    };

    if caught {
        record_capture(state, enc, ball)
    } else {
        resolve_breakout(state, enc)
    }
}

/// Feeds a Razz Berry to calm the wild Pokemon.
pub fn feed_berry(state: &mut TrainerState) -> Result<String, String> {
    let mut enc = state
        .active_encounter
        .clone()
        .ok_or_else(|| "No active wild Pokemon to feed.".to_string())?;

    if state.inventory.razz_berries == 0 {
        return Err("You have no Razz Berries left! Visit 'poke shop'.".to_string());
    }
    state.inventory.razz_berries -= 1;
    enc.razz_fed = true;
    let name = enc.name.clone();
    state.active_encounter = Some(enc);
    storage::save_state(state).map_err(|e| e.to_string())?;
    Ok(format!("Fed a Razz Berry to {}! Catch chance increased.", name))
}

/// Flees from the active encounter.
pub fn flee(state: &mut TrainerState) -> Result<String, String> {
    if state.active_encounter.is_none() {
        return Err("No active encounter to flee from.".to_string());
    }
    state.active_encounter = None;
    storage::save_state(state).map_err(|e| e.to_string())?;
    Ok("Got away safely!".to_string())
}
