use serde::{Serialize, Deserialize};
use super::Stage;

/// A job the player can hold for income and growth.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub id: String,
    pub title: String,
    pub required_tags: Vec<String>,
    pub recommended_tags: Vec<String>,
    pub pay_per_turn: i32,
    pub stress_per_turn: i32,
    pub growth_rate: u32,
    #[serde(default)]
    pub growth_tag: Option<String>,
    pub stages: Vec<Stage>,
    pub description: String,
}
