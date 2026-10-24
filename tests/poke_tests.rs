use poke::catch::{self, consume_ball};
use poke::daily;
use poke::models::{BallType, Encounter, Inventory, TrainerState};
use poke::species;

/// Tests species database loading and query lookups.
#[test]
fn test_species_lookup() {
    let pikachu = species::find_by_query("pikachu");
    assert!(pikachu.is_some());
    let p = pikachu.unwrap();
    assert_eq!(p.id, 25);
    assert_eq!(p.name, "Pikachu");
    assert_eq!(p.type1, "Electric");
}

/// Tests catch rate scoring for a Master Ball and normal Poke Ball.
#[test]
fn test_catch_rate_scoring() {
    let enc = Encounter {
        species_id: 25,
        name: "Pikachu".to_string(),
        slug: "pikachu".to_string(),
        level: 10,
        max_hp: 30,
        current_hp: 30,
        shiny: false,
        spawned_at: 1000,
        razz_fed: false,
    };
    let score_poke = catch::calculate_catch_rate(&enc, BallType::Poke, 190);
    assert!(score_poke > 0.0);
    let score_master = catch::calculate_catch_rate(&enc, BallType::Master, 190);
    assert!(score_master > 1000.0);
}

/// Tests inventory consumption and daily streak logic.
#[test]
fn test_inventory_and_streak() {
    let mut inv = Inventory::default();
    assert!(consume_ball(&mut inv, BallType::Poke));
    assert_eq!(inv.poke_balls, 14);

    let mut state = TrainerState::default();
    let msg = daily::check_daily(&mut state);
    assert!(msg.is_some());
    assert_eq!(state.streak, 1);
}
