use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

use crate::Card;

#[derive(Serialize, Deserialize, Debug, PartialEq, FromPrimitive, ToPrimitive, Copy, Clone)]
pub enum FaceDirection {
    Up,
    Down,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, FromPrimitive, ToPrimitive, Copy, Clone)]
pub enum CardMode {
    Attack,
    Defense,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, FromPrimitive, ToPrimitive, Copy, Clone)]
pub enum GuardianStarChoice {
    A,
    B,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonsterRowPosition {
    pub card: Card,
    pub face_direction: FaceDirection,
    pub card_mode: CardMode,
    pub guardian_star_choice: GuardianStarChoice,
    pub disabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpellRowPosition {
    pub card: Card,
    pub face_direction: FaceDirection,
}
