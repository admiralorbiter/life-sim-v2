use serde::{Serialize, Deserialize};
use super::Stage;
use super::event::StatEffect;

/// A decision the player makes during Phase 2 (Commit).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Decision {
    pub id: String,
    pub stage: Stage,
    pub turn: u32,
    pub prompt: String,
    pub options: Vec<DecisionOption>,
}

/// One option within a decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionOption {
    pub label: String,
    pub description: String,
    pub effects: Vec<StatEffect>,
    #[serde(default)]
    pub grants_tag: Option<String>,
    /// If set, assigns monthly_bills to this value on the game state.
    #[serde(default)]
    pub sets_bills: Option<i32>,
    /// If set, assigns the player's current_job to the job with this ID.
    #[serde(default)]
    pub sets_job: Option<String>,
    /// If set, this option is only available if the player has this credential.
    #[serde(default)]
    pub requires_tag: Option<String>,
}
