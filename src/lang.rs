use std::collections::HashMap;
use std::io::Cursor;
use std::sync::OnceLock;

use serde::Deserialize;

use crate::species;

static NAMES: OnceLock<HashMap<usize, [String; 4]>> = OnceLock::new();

/// Language used to display Pokémon names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
pub enum Language {
    #[default]
    #[serde(rename = "en", alias = "english")]
    English,
    #[serde(rename = "ja", alias = "japanese")]
    Japanese,
    #[serde(rename = "fr", alias = "french")]
    French,
    #[serde(rename = "es", alias = "spanish")]
    Spanish,
    #[serde(rename = "de", alias = "german")]
    German,
}

impl Language {
    /// Two-letter code used in the config file.
    pub fn code(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Japanese => "ja",
            Self::French => "fr",
            Self::Spanish => "es",
            Self::German => "de",
        }
    }

    /// Column of this language in the translation table (English lives in species.csv).
    fn column(self) -> Option<usize> {
        match self {
            Self::English => None,
            Self::Japanese => Some(0),
            Self::French => Some(1),
            Self::Spanish => Some(2),
            Self::German => Some(3),
        }
    }
}

/// Parses the embedded translation table (ja, fr, es, de) once.
fn table() -> &'static HashMap<usize, [String; 4]> {
    NAMES.get_or_init(|| {
        const CSV_DATA: &str = include_str!("../data/names_i18n.csv");
        let mut reader = csv::Reader::from_reader(Cursor::new(CSV_DATA));
        let mut map = HashMap::with_capacity(905);
        for record in reader.records().flatten() {
            let id = record[0].parse().unwrap_or(0);
            map.insert(id, [1, 2, 3, 4].map(|i| record[i].to_string()));
        }
        map
    })
}

/// Name of a species in the given language, falling back to English.
pub fn pokemon_name(species_id: usize, lang: Language) -> String {
    let translated = lang.column().and_then(|c| table().get(&species_id).map(|names| names[c].clone()));
    translated
        .or_else(|| species::all_species().iter().find(|s| s.id == species_id).map(|s| s.name.clone()))
        .unwrap_or_else(|| format!("#{}", species_id))
}

/// Finds a species id from a name written in any supported language.
pub fn find_id(query: &str) -> Option<usize> {
    let wanted = query.trim().to_lowercase();
    table()
        .iter()
        .find(|(_, names)| names.iter().any(|n| n.to_lowercase() == wanted))
        .map(|(id, _)| *id)
}
