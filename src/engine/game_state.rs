use serde::{Serialize, Deserialize};
use crate::models::{Stage, Job};

/// An entry in the player's decision log, used for the timeline recap.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionEntry {
    pub turn: u32,
    pub stage: Stage,
    pub description: String,
    pub impact: String,
}

/// The complete game state, held in server memory.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    pub current_stage: Stage,
    pub current_turn: u32,
    pub total_turns: u32,

    // Resources
    pub money: i32,
    pub stress: i32,
    pub support: i32,
    pub time_slots: u32,
    pub credentials: Vec<String>,

    // Tracking
    pub current_job: Option<Job>,
    pub monthly_bills: i32,
    pub emergency_fund: i32,
    pub decision_log: Vec<DecisionEntry>,
    pub used_event_ids: Vec<String>,

    // Meta
    pub seed: String,
}

impl GameState {
    /// Create a new game with default starting values (Stage A: Middle School).
    pub fn new(seed: String) -> Self {
        Self {
            current_stage: Stage::MiddleSchool,
            current_turn: 1,
            total_turns: 16, // 3-4 + 5-6 + 2-3 + 5-6 turns across stages

            money: 100,
            stress: 20,
            support: 5,
            time_slots: 3,
            credentials: Vec::new(),

            current_job: None,
            monthly_bills: 0,
            emergency_fund: 0,
            decision_log: Vec::new(),
            used_event_ids: Vec::new(),

            seed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game_defaults() {
        let state = GameState::new("TEST".to_string());
        assert_eq!(state.current_stage, Stage::MiddleSchool);
        assert_eq!(state.current_turn, 1);
        assert_eq!(state.money, 100);
        assert_eq!(state.stress, 20);
        assert_eq!(state.support, 5);
        assert_eq!(state.time_slots, 3);
        assert!(state.credentials.is_empty());
        assert!(state.current_job.is_none());
        assert_eq!(state.seed, "TEST");
    }
}
