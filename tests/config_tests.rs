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
