use serde::{Deserialize, Serialize};

use super::{
    Habitat::{self, *},
    NestType, Player, PlayerState, Round,
};
use GoalType::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum GoalSide {
    Majority,      // Green Side: Rank-based rewards
    PointsPerItem, // Blue Side: 1 point per item, max 5
}

impl GoalSide {
    const MAJORITY_POINTS: [[u8; 4]; 4] = [
        [4, 1, 0, 0], // Round One
        [5, 2, 1, 0], // Round Two
        [6, 3, 2, 0], // Round Three
        [7, 4, 3, 0], // Round Four
    ];

    pub fn calculate_points(self, round: Round, value: u8) -> u8 {
        match self {
            GoalSide::Majority => {
                let rank = value as usize;
                if rank < 4 {
                    Self::MAJORITY_POINTS[round as usize][rank]
                } else {
                    0
                }
            }
            GoalSide::PointsPerItem => {
                // 1 point per item, maxed out at 5 points
                std::cmp::min(value, 5)
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum GoalType {
    TotalBirds,
    BirdsInHabitat(Habitat),
    EggsInHabitat(Habitat),
    EggsInNest(NestType),
    BirdsWithEggsInNest(NestType),
    SetsOfEggs,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EndOfRoundGoal {
    id: i64,
    goal_type: GoalType,
}

impl EndOfRoundGoal {
    pub fn evaluate(&self, player: &PlayerState) -> u8 {
        match self.goal_type {
            TotalBirds => player.all_played_birds().count() as u8,
            BirdsInHabitat(habitat) => match habitat {
                Forest => player.forest().len() as u8,
                Grassland => player.grassland().len() as u8,
                Wetland => player.wetland().len() as u8,
            },
            EggsInHabitat(habitat) => {
                let habitat_row = match habitat {
                    Forest => player.forest(),
                    Grassland => player.grassland(),
                    Wetland => player.wetland(),
                };
                habitat_row.iter().map(|pb| pb.laid_eggs()).sum()
            }
            EggsInNest(nest_type) => player
                .all_played_birds()
                .filter(|pb| pb.bird().nest_type() == nest_type)
                .map(|pb| pb.laid_eggs())
                .sum(),
            BirdsWithEggsInNest(nest_type) => player
                .all_played_birds()
                .filter(|pb| pb.bird().nest_type() == nest_type && pb.laid_eggs() > 0)
                .count() as u8,
            SetsOfEggs => player
                .forest()
                .iter()
                .zip(player.grassland().iter())
                .zip(player.wetland().iter())
                .filter(|((f, g), w)| f.laid_eggs() > 0 && g.laid_eggs() > 0 && w.laid_eggs() > 0)
                .count() as u8,
        }
    }
}
