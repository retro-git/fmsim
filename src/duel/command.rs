use derive_builder::Builder;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;

use crate::CardVariant;

use super::{
    field::{CardMode, FaceDirection, GuardianStarChoice},
    state::*,
    Duel,
};

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Invalid Duel State.")]
    InvalidDuelState,
    #[error("When selecting multiple cards, must pick 2-5 cards.")]
    InvalidNumberOfCardsSelected,
    #[error("Face-up magic or ritual cards cannot be placed on the field.")]
    CannotPlaceFaceUpMagicOrRitualAtFieldIndex,
    #[error("Field index not set.")]
    FieldIndexNotSet,
    #[error("Out-of-bounds Hand Selection.")]
    OutOfBoundsHandSelection,
    #[error("Out-of-bounds Field Selection.")]
    OutOfBoundsFieldSelection,
    #[error("Out-of-bounds Hand Selection.")]
    DuplicateHandSelection,
    #[error("Monster not present at the selected position.")]
    MonsterNotPresentAtSelectedPosition,
    #[error("Spell not present at the selected position.")]
    SpellNotPresentAtSelectedPosition,
    #[error("Cannot attack empty position while enemy monsters are present.")]
    CannotAttackEmptyPositionWhileMonstersPresent,
    #[error("The selected monster is disabled.")]
    CannotSelectDisabledMonster,
    #[error("The selected monster cannot attack because it is in defense mode.")]
    CannotAttackWithMonsterInDefense,
    #[error("Tried to apply an equip to an empty position. It must be a monster.")]
    CannotEquipEmptyPosition,
}

#[enum_dispatch]
pub trait DuelCommand {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), CommandError>;
}

#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
pub struct HandPlaySingleCmd {
    pub hand_index: usize,
    pub face_direction: FaceDirection,
    pub field_index: Option<usize>,
}
impl DuelCommand for HandPlaySingleCmd {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), CommandError> {
        // Check if the duel is in the correct state
        if !matches!(duel.state, DuelStateEnum::HandState { .. }) {
            return Err(CommandError::InvalidDuelState);
        }

        // Check if the hand index is valid
        let card = duel
            .get_player()
            .hand
            .get(self.hand_index)
            .ok_or(CommandError::OutOfBoundsHandSelection)?;

        if matches!(
            card.variant,
            CardVariant::Magic { .. } | CardVariant::Ritual { .. }
        ) && self.face_direction == FaceDirection::Up
            && self.field_index.is_some()
        {
            return Err(CommandError::CannotPlaceFaceUpMagicOrRitualAtFieldIndex);
        } else if self.field_index.is_none() {
            return Err(CommandError::FieldIndexNotSet);
        } else if let Some(field_index) = self.field_index {
            if field_index >= duel.get_player().monster_row.len() {
                return Err(CommandError::OutOfBoundsFieldSelection);
            }
        }

        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HandPlayMultipleCmd {
    pub hand_indices: Vec<usize>,
    pub field_index: usize,
}
impl DuelCommand for HandPlayMultipleCmd {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), CommandError> {
        // Check if the duel is in the correct state
        if !matches!(duel.state, DuelStateEnum::HandState { .. }) {
            return Err(CommandError::InvalidDuelState);
        }

        // Check that there are a valid number of cards selected
        if self.hand_indices.len() < 2 || self.hand_indices.len() > 5 {
            return Err(CommandError::InvalidNumberOfCardsSelected);
        }

        // Check that there are no duplicate cards selected
        let hand_indices_set: HashSet<_> = self.hand_indices.iter().collect();
        if hand_indices_set.len() != self.hand_indices.len() {
            return Err(CommandError::DuplicateHandSelection);
        }

        // Check if the field index is valid
        if self.field_index >= duel.get_player().monster_row.len() {
            return Err(CommandError::OutOfBoundsFieldSelection);
        }

        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetGuardianStarCmd {
    pub guardian_star_choice: GuardianStarChoice,
}
impl DuelCommand for SetGuardianStarCmd {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), CommandError> {
        // Check if the duel is in the correct state (SetGuardianStarState)
        if !matches!(duel.state, DuelStateEnum::SetGuardianStarState(_)) {
            return Err(CommandError::InvalidDuelState);
        }

        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldAttackCmd {
    pub monster_row_index: usize,
    pub enemy_monster_row_index: usize,
}
impl DuelCommand for FieldAttackCmd {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), CommandError> {
        // Check if the duel is in the correct state (FieldState)
        if !matches!(duel.state, DuelStateEnum::FieldState(_)) {
            return Err(CommandError::InvalidDuelState);
        }

        // Check that monster_row_index contains a monster, and that it is not disabled
        let monster = duel
            .get_player()
            .monster_row
            .get(self.monster_row_index)
            .ok_or(CommandError::OutOfBoundsFieldSelection)?;
        if let Some(monster) = monster {
            if monster.disabled {
                return Err(CommandError::CannotSelectDisabledMonster);
            }
            if monster.card_mode == CardMode::Defense {
                return Err(CommandError::CannotAttackWithMonsterInDefense);
            }
        } else {
            return Err(CommandError::MonsterNotPresentAtSelectedPosition);
        }

        let enemy = duel.get_enemy();
        if self.enemy_monster_row_index >= enemy.monster_row.len() {
            return Err(CommandError::OutOfBoundsFieldSelection);
        }
        if enemy.monster_row[self.enemy_monster_row_index].is_none() {
            if enemy.monster_row.iter().any(|monster| monster.is_some()) {
                return Err(CommandError::CannotAttackEmptyPositionWhileMonstersPresent);
            }
        }

        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldChangeModeCmd {
    pub monster_index: usize,
}
impl DuelCommand for FieldChangeModeCmd {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), CommandError> {
        // Check if the duel is in the correct state (FieldState)
        if !matches!(duel.state, DuelStateEnum::FieldState(_)) {
            return Err(CommandError::InvalidDuelState);
        }

        // Check that monster_index contains a monster, and that it is not disabled
        let monster = duel
            .get_player()
            .monster_row
            .get(self.monster_index)
            .ok_or(CommandError::OutOfBoundsFieldSelection)?;
        if let Some(monster) = monster {
            if monster.disabled {
                return Err(CommandError::CannotSelectDisabledMonster);
            }
        } else {
            return Err(CommandError::MonsterNotPresentAtSelectedPosition);
        }

        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct FieldPlaySpellCmd {
    pub spell_row_index: usize,
}
impl DuelCommand for FieldPlaySpellCmd {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), CommandError> {
        // Check if the duel is in the correct state (FieldState)
        if !matches!(duel.state, DuelStateEnum::FieldState(_)) {
            return Err(CommandError::InvalidDuelState);
        }

        // Check that spell_row_index contains a spell
        let spell = duel
            .get_player()
            .spell_row
            .get(self.spell_row_index)
            .ok_or(CommandError::OutOfBoundsFieldSelection)?;
        if spell.is_none() {
            return Err(CommandError::SpellNotPresentAtSelectedPosition);
        }

        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldCancelPlayEquipCmd;
impl DuelCommand for FieldCancelPlayEquipCmd {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), CommandError> {
        // Check for FieldEquipSelectedState
        if !matches!(duel.state, DuelStateEnum::FieldEquipSelectedState(_)) {
            return Err(CommandError::InvalidDuelState);
        }

        // return the duel unmodified, expect the state is set to FieldState.
        duel.state = FieldState.into();
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldPlayEquipPickMonsterCmd {
    pub monster_row_index: usize,
}
impl DuelCommand for FieldPlayEquipPickMonsterCmd {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), CommandError> {
        // Check for FieldEquipSelectedState
        if !matches!(duel.state, DuelStateEnum::FieldEquipSelectedState(_)) {
            return Err(CommandError::InvalidDuelState);
        }

        // Check that monster_row_index contains a monster
        let monster = duel
            .get_player()
            .monster_row
            .get(self.monster_row_index)
            .ok_or(CommandError::OutOfBoundsFieldSelection)?;
        if monster.is_none() {
            return Err(CommandError::CannotEquipEmptyPosition);
        }

        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndTurnCmd;
impl DuelCommand for EndTurnCmd {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), CommandError> {
        // Check for FieldState
        if !matches!(duel.state, DuelStateEnum::FieldState(_)) {
            return Err(CommandError::InvalidDuelState);
        }

        duel.turn += 1;

        let player = duel.get_player_mut();
        player.draw();

        if player.hand.len() < 5 {
            duel.state = EndState.into();
        } else {
            duel.state = HandState.into();
        }

        Ok(())
    }
}

#[enum_dispatch(DuelCommand)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DuelCommandEnum {
    HandPlaySingleCmd,
    HandPlayMultipleCmd,
    SetGuardianStarCmd,
    FieldAttackCmd,
    FieldChangeModeCmd,
    FieldPlaySpellCmd,
    FieldPlayEquipPickMonsterCmd,
    FieldCancelPlayEquipCmd,
    EndTurnCmd,
}

// test creating a HandPlaySingleMonsterCmd with builder
#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_create_hand_play_single_monster_cmd_with_builder() {
    //     let mut builder = HandPlaySingleMonsterCmdBuilder::default();
    //     let hand_index = 0;
    //     let field_index = 1;
    //     let face_direction = FaceDirection::Up;

    //     builder.hand_index(hand_index).field_index(field_index).face_direction(face_direction);
    //     let command = builder.build().unwrap();

    //     assert_eq!(command.hand_index, hand_index);
    //     assert_eq!(command.field_index, field_index);
    //     assert_eq!(command.face_direction, face_direction);
    // }
}
