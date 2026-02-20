use serde::{Serialize, Deserialize};
use super::Stage;

/// A life event card drawn during Phase 3 of each turn.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventCard {
    pub id: String,
    pub title: String,
    pub flavor_text: String,
    pub stages: Vec<Stage>,
    pub rarity: Rarity,
    pub options: Vec<EventOption>,
}

/// One response option on an event card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventOption {
    pub label: String,
    pub description: String,
    pub effects: Vec<StatEffect>,
    #[serde(default)]
    pub delayed_effects: Option<Vec<DelayedEffect>>,
    #[serde(default)]
    pub requires_support: Option<i32>,
}

/// A single stat modification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatEffect {
    pub stat: StatType,
    pub delta: i32,
    #[serde(default)]
    pub tag: Option<String>,
}

/// A stat effect that triggers N turns in the future.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelayedEffect {
    pub turns_until: u32,
    pub effects: Vec<StatEffect>,
}

/// Which player stat is affected.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum StatType {
    Money,
    Stress,
    Support,
    TimeSlots,
    Credentials,
}

/// Card rarity tier.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
}
