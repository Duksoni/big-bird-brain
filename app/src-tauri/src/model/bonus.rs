use serde::{Deserialize, Serialize};

use crate::model::PlayedBird;

use super::{
    Bird, BirdResource,
    Habitat::{self, *},
    NestType, Player, PlayerState,
};
use BonusCondition::*;
use MiscellaneousBonus::*;

const ANATOMIST_WORDS: [&str; 18] = [
    "beak", "belly", "bill", "breast", "cap", "chin", "collar", "crest", "crown", "eye", "face",
    "head", "neck", "rump", "shoulder", "tail", "throat", "wing",
];

const CARTOGRAPHER_WORDS: [&str; 16] = [
    "american",
    "atlantic",
    "baltimore",
    "california",
    "canada",
    "carolina",
    "chihuahua",
    "eastern",
    "inca",
    "mississippi",
    "mountain",
    "northern",
    "prairie",
    "sandhill",
    "savannah",
    "western",
];

const PHOTOGRAPHER_WORDS: [&str; 24] = [
    "ash",
    "black",
    "blue",
    "bronze",
    "brown",
    "cerulean",
    "chestnut",
    "ferruginous",
    "gold",
    "gray",
    "green",
    "indigo",
    "lazuli",
    "purple",
    "red",
    "rose",
    "roseate",
    "ruby",
    "ruddy",
    "rufous",
    "snowy",
    "violet",
    "white",
    "yellow",
];

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum BonusScoring {
    Threshold {
        low_threshold: u8,
        high_threshold: u8,
        low_reward_points: u8,
        high_reward_points: u8,
    },
    PerBird(u8),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum BonusCondition {
    NestTypeCount(NestType),
    HabitatCount(Habitat),
    FoodCost(BirdResource),
    WingspanGreater(u16),
    WingspanLess(u16),
    Miscellaneous(MiscellaneousBonus), // For complex name-based bonuses
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum MiscellaneousBonus {
    Anatomist,
    BackyardBirder,
    BirdCounter,
    Cartographer,
    Ecologist,
    Falconer,
    Historian,
    Oologist,
    Photographer,
    VisionaryLeader,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Bonus {
    id: i64,
    condition: BonusCondition,
    scoring: BonusScoring,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BonusDisplay {
    id: i64,
    name: String,
    description: String,
    condition: BonusCondition,
    scoring: BonusScoring,
}

impl Default for BonusScoring {
    fn default() -> Self {
        Self::Threshold {
            low_threshold: 0,
            high_threshold: 0,
            low_reward_points: 0,
            high_reward_points: 0,
        }
    }
}

impl BonusCondition {
    pub fn matches_played_bird(&self, played_bird: &PlayedBird) -> bool {
        match self {
            Miscellaneous(Oologist) => played_bird.laid_eggs() > 0,
            _ => self.matches_bird(played_bird.bird()),
        }
    }

    pub fn matches_bird(&self, bird: &Bird) -> bool {
        match self {
            NestTypeCount(nest) => bird.nest_type() == *nest,
            HabitatCount(habitat) => bird.habitats().contains(habitat),
            FoodCost(resource) => bird.cost().contains(resource),
            WingspanGreater(limit) => bird.wingspan() > *limit,
            WingspanLess(limit) => bird.wingspan() < *limit,
            Miscellaneous(misc) => match misc {
                Anatomist => ANATOMIST_WORDS
                    .iter()
                    .any(|word| bird.name().to_lowercase().contains(word)),
                BackyardBirder => bird.victory_points() <= 3,
                BirdCounter => bird.is_flocking(),
                Cartographer => CARTOGRAPHER_WORDS
                    .iter()
                    .any(|word| bird.name().to_lowercase().contains(word)),
                Falconer => bird.is_predator(),
                Historian => bird.name().contains("'s"),
                Photographer => PHOTOGRAPHER_WORDS
                    .iter()
                    .any(|word| bird.name().to_lowercase().contains(word)),
                Ecologist | Oologist | VisionaryLeader => false, // Not applicable to single bird; handled elsewhere
            },
        }
    }
}

impl Bonus {
    pub fn new(id: i64, condition: BonusCondition, scoring: BonusScoring) -> Self {
        Self {
            id,
            condition,
            scoring,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn matches_played_bird(&self, played_bird: &PlayedBird) -> bool {
        self.condition.matches_played_bird(played_bird)
    }

    pub fn matches_bird(&self, bird: &Bird) -> bool {
        self.condition.matches_bird(bird)
    }

    pub fn evaluate(&self, player: &PlayerState) -> u8 {
        let count = match self.condition {
            Miscellaneous(VisionaryLeader) => player.hand().len() as u8,
            _ => self.count_matching_birds(player) as u8,
        };

        match self.scoring {
            BonusScoring::Threshold {
                low_threshold,
                high_threshold,
                low_reward_points,
                high_reward_points,
            } => {
                if count >= high_threshold {
                    high_reward_points
                } else if count >= low_threshold {
                    low_reward_points
                } else {
                    0
                }
            }
            BonusScoring::PerBird(points) => count * points,
        }
    }

    fn count_matching_birds(&self, player: &PlayerState) -> usize {
        match self.condition {
            HabitatCount(habitat) => match habitat {
                Forest => player.forest().len(),
                Grassland => player.grassland().len(),
                Wetland => player.wetland().len(),
            },
            Miscellaneous(Ecologist) => {
                let forest = player.forest().len();
                let grassland = player.grassland().len();
                let wetland = player.wetland().len();
                let min_count = [forest, grassland, wetland]
                    .iter()
                    .copied()
                    .min()
                    .unwrap_or(0);

                let mut count = 0;
                if forest == min_count {
                    count += forest;
                }
                if grassland == min_count {
                    count += grassland;
                }
                if wetland == min_count {
                    count += wetland;
                }
                count
            }
            _ => player
                .all_played_birds()
                .filter(|played_bird| self.matches_played_bird(played_bird))
                .count(),
        }
    }
}
