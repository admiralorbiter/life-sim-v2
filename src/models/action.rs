use serde::{Serialize, Deserialize};
use super::Stage;
use super::event::StatEffect;

/// An action the player can select during Phase 1 (Plan).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub id: String,
    pub label: String,
    pub description: String,
    pub stages: Vec<Stage>,
    pub effects: Vec<StatEffect>,
    pub time_cost: u32,
    /// Engine hook: "emergency_fund_deposit", "reduce_bills", etc.
    #[serde(default)]
    pub special_effect: Option<String>,
}
