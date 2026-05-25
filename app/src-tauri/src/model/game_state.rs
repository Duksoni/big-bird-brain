use serde::{Deserialize, Serialize};

use super::{Bird, BirdFeederResource, Bonus, GoalSide, OpponentPlayerState, PlayerState, Round};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayerCount {
    #[default]
    Two = 2,
    Three,
    Four,
    Five,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewGameSetup {
    pub game_name: String,
    pub player_count: PlayerCount,
    pub goal_side: GoalSide,
    pub round_goal_ids: Vec<i64>,
    pub hand_bird_ids: Vec<i64>,
    pub bonus_card_ids: Vec<i64>,
    pub birdfeeder: Vec<BirdFeederResource>,
    pub bird_tray_ids: Vec<i64>,
}

pub struct DraftSetup {
    pub hand_birds: Vec<Bird>,
    pub bonus_cards: Vec<Bonus>,
    pub birdfeeder: Vec<BirdFeederResource>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    player: PlayerState,
    opponents: Vec<OpponentPlayerState>,
    round: Round,
    remaining_turns: u8,
    player_count: PlayerCount,
    bird_tray: Vec<Bird>,
    birdfeeder: Vec<BirdFeederResource>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            player: Default::default(),
            opponents: Vec::with_capacity(4),
            round: Round::One,
            remaining_turns: Round::One.turns(),
            player_count: PlayerCount::Five,
            bird_tray: Vec::with_capacity(3),
            birdfeeder: Vec::with_capacity(5),
        }
    }
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init(
        &mut self,
        player_count: PlayerCount,
        bird_tray: &[Bird; 3],
        birdfeeder: &[BirdFeederResource; 5],
    ) {
        self.player_count = player_count;
        for _ in 0..(player_count as usize - 1) {
            self.opponents.push(OpponentPlayerState::default());
        }
        self.bird_tray.extend_from_slice(bird_tray);
        self.birdfeeder.extend_from_slice(birdfeeder);
        self.birdfeeder = birdfeeder.to_vec();
    }
}

impl GameState {
    pub fn player(&self) -> &PlayerState {
        &self.player
    }

    pub fn player_mut(&mut self) -> &mut PlayerState {
        &mut self.player
    }

    pub fn round(&self) -> Round {
        self.round
    }

    pub fn remaining_turns(&self) -> u8 {
        self.remaining_turns
    }

    pub fn bird_tray(&self) -> &[Bird] {
        &self.bird_tray
    }

    pub fn birdfeeder(&self) -> &[BirdFeederResource] {
        &self.birdfeeder
    }
}
