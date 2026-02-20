use serde::{Serialize, Deserialize};

/// An ending the player can reach at the end of the game.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ending {
    pub id: String,
    pub title: String,
    pub conditions: EndingConditions,
    pub narrative: String,
    pub reflection: String,
}

/// Threshold conditions that determine which ending applies.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndingConditions {
    #[serde(default)]
    pub money: Option<ThresholdCondition>,
    #[serde(default)]
    pub stress: Option<ThresholdCondition>,
    #[serde(default)]
    pub support: Option<ThresholdCondition>,
    #[serde(default)]
    pub credentials: Option<CountCondition>,
}

/// A numeric min/max threshold.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdCondition {
    #[serde(default)]
    pub min: Option<i32>,
    #[serde(default)]
    pub max: Option<i32>,
}

/// A count-based condition (e.g., minimum number of credentials).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountCondition {
    #[serde(default)]
    pub min_count: Option<u32>,
}
