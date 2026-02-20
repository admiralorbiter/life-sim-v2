pub mod action;
pub mod decision;
pub mod ending;
pub mod event;
pub mod job;

// Re-export common types
pub use action::Action;
pub use decision::Decision;
pub use ending::Ending;
pub use event::{EventCard, Rarity};
pub use job::Job;

use serde::{Serialize, Deserialize};

/// The four life stages of the game.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum Stage {
    MiddleSchool,
    HighSchool,
    PostHigh,
    EarlyAdult,
}

impl std::fmt::Display for Stage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stage::MiddleSchool => write!(f, "Middle School"),
            Stage::HighSchool => write!(f, "High School"),
            Stage::PostHigh => write!(f, "Post-High"),
            Stage::EarlyAdult => write!(f, "Early Adult"),
        }
    }
}
