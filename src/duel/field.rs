use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

use crate::{Card, CardVariant, GuardianStarType};

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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct MonsterRowPosition {
    pub card: Card,
    pub face_direction: FaceDirection,
    pub card_mode: CardMode,
    pub guardian_star_choice: GuardianStarChoice,
    pub disabled: bool,
}

impl MonsterRowPosition {
    pub fn get_selected_gs(&self) -> GuardianStarType {
        // card.variant, if its a monster, contains guardian_star_a and guardian_star_b.
        // panic if its not a monster.
        match self.card.variant {
            CardVariant::Monster {
                guardian_star_a,
                guardian_star_b,
                ..
            } => match self.guardian_star_choice {
                GuardianStarChoice::A => guardian_star_a,
                GuardianStarChoice::B => guardian_star_b,
            },
            _ => panic!("get_selected_gs: Card is not a monster"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SpellRowPosition {
    pub card: Card,
    pub face_direction: FaceDirection,
}
