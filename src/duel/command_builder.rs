use crate::CardVariant;
use std::collections::HashSet;

use super::{
    command::*,
    field::{CardMode, FaceDirection, GuardianStarChoice},
    state::DuelStateEnum,
    Duel,
};

pub struct Start;
pub struct Hand;
pub struct HandSingleSelected {
    hand_index: usize,
}
pub struct HandSingleSelectedWithFaceDirection {
    hand_index: usize,
    face_direction: FaceDirection,
}
pub struct HandMultipleSelected {
    hand_indices: Vec<usize>,
}
pub struct CommandBuilder<'a, State> {
    state: State,
    duel: &'a Duel,
}

impl<'a> CommandBuilder<'a, Start> {
    pub fn new(duel: &'a Duel) -> Self {
        CommandBuilder { state: Start, duel }
    }

    pub fn hand(self) -> Result<CommandBuilder<'a, Hand>, CommandError> {
        if let DuelStateEnum::HandState { .. } = self.duel.state {
            Ok(CommandBuilder {
                state: Hand,
                duel: self.duel,
            })
        } else {
            Err(CommandError::InvalidDuelState)
        }
    }

    pub fn field(self) -> Result<CommandBuilder<'a, Field>, CommandError> {
        if let DuelStateEnum::FieldState { .. } = self.duel.state {
            Ok(CommandBuilder {
                state: Field,
                duel: self.duel,
            })
        } else {
            Err(CommandError::InvalidDuelState)
        }
    }

    pub fn apply_equip(self, monster_row_index: usize) -> Result<DuelCommandEnum, CommandError> {
        if let DuelStateEnum::FieldEquipSelectedState(_) = &self.duel.state {
            if self
                .duel
                .get_player()
                .monster_row
                .get(monster_row_index)
                .is_some()
            {
                Ok(FieldPlayEquipPickMonsterCmd { monster_row_index }.into())
            } else {
                Err(CommandError::CannotEquipEmptyPosition)
            }
        } else {
            Err(CommandError::InvalidDuelState)
        }
    }

    pub fn cancel_equip(self) -> Result<DuelCommandEnum, CommandError> {
        if let DuelStateEnum::FieldEquipSelectedState(_) = &self.duel.state {
            Ok(FieldCancelPlayEquipCmd.into())
        } else {
            Err(CommandError::InvalidDuelState)
        }
    }

    pub fn set_guardian_star(
        self,
        guardian_star_choice: GuardianStarChoice,
    ) -> Result<DuelCommandEnum, CommandError> {
        if let DuelStateEnum::SetGuardianStarState(_) = self.duel.state {
            Ok(SetGuardianStarCmd {
                guardian_star_choice,
            }
            .into())
        } else {
            Err(CommandError::InvalidDuelState)
        }
    }
    // fn guardian_star
}

impl<'a> CommandBuilder<'a, Hand> {
    pub fn select(
        self,
        hand_index: usize,
    ) -> Result<CommandBuilder<'a, HandSingleSelected>, CommandError> {
        if hand_index < self.duel.get_player().hand.len() {
            Ok(CommandBuilder {
                state: HandSingleSelected { hand_index },
                duel: self.duel,
            })
        } else {
            Err(CommandError::OutOfBoundsHandSelection)
        }
    }

    pub fn select_multiple(
        self,
        hand_indices: Vec<usize>,
    ) -> Result<CommandBuilder<'a, HandMultipleSelected>, CommandError> {
        let hand_len = self.duel.get_player().hand.len();
        let unique_indices: HashSet<_> = hand_indices.iter().collect();
        if unique_indices.len() != hand_indices.len() {
            return Err(CommandError::DuplicateHandSelection);
        }
        if hand_indices.len() < 2 || hand_indices.len() > 5 {
            return Err(CommandError::InvalidNumberOfCardsSelected);
        }
        for &index in &hand_indices {
            if index >= hand_len {
                return Err(CommandError::OutOfBoundsHandSelection);
            }
        }
        Ok(CommandBuilder {
            state: HandMultipleSelected { hand_indices },
            duel: self.duel,
        })
    }
}

impl<'a> CommandBuilder<'a, HandSingleSelected> {
    pub fn facing(
        self,
        face_direction: FaceDirection,
    ) -> CommandBuilder<'a, HandSingleSelectedWithFaceDirection> {
        CommandBuilder {
            state: HandSingleSelectedWithFaceDirection {
                hand_index: self.state.hand_index,
                face_direction,
            },
            duel: self.duel,
        }
    }
}

impl<'a> CommandBuilder<'a, HandSingleSelectedWithFaceDirection> {
    pub fn play(self) -> Result<DuelCommandEnum, CommandError> {
        // play() handles playing a faceup magic or ritual
        // if its not a faceup magic or ritual, return a OnlyFaceUpMagicOrRitualCanBePlayedFromHand
        let card = &self.duel.get_player().hand[self.state.hand_index];
        if !matches!(
            card.variant,
            CardVariant::Magic { .. } | CardVariant::Ritual { .. }
        ) || self.state.face_direction == FaceDirection::Down
        {
            return Err(CommandError::FieldIndexNotSet);
        }

        Ok(HandPlaySingleCmd {
            hand_index: self.state.hand_index,
            face_direction: self.state.face_direction,
            field_index: None,
        }
        .into())
    }

    pub fn place(self, field_index: usize) -> Result<DuelCommandEnum, CommandError> {
        // Return an error if the card is a magic or ritual and the face_direction is FaceDirection::Up.
        let card = &self.duel.get_player().hand[self.state.hand_index];
        if matches!(
            card.variant,
            CardVariant::Magic { .. } | CardVariant::Ritual { .. }
        ) && self.state.face_direction == FaceDirection::Up
        {
            return Err(CommandError::CannotPlaceFaceUpMagicOrRitualAtFieldIndex);
        }

        // if the card is a monster or faceup equip, we need to check that the field_index is within the monster row.
        // otherwise, we need to check that the field_index is within the spell row.
        if matches!(card.variant, CardVariant::Monster { .. })
            || (matches!(card.variant, CardVariant::Equip { .. })
                && self.state.face_direction == FaceDirection::Up)
        {
            if field_index >= self.duel.get_player().monster_row.len() {
                return Err(CommandError::OutOfBoundsFieldSelection);
            }
            // Check if monster is present at field index for Equip card
            if matches!(card.variant, CardVariant::Equip { .. })
                && self.duel.get_player().monster_row[field_index].is_none()
            {
                return Err(CommandError::CannotEquipEmptyPosition);
            }
        } else {
            if field_index >= self.duel.get_player().spell_row.len() {
                return Err(CommandError::OutOfBoundsFieldSelection);
            }
        }

        Ok(HandPlaySingleCmd {
            hand_index: self.state.hand_index,
            face_direction: self.state.face_direction,
            field_index: Some(field_index),
        }
        .into())
    }
}

impl<'a> CommandBuilder<'a, HandMultipleSelected> {
    pub fn place(self, field_index: usize) -> Result<DuelCommandEnum, CommandError> {
        let mut cards = Vec::new();
        // if field_index already has a monster, we need to prepend it to cards.
        if let Some(monster) = self.duel.get_player().monster_row[field_index].as_ref() {
            cards.push(monster.card.clone());
        }
        for index in &self.state.hand_indices {
            cards.push(self.duel.get_player().hand[*index].clone());
        }
        let card = crate::combine_cards(cards).last().unwrap().2.clone();

        // if the card is a monster, we need to check that the field_index is within the length of the monster row.
        if let CardVariant::Monster { .. } = card.variant {
            if field_index >= self.duel.get_player().monster_row.len() {
                return Err(CommandError::OutOfBoundsFieldSelection);
            }
        }

        Ok(HandPlayMultipleCmd {
            hand_indices: self.state.hand_indices,
            field_index,
        }
        .into())
    }
}

pub struct Field;
pub struct FieldMonsterSelected {
    monster_index: usize,
}
impl<'a> CommandBuilder<'a, Field> {
    pub fn select_monster(
        self,
        monster_index: usize,
    ) -> Result<CommandBuilder<'a, FieldMonsterSelected>, CommandError> {
        if monster_index >= self.duel.get_player().monster_row.len() {
            Err(CommandError::OutOfBoundsFieldSelection)
        } else {
            let monster = &self.duel.get_player().monster_row[monster_index];
            if let Some(monster) = monster {
                if monster.disabled {
                    return Err(CommandError::CannotSelectDisabledMonster);
                }
            } else {
                return Err(CommandError::MonsterNotPresentAtSelectedPosition);
            }
            Ok(CommandBuilder {
                state: FieldMonsterSelected { monster_index },
                duel: self.duel,
            })
        }
    }

    pub fn play_spell(self, spell_index: usize) -> Result<DuelCommandEnum, CommandError> {
        let player = self.duel.get_player();
        let spell_card_pos = player
            .spell_row
            .get(spell_index)
            .ok_or(CommandError::OutOfBoundsFieldSelection)?;

        match spell_card_pos {
            Some(_) => Ok(FieldPlaySpellCmd {
                spell_row_index: spell_index,
            }
            .into()),
            None => Err(CommandError::SpellNotPresentAtSelectedPosition),
        }
    }

    pub fn end_turn(self) -> DuelCommandEnum {
        EndTurnCmd.into()
    }
}

impl<'a> CommandBuilder<'a, FieldMonsterSelected> {
    pub fn attack(self, enemy_monster_index: usize) -> Result<DuelCommandEnum, CommandError> {
        let player = self.duel.get_player();
        let monster = &player.monster_row[self.state.monster_index];
        if let Some(monster) = monster {
            if monster.card_mode == CardMode::Defense {
                return Err(CommandError::CannotAttackWithMonsterInDefense);
            }
        }
        let enemy = self.duel.get_enemy();
        if enemy_monster_index >= enemy.monster_row.len() {
            return Err(CommandError::OutOfBoundsFieldSelection);
        }
        if enemy.monster_row[enemy_monster_index].is_none() {
            if enemy.monster_row.iter().any(|monster| monster.is_some()) {
                return Err(CommandError::CannotAttackEmptyPositionWhileMonstersPresent);
            }
        }
        Ok(FieldAttackCmd {
            monster_row_index: self.state.monster_index,
            enemy_monster_row_index: enemy_monster_index,
        }
        .into())
    }

    pub fn change_mode(self) -> DuelCommandEnum {
        FieldChangeModeCmd {
            monster_index: self.state.monster_index,
        }
        .into()
    }
}

#[cfg(test)]
// test for playing a single card from hand to field
mod test {
    use super::*;

    // #[test]
    // fn test_play_single_card() -> Result<(), CommandError> {
    //     let mut duel = Rc::new(Duel::default());
    //     let builder = CommandBuilder::new(Rc::clone(&duel));
    //     let command = builder.hand()?.select(0)?.facing(FaceDirection::Up).place(0)?;
    //     dbg!(&command);

    //     let card = duel.get_player().hand.get(0).unwrap().clone();
    //     Rc::get_mut(&mut duel).unwrap().execute_command(command);

    //     // if the card is a monster, we should now be in a DualStateEnum::SetGuardianStarState
    //     // if the card is a spell, we should now be in a DualStateEnum::FieldState
    //     match card.variant {
    //         CardVariant::Monster { .. } => {
    //             dbg!(&duel.state);
    //             assert!(matches!(duel.state, DuelStateEnum::SetGuardianStarState(_)));
    //         },
    //         _ => {
    //             dbg!(&duel.state);
    //             assert!(matches!(duel.state, DuelStateEnum::FieldState(_)));
    //         },
    //     }

    //     Ok(())
    // }

    // create a test that causes an error
    #[test]
    fn test_play_single_card_error() {
        let duel = Duel::default();
        let builder = CommandBuilder::new(&duel);
        let command = builder.field(); // this will cause InvalidDuelState error
                                       // let err = command.unwrap_err();
        if let Err(e) = command {
            dbg!(&e);
            println!("{}", e);
            assert!(true);
        } else {
            assert!(false);
        }
    }
}
