use poke::config::Config;

/// Config accepts a number or a region for the generation limit and rejects unknown keys.
#[test]
fn test_config_parsing() {
    let cfg: Config = serde_yaml_ng::from_str("max_gen: unova\nanimation: false").unwrap();
    assert_eq!(cfg.generation_limit(), Ok(5));
    assert!(!cfg.animation);
    let cfg: Config = serde_yaml_ng::from_str("max_gen: 3").unwrap();
    assert_eq!(cfg.generation_limit(), Ok(3));
    assert!(serde_yaml_ng::from_str::<Config>("max_gne: 3").is_err());
    assert!(serde_yaml_ng::from_str::<Config>("max_gen: 12").unwrap().generation_limit().is_err());
}

/// Names resolve in every supported language and can be looked up from any of them.
#[test]
fn test_localized_names() {
    use poke::lang::{pokemon_name, Language};
    assert_eq!(pokemon_name(1, Language::French), "Bulbizarre");
    assert_eq!(pokemon_name(1, Language::German), "Bisasam");
    assert_eq!(pokemon_name(1, Language::Spanish), "Bulbasaur");
    assert_eq!(pokemon_name(25, Language::Japanese), "ピカチュウ");
    assert_eq!(pokemon_name(25, Language::English), "Pikachu");
    assert_eq!(poke::species::find_by_query("ピカチュウ").unwrap().id, 25);
    assert_eq!(poke::species::find_by_query("bisasam").unwrap().id, 1);
    let cfg: Config = serde_yaml_ng::from_str("language: french").unwrap();
    assert_eq!(cfg.language, Language::French);
}

/// Encounter formatting guides the player on catch syntax including balls.
#[test]
fn test_encounter_prompt_mentions_ball_syntax() {
    use poke::encounter::format_encounter;
    use poke::lang::Language;
    use poke::models::Encounter;

    let enc = Encounter {
        species_id: 1,
        name: "Bulbasaur".into(),
        slug: "bulbasaur".into(),
        level: 5,
        max_hp: 20,
        current_hp: 20,
        shiny: false,
        spawned_at: 0,
        razz_fed: false,
        turns: 0,
    };
    let formatted = format_encounter(&enc, Language::English);
    assert!(formatted.contains("poke catch [ball]"));
}
