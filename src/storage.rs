use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::TrainerState;

/// Computes the path to the state file under the user data directory.
pub fn state_file_path() -> PathBuf {
    let base = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("poke").join("state.json")
}

/// Loads the trainer state from disk or initializes defaults.
pub fn load_state() -> TrainerState {
    let path = state_file_path();
    if !path.exists() {
        return TrainerState::default();
    }
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return TrainerState::default(),
    };
    serde_json::from_str(&content).unwrap_or_default()
}

/// Saves the current trainer state to disk.
pub fn save_state(state: &TrainerState) -> Result<(), String> {
    let path = state_file_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let data = serde_json::to_string_pretty(state).map_err(|e| e.to_string())?;
    let tmp_path = path.with_extension("json.tmp");
    fs::write(&tmp_path, data).map_err(|e| e.to_string())?;
    fs::rename(&tmp_path, &path).map_err(|e| e.to_string())
}

/// Returns current Unix epoch timestamp in seconds.
pub fn current_epoch_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
