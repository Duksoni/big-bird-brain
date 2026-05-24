use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
pub enum Round {
    One,
    Two,
    Three,
    Four,
}

impl Round {
    const TURNS: [u8; 4] = [8, 7, 6, 5];

    #[inline]
    pub fn turns(self) -> u8 {
        Self::TURNS[self as usize]
    }

    #[inline]
    pub fn next(self) -> Option<Self> {
        match self {
            Self::One => Some(Self::Two),
            Self::Two => Some(Self::Three),
            Self::Three => Some(Self::Four),
            Self::Four => None,
        }
    }
}
