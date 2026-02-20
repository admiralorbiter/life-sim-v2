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
}
