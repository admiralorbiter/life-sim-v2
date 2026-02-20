use std::path::Path;
use crate::models::{EventCard, Action, Decision, Job, Ending};

/// All game data loaded from JSON files.
#[derive(Debug, Clone)]
pub struct GameData {
    pub events: Vec<EventCard>,
    pub actions: Vec<Action>,
    pub decisions: Vec<Decision>,
    pub jobs: Vec<Job>,
    pub endings: Vec<Ending>,
}

impl GameData {
    /// Load all game data from JSON files in the given directory.
    pub fn load_from_dir(data_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let events: Vec<EventCard> = load_json(data_dir, "events.json")?;
        let actions: Vec<Action> = load_json(data_dir, "actions.json")?;
        let decisions: Vec<Decision> = load_json(data_dir, "decisions.json")?;
        let jobs: Vec<Job> = load_json(data_dir, "jobs.json")?;
        let endings: Vec<Ending> = load_json(data_dir, "endings.json")?;

        println!("Loaded game data:");
        println!("  {} events", events.len());
        println!("  {} actions", actions.len());
        println!("  {} decisions", decisions.len());
        println!("  {} jobs", jobs.len());
        println!("  {} endings", endings.len());

        Ok(Self {
            events,
            actions,
            decisions,
            jobs,
            endings,
        })
    }
}

/// Load and deserialize a JSON file into the target type.
fn load_json<T: serde::de::DeserializeOwned>(
    dir: &Path,
    filename: &str,
) -> Result<T, Box<dyn std::error::Error>> {
    let path = dir.join(filename);
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
    let data: T = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse {}: {}", path.display(), e))?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_load_game_data() {
        let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
        let result = GameData::load_from_dir(&data_dir);
        assert!(result.is_ok(), "Should load all JSON data files: {:?}", result.err());

        let data = result.unwrap();
        assert!(!data.events.is_empty(), "Should have at least one event");
        assert!(!data.actions.is_empty(), "Should have at least one action");
        assert!(!data.decisions.is_empty(), "Should have at least one decision");
        assert!(!data.jobs.is_empty(), "Should have at least one job");
        assert!(!data.endings.is_empty(), "Should have at least one ending");
    }
}
