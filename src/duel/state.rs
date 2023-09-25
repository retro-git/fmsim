use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use super::command::DuelCommandEnum;

#[enum_dispatch]
pub trait DuelState {
    fn get_all_valid_commands(&self) -> Vec<DuelCommandEnum>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HandState;
impl DuelState for HandState {
    fn get_all_valid_commands(&self) -> Vec<DuelCommandEnum> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldState;
impl DuelState for FieldState {
    fn get_all_valid_commands(&self) -> Vec<DuelCommandEnum> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetGuardianStarState {
    pub monster_row_index: usize,
}
impl DuelState for SetGuardianStarState {
    fn get_all_valid_commands(&self) -> Vec<DuelCommandEnum> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndState;
impl DuelState for EndState {
    fn get_all_valid_commands(&self) -> Vec<DuelCommandEnum> {
        todo!()
    }
}

#[enum_dispatch(DuelState)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DuelStateEnum {
    HandState, // The user is selecting a card, or multiple cards, from the hand.
    // HandPlaySingle { card: Card }, // The user selected a single card to play from the hand. Now awaiting further info depending on the card type (e.g. face up/down, field position).
    FieldState, // The hand phase is done and the user can perform field actions (such as attacking, toggling between attack/defense, etc.)
    // FieldPlayEquip { position: usize }, // The user selected an equip card on the spell row. Now awaiting them to pick the monster to equip to.
    SetGuardianStarState, // Happens when a monster is played from the hand, or when an equip is played on an existing monster but the equip fails.
    EndState,             // The game is over.
}
