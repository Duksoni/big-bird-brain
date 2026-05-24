use serde::Serialize;

use super::{Bird, Bonus, DraftCombination, Habitat, Resource};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftSuggestion {
    pub kept_birds: Vec<Bird>,
    pub kept_bonus: Bonus,
    pub kept_food: Vec<Resource>,
    pub score: f64,
    pub reasoning: String,
    pub tags: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    PlayBird { bird: Box<Bird>, habitat: Habitat },
    GainFood { resources: Vec<Resource> },
    LayEggs { bird: Box<Bird>, count: u8 },
    DrawCards { count: u8 },
}

#[derive(Serialize)]
pub struct MoveSuggestion {
    pub action: Action,
    pub score: f64,
    pub reasoning: String,
    pub tags: Vec<String>,
}

impl From<DraftCombination> for DraftSuggestion {
    fn from(value: DraftCombination) -> Self {
        Self {
            kept_birds: value.kept_birds,
            kept_bonus: value.bonus_card,
            kept_food: value.kept_food,
            score: value.score,
            reasoning: value.reasons.join("; "),
            tags: value.tags,
        }
    }
}

impl MoveSuggestion {
    pub fn new(action: Action, score: f64, reasoning: String, tags: Vec<String>) -> Self {
        Self {
            action,
            score,
            reasoning,
            tags,
        }
    }
}
