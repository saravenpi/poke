use poke::ball::BallType;
use poke::capture::{capture_chance, catch_value, critical_chance, roll_throw};
use poke::catch;
use poke::daily;
use poke::models::{Encounter, TrainerState};
use poke::species;

/// Builds a full-HP encounter for a species at a given level.
fn encounter(slug: &str, level: u8) -> Encounter {
    let s = species::find_by_query(slug).unwrap();
    Encounter {
        species_id: s.id,
        name: s.name.clone(),
        slug: s.slug.clone(),
        level,
        max_hp: 100,
        current_hp: 100,
        shiny: false,
        spawned_at: 0,
        razz_fed: false,
        turns: 0,
    }
}

/// Species lookup works by name and id.
#[test]
fn test_species_lookup() {
    let p = species::find_by_query("pikachu").unwrap();
    assert_eq!((p.id, p.type1.as_str()), (25, "Electric"));
    assert_eq!(species::find_by_query("25").unwrap().name, "Pikachu");
}

/// Gen 8 formula: full-HP Pidgey (rate 255, level 25) in a Poké Ball is X = 85, about 44% per throw.
#[test]
fn test_capture_chance_matches_gen8() {
    let enc = encounter("pidgey", 25);
    let pidgey = species::find_by_query("pidgey").unwrap();
    let x = catch_value(&enc, pidgey, BallType::Poke, false);
    assert!((x - 85.0).abs() < 1e-9);
    assert!((capture_chance(x) - 0.4375).abs() < 0.01);
    assert_eq!(capture_chance(255.0), 1.0);
}

/// Ball bonuses follow the Gen 8 table.
#[test]
fn test_ball_multipliers() {
    let mut enc = encounter("magikarp", 10);
    let s = species::find_by_query("magikarp").unwrap();
    assert_eq!(BallType::Net.multiplier(&enc, s, false), 3.5);
    assert_eq!(BallType::Nest.multiplier(&enc, s, false), 3.1);
    assert_eq!(BallType::Repeat.multiplier(&enc, s, false), 1.0);
    assert_eq!(BallType::Repeat.multiplier(&enc, s, true), 3.5);
    assert_eq!(BallType::Quick.multiplier(&enc, s, false), 5.0);
    enc.turns = 1;
    assert_eq!(BallType::Quick.multiplier(&enc, s, false), 1.0);
    enc.turns = 50;
    assert_eq!(BallType::Timer.multiplier(&enc, s, false), 4.0);
}

/// Critical capture needs more than 30 species caught; Master Balls never fail.
#[test]
fn test_critical_and_master() {
    assert_eq!(critical_chance(100.0, 30), 0.0);
    assert!(critical_chance(100.0, 31) > 0.0);
    assert!(roll_throw(BallType::Master, 1.0, 0).caught);
}

/// Ball keywords parse with common spellings.
#[test]
fn test_ball_parse() {
    assert_eq!(BallType::parse("Great-Ball"), Some(BallType::Great));
    assert_eq!(BallType::parse("quick"), Some(BallType::Quick));
    assert_eq!(BallType::parse("nope"), None);
}

/// Region names and numbers both set the generation limit; out of range is rejected.
#[test]
fn test_generation_parse_and_pool() {
    assert_eq!(species::parse_generation("unova"), Some(5));
    assert_eq!(species::parse_generation("3"), Some(3));
    assert_eq!(species::parse_generation("9"), None);
    for _ in 0..200 {
        assert!(species::roll_encounter(5, None).unwrap().gen <= 5);
    }
    assert!(species::roll_encounter(2, Some(5)).is_none());
}

/// Inventory consumption and daily streak logic.
#[test]
fn test_inventory_and_streak() {
    let mut state = TrainerState::default();
    assert!(state.inventory.take(BallType::Poke));
    assert_eq!(state.inventory.count(BallType::Poke), 14);
    assert!(!state.inventory.take(BallType::Master));
    assert!(daily::check_daily(&mut state).is_some());
    assert_eq!(state.streak, 1);
    assert!(daily::check_daily(&mut state).is_none());
    let _ = catch::flee;
}
