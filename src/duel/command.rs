use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{combine, CardVariant};

use super::{Duel, field::{MonsterRowPosition, FaceDirection, CardMode, GuardianStarChoice}, state::*};

#[derive(Error, Debug)]
pub enum DuelCommandError {
    #[error("Invalid Duel State")]
    InvalidState,
    #[error("Invalid Hand Index")]
    InvalidHandIndex,
    #[error("Invalid Field Index")]
    InvalidFieldIndex,
    #[error("Hand Card Type Mismatch")]
    HandCardTypeMismatch,
}

#[enum_dispatch]
pub trait DuelCommand {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), DuelCommandError>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HandPlaySingleMonsterCmd {
    pub hand_index: usize,
    pub field_index: usize,
    pub face_direction: FaceDirection,
}
impl DuelCommand for HandPlaySingleMonsterCmd {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), DuelCommandError> {
        // check the duel state. if it is not Hand, return an error.
        matches!(duel.state, DuelStateEnum::HandState(_)).then(|| ()).ok_or(DuelCommandError::InvalidState)?;
        // check the hand index is not out of bounds. it must be less than the hand length.
        if self.hand_index >= duel.get_player().hand.len() {
            return Err(DuelCommandError::InvalidHandIndex);
        }
        // check that the card at hand_index is a monster by matching on card.variant
        if !matches!(duel.get_player().hand[self.hand_index].variant, CardVariant::Monster{ .. }) {
            return Err(DuelCommandError::HandCardTypeMismatch);
        }
        // check the field index is not out of bounds. it must be less than the monster row length.
        if self.field_index >= duel.get_player().monster_row.len() {
            return Err(DuelCommandError::InvalidFieldIndex);
        }

        // pop the card at hand_index from the hand.
        let card = duel.get_player().hand.remove(self.hand_index);
        // check the monster row at field_index. if there is already a monster there, call combine on the two monsters.
        // put the result in the monster row.
        let existing_position = duel.get_player().monster_row[self.field_index].take();
        let card = if let Some(monster) = existing_position {
            self.face_direction = FaceDirection::Up;
            combine(&card, &monster.card)
        } else {
            card
        };

        duel.get_player().monster_row[self.field_index] = Some(MonsterRowPosition {
            card,
            face_direction: self.face_direction,
            card_mode: CardMode::Attack,
            guardian_star_choice: GuardianStarChoice::A,
        });

        duel.state = SetGuardianStarState {
            field_index: self.field_index,
        }.into();

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HandPlaySingleMagicUpCmd {
    pub hand_index: usize,
}
impl DuelCommand for HandPlaySingleMagicUpCmd {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), DuelCommandError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HandPlaySingleMagicDownCmd {
    pub hand_index: usize,
    pub field_index: usize,
}
impl DuelCommand for HandPlaySingleMagicDownCmd {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), DuelCommandError> {
        // matches!(duel.state, DuelStateEnum::HandState(_)).then(|| ()).ok_or(DuelCommandError::InvalidState)?;

        todo!();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HandPlaySingleTrapCmd {
    pub hand_index: usize,
}
impl DuelCommand for HandPlaySingleTrapCmd {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), DuelCommandError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HandPlaySingleEquipCmd {
    pub hand_index: usize,
    pub monster_index: usize,
    pub face_direction: FaceDirection,
}
impl DuelCommand for HandPlaySingleEquipCmd {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), DuelCommandError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HandPlayMultipleCmd {
    pub hand_indices: Vec<usize>,
    pub field_index: usize,
}
impl DuelCommand for HandPlayMultipleCmd {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), DuelCommandError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetGuardianStarCmd {
    pub star: GuardianStarChoice,
}
impl DuelCommand for SetGuardianStarCmd {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), DuelCommandError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldAttackCmd {
    pub player_monster_index: usize,
    pub opponent_monster_index: usize,
}
impl DuelCommand for FieldAttackCmd {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), DuelCommandError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldChangeModeCmd {
    pub monster_index: usize,
}
impl DuelCommand for FieldChangeModeCmd {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), DuelCommandError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldPlayEquipCmd {
    pub equip_index: usize,
    pub monster_index: usize,
}
impl DuelCommand for FieldPlayEquipCmd {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), DuelCommandError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldPlaySpellCmd {
    pub spell_index: usize,
}
impl DuelCommand for FieldPlaySpellCmd {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), DuelCommandError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndTurnCmd;
impl DuelCommand for EndTurnCmd {
    fn execute(&mut self, duel: &mut Duel) -> Result<(), DuelCommandError> {
        duel.turn += 1;

        let player = duel.get_player();
        player.draw();

        if player.hand.len() < 5 {
            duel.state = EndState.into();
        }
        else {
            duel.state = HandState.into();
        }

        Ok(())
    }
}

#[enum_dispatch(DuelCommand)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DuelCommandEnum {
    HandPlaySingleMonsterCmd,
    HandPlaySingleMagicUpCmd,
    HandPlaySingleMagicDownCmd,
    HandPlaySingleTrapCmd,
    HandPlaySingleEquipCmd,
    HandPlayMultipleCmd,
    SetGuardianStarCmd,
    FieldAttackCmd,
    FieldChangeModeCmd,
    FieldPlayEquipCmd,
    FieldPlaySpellCmd,
    EndTurnCmd,
}
