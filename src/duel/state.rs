use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use super::{field::MonsterRowPosition, PlayerEnum};

#[enum_dispatch]
pub trait DuelState {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct HandState;
impl DuelState for HandState {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct FieldState;
impl DuelState for FieldState {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SetGuardianStarState {
    pub monster_row_position: MonsterRowPosition,
    pub monster_row_index: usize,
    pub applied_equips_amount: Option<u32>,
}
impl DuelState for SetGuardianStarState {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct EndState {
    pub winner: PlayerEnum,
}
impl DuelState for EndState {}

#[enum_dispatch(DuelState)]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum DuelStateEnum {
    HandState, // The user is selecting a card, or multiple cards, from the hand.
    // HandPlaySingle { card: Card }, // The user selected a single card to play from the hand. Now awaiting further info depending on the card type (e.g. face up/down, field position).
    FieldState, // The hand phase is done and the user can perform field actions (such as attacking, toggling between attack/defense, etc.)
    SetGuardianStarState, // Happens when a monster is played from the hand, or when an equip is played on an existing monster but the equip fails.
    EndState,             // The game is over.
}
