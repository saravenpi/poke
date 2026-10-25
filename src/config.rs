use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

use crate::models::LATEST_GEN;
use crate::species;

/// A generation given as a number (`5`) or a region name (`unova`).
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum GenSetting {
    Number(u8),
    Region(String),
}

/// User settings read from `~/.config/poke/config.yml` or `~/.poke.yml`.
#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub max_gen: GenSetting,
    pub animation: bool,
    pub shiny_rate: Option<u32>,
    pub encounter_timeout_secs: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_gen: GenSetting::Number(LATEST_GEN),
            animation: true,
            shiny_rate: None,
            encounter_timeout_secs: 600,
        }
    }
}

impl Config {
    /// Newest generation that may spawn, or an error naming the bad value.
    pub fn generation_limit(&self) -> Result<u8, String> {
        let parsed = match &self.max_gen {
            GenSetting::Number(n) => species::parse_generation(&n.to_string()),
            GenSetting::Region(name) => species::parse_generation(name),
        };
        parsed.ok_or_else(|| format!("config: max_gen must be 1-{} or a region name", LATEST_GEN))
    }
}

/// Config locations in priority order.
pub fn config_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Some(home) = dirs::home_dir() {
        paths.push(home.join(".config").join("poke").join("config.yml"));
        paths.push(home.join(".poke.yml"));
    }
    paths
}

/// Loads the first config file found, falling back to defaults with a warning on errors.
pub fn load_config() -> Config {
    let Some(path) = config_paths().into_iter().find(|p| p.is_file()) else {
        return Config::default();
    };
    let parsed = fs::read_to_string(&path)
        .map_err(|e| e.to_string())
        .and_then(|text| serde_yaml_ng::from_str::<Config>(&text).map_err(|e| e.to_string()));
    parsed.unwrap_or_else(|err| {
        eprintln!("poke: ignoring {}: {}", path.display(), err);
        Config::default()
    })
}
