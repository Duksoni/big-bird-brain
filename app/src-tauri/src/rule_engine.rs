mod draft_engine;
mod main_engine;

use crate::{
    error::{AppError, Result},
    model::{DraftSetup, DraftSuggestion, GameState, MoveSuggestion},
};
use draft_engine::DraftEngine;
use main_engine::MainEngine;

enum GameEngineState {
    Draft(DraftEngine),
    Main(MainEngine),
}

pub struct RuleEngine {
    state: GameEngineState,
}

impl RuleEngine {
    pub fn new_draft() -> Result<Self> {
        let draft_engine = DraftEngine::new()?;
        Ok(Self {
            state: GameEngineState::Draft(draft_engine),
        })
    }

    pub fn new_main() -> Result<Self> {
        let main_engine = MainEngine::new()?;
        Ok(Self {
            state: GameEngineState::Main(main_engine),
        })
    }

    pub fn evaluate_draft(&mut self, setup: DraftSetup) -> Result<Vec<DraftSuggestion>> {
        if let GameEngineState::Draft(engine) = &mut self.state {
            engine.evaluate(setup)
        } else {
            Err(AppError::RuleEngineError(String::from(
                "Not in draft phase",
            )))
        }
    }

    pub fn evaluate_action(&mut self, game_state: &GameState) -> Result<Vec<MoveSuggestion>> {
        if let GameEngineState::Main(engine) = &mut self.state {
            engine.evaluate(game_state)
        } else {
            Err(AppError::RuleEngineError(String::from("Not in main phase")))
        }
    }

    /// Transition from draft to main phase
    pub fn to_main_phase(&mut self) -> Result<()> {
        if matches!(self.state, GameEngineState::Draft(_)) {
            let main_engine = MainEngine::new()?;
            self.state = GameEngineState::Main(main_engine);
            Ok(())
        } else {
            Err(AppError::RuleEngineError(String::from(
                "Can only transition from draft phase",
            )))
        }
    }
}
