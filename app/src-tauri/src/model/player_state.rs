use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    Bird, Bonus,
    Habitat::{self, *},
    PlayedBird, Resource, Round,
};

pub trait Player {
    fn id(&self) -> &str;
    fn food(&self) -> &[Resource];
    fn forest(&self) -> &[PlayedBird];
    fn grassland(&self) -> &[PlayedBird];
    fn wetland(&self) -> &[PlayedBird];
    fn end_of_round_points(&self) -> &[u8; 4];
    fn end_of_round_points_mut(&mut self) -> &mut [u8; 4];

    fn all_played_birds(&self) -> impl Iterator<Item = &PlayedBird> {
        self.forest()
            .iter()
            .chain(self.grassland().iter())
            .chain(self.wetland().iter())
    }

    fn total_eggs(&self) -> u8 {
        self.all_played_birds()
            .map(|played_bird| played_bird.laid_eggs())
            .sum()
    }

    fn has_habitat_space(&self, habitat: Habitat) -> bool {
        match habitat {
            Forest => self.forest().len() < 5,
            Grassland => self.grassland().len() < 5,
            Wetland => self.wetland().len() < 5,
        }
    }

    fn egg_cost_for_habitat(&self, habitat: Habitat) -> u8 {
        let count = match habitat {
            Forest => self.forest().len(),
            Grassland => self.grassland().len(),
            Wetland => self.wetland().len(),
        };
        match count {
            0 => 0,
            1 | 2 => 1,
            3 | 4 => 2,
            _ => 0,
        }
    }

    fn update_round_points(&mut self, round: Round, round_points: u8) {
        self.end_of_round_points_mut()[round as usize] = round_points;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerState {
    id: String,
    food: Vec<Resource>,
    hand: Vec<Bird>,
    forest: Vec<PlayedBird>,
    grassland: Vec<PlayedBird>,
    wetland: Vec<PlayedBird>,
    bonus_goals: Vec<Bonus>,
    end_of_round_points: [u8; 4],
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpponentPlayerState {
    id: String,
    food: Vec<Resource>,
    forest: Vec<PlayedBird>,
    grassland: Vec<PlayedBird>,
    wetland: Vec<PlayedBird>,
    end_of_round_points: [u8; 4],
}

impl PlayerState {
    pub fn init(&mut self, id: String, food: Vec<Resource>, hand: Vec<Bird>, bonus_goal: Bonus) {
        self.id = id;
        self.food = food;
        self.hand = hand;
        self.bonus_goals.push(bonus_goal);
    }

    pub fn hand(&self) -> &[Bird] {
        &self.hand
    }

    pub fn count_points(&self) -> u16 {
        let round_points: u16 = self
            .end_of_round_points
            .iter()
            .map(|&point| point as u16)
            .sum();

        let bird_points = self
            .all_played_birds()
            .map(PlayedBird::sum_points)
            .sum::<u16>();

        let bonus_points: u16 = self
            .bonus_goals
            .iter()
            .map(|bonus| bonus.evaluate(self) as u16)
            .sum();

        round_points + bird_points + bonus_points
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            food: Default::default(),
            hand: Default::default(),
            forest: Vec::with_capacity(5),
            grassland: Vec::with_capacity(5),
            wetland: Vec::with_capacity(5),
            bonus_goals: Default::default(),
            end_of_round_points: [0; 4],
        }
    }
}

impl Default for OpponentPlayerState {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            food: Default::default(),
            forest: Vec::with_capacity(5),
            grassland: Vec::with_capacity(5),
            wetland: Vec::with_capacity(5),
            end_of_round_points: [0; 4],
        }
    }
}

impl Player for PlayerState {
    fn id(&self) -> &str {
        &self.id
    }

    fn food(&self) -> &[Resource] {
        &self.food
    }

    fn forest(&self) -> &[PlayedBird] {
        &self.forest
    }

    fn grassland(&self) -> &[PlayedBird] {
        &self.grassland
    }

    fn wetland(&self) -> &[PlayedBird] {
        &self.wetland
    }

    fn end_of_round_points(&self) -> &[u8; 4] {
        &self.end_of_round_points
    }

    fn end_of_round_points_mut(&mut self) -> &mut [u8; 4] {
        &mut self.end_of_round_points
    }
}

impl Player for OpponentPlayerState {
    fn id(&self) -> &str {
        &self.id
    }

    fn food(&self) -> &[Resource] {
        &self.food
    }

    fn forest(&self) -> &[PlayedBird] {
        &self.forest
    }

    fn grassland(&self) -> &[PlayedBird] {
        &self.grassland
    }

    fn wetland(&self) -> &[PlayedBird] {
        &self.wetland
    }

    fn end_of_round_points(&self) -> &[u8; 4] {
        &self.end_of_round_points
    }

    fn end_of_round_points_mut(&mut self) -> &mut [u8; 4] {
        &mut self.end_of_round_points
    }
}
