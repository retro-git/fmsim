use std::{rc::Rc, collections::HashSet};
use thiserror::Error;
use crate::CardVariant;

use super::{field::{FaceDirection, CardMode, GuardianStarChoice}, command::*, Duel, state::DuelStateEnum};

struct Start;
struct Hand;
struct HandSingleSelected {
    hand_index: usize,
}
struct HandAwaitingField {
    hand_indices: Vec<usize>,
    face_direction: Option<FaceDirection>,
}
struct CommandBuilder<State> {
    state: State,
    duel: Rc<Duel>,
}

#[derive(Error, Debug)]
pub enum CommandBuilderError {
    #[error("Invalid Duel State.")]
    InvalidDuelState,
    #[error("Invalid Hand Selection.")]
    OutOfBoundsHandSelection,
    #[error("Invalid Field Selection.")]
    OutOfBoundsFieldSelection,
    #[error("Invalid Hand Selection.")]
    DuplicateHandSelection,
    #[error("Cannot attack empty position while enemy monsters are present.")]
    CannotAttackEmptyPositionWhileMonstersPresent,
    #[error("The selected monster is disabled.")]
    CannotSelectDisabledMonster,
    #[error("The selected monster cannot attack because it is in defense mode.")]
    CannotAttackWithMonsterInDefense,
    #[error("Tried to apply an equip to an empty position. It must be a monster.")]
    CannotEquipEmptyPosition,
}

impl CommandBuilder<Start> {
    fn new(duel: Rc<Duel>) -> Self {
        CommandBuilder {
            state: Start,
            duel,
        }
    }

    fn hand(self) -> Result<CommandBuilder<Hand>, CommandBuilderError> {
        if let DuelStateEnum::HandState{ .. } = self.duel.state {
            Ok(CommandBuilder {
                state: Hand,
                duel: self.duel,
            })
        } else {
            Err(CommandBuilderError::InvalidDuelState)
        }
    }

    fn field(self) -> Result<CommandBuilder<Field>, CommandBuilderError> {
        if let DuelStateEnum::FieldState{ .. } = self.duel.state {
            Ok(CommandBuilder {
                state: Field,
                duel: self.duel,
            })
        } else {
            Err(CommandBuilderError::InvalidDuelState)
        }
    }

    fn apply_equip(self, monster_row_index: usize) -> Result<DuelCommandEnum, CommandBuilderError> {
        if let DuelStateEnum::FieldEquipSelectedState(_) = &self.duel.state {
            if self.duel.get_player().monster_row.get(monster_row_index).is_some() {
                Ok(FieldPlayEquipCmd {
                    monster_row_index,
                }.into())
            } else {
                Err(CommandBuilderError::CannotEquipEmptyPosition)
            }
        } else {
            Err(CommandBuilderError::InvalidDuelState)
        }
    }

    fn cancel_equip(self) -> Result<DuelCommandEnum, CommandBuilderError> {
        if let DuelStateEnum::FieldEquipSelectedState(_) = &self.duel.state {
            Ok(FieldCancelSelectEquipCmd.into())
        } else {
            Err(CommandBuilderError::InvalidDuelState)
        }
    }

    fn set_guardian_star(self, guardian_star_choice: GuardianStarChoice) -> Result<DuelCommandEnum, CommandBuilderError> {
        if let DuelStateEnum::SetGuardianStarState(_) = self.duel.state {
            Ok(SetGuardianStarCmd {
                guardian_star_choice,
            }.into())
        } else {
            Err(CommandBuilderError::InvalidDuelState)
        }
    }
    // fn guardian_star
}

impl CommandBuilder<Hand> {
    fn select(self, hand_index: usize) -> Result<CommandBuilder<HandSingleSelected>, CommandBuilderError> {
        if hand_index < self.duel.get_player().hand.len() {
            Ok(CommandBuilder {
                state: HandSingleSelected {
                    hand_index,
                },
                duel: self.duel,
            })
        } else {
            Err(CommandBuilderError::OutOfBoundsHandSelection)
        }
    }

    fn select_multiple(self, hand_indices: Vec<usize>) -> Result<CommandBuilder<HandAwaitingField>, CommandBuilderError> {
        let hand_len = self.duel.get_player().hand.len();
        let unique_indices: HashSet<_> = hand_indices.iter().collect();
        if unique_indices.len() != hand_indices.len() {
            return Err(CommandBuilderError::DuplicateHandSelection);
        }
        for &index in &hand_indices {
            if index >= hand_len {
                return Err(CommandBuilderError::OutOfBoundsHandSelection);
            }
        }
        Ok(CommandBuilder {
            state: HandAwaitingField {
                hand_indices,
                face_direction: None,
            },
            duel: self.duel,
        })
    }
}

impl CommandBuilder<HandSingleSelected> {
    fn facing(self, face_direction: FaceDirection) -> CommandBuilder<HandAwaitingField> {
        CommandBuilder {
            state: HandAwaitingField {
                hand_indices: vec![self.state.hand_index],
                face_direction: Some(face_direction),
            },
            duel: self.duel,
        }
    }
}

impl CommandBuilder<HandAwaitingField> {
    fn place(self, field_index: usize) -> Result<DuelCommandEnum, CommandBuilderError> {
        let card = match self.state.hand_indices.len() {
            1 => {
                self.duel.get_player().hand[self.state.hand_indices[0]].clone()
            }
            _ => {
                let mut cards = Vec::new();
                // if field_index already has a monster, we need to clone it and prepend it to cards.
                if let Some(monster) = self.duel.get_player().monster_row[field_index].as_ref() {
                    cards.push(monster.card.clone());
                }
                for index in &self.state.hand_indices {
                    cards.push(self.duel.get_player().hand[*index].clone());
                }
                crate::combine_cards(cards)
            }
        };

        // if the card is a monster, we need to check that the field_index is within the length of the monster row.
        // otherwise, we need to check that the field_index is within the length of the spell row.
        match card.variant {
            CardVariant::Monster { .. } => {
                if field_index >= self.duel.get_player().monster_row.len() {
                    return Err(CommandBuilderError::OutOfBoundsFieldSelection);
                }
            },
            _ => {
                if field_index >= self.duel.get_player().spell_row.len() {
                    return Err(CommandBuilderError::OutOfBoundsFieldSelection);
                }
            },
        }

        match self.state.hand_indices.len() {
            1 => {
                let face_direction = self.state.face_direction.unwrap();
                let hand_index = self.state.hand_indices[0];
                match card.variant {
                    CardVariant::Monster { .. } => Ok(HandPlaySingleMonsterCmd {
                        hand_index,
                        face_direction,
                        field_index,
                    }.into()),
                    CardVariant::Magic { .. } => {
                        match face_direction {
                            FaceDirection::Up => Ok(HandPlaySingleMagicUpCmd {
                                hand_index,
                            }.into()),
                            FaceDirection::Down => Ok(HandPlaySingleMagicDownCmd {
                                hand_index,
                                field_index,
                            }.into()),
                        }
                    },
                    CardVariant::Ritual { .. } => {
                        match face_direction {
                            FaceDirection::Up => Ok(HandPlaySingleRitualUpCmd {
                                hand_index,
                            }.into()),
                            FaceDirection::Down => Ok(HandPlaySingleRitualDownCmd {
                                hand_index,
                                field_index,
                            }.into()),
                        }
                    },
                    CardVariant::Trap { .. } => Ok(HandPlaySingleTrapCmd {
                        hand_index,
                        field_index,
                        face_direction,
                    }.into()),
                    CardVariant::Equip { .. } => Ok(HandPlaySingleEquipCmd {
                        hand_index,
                        field_index,
                        face_direction,
                    }.into()),
                }
            },
            _ => Ok(HandPlayMultipleCmd {
                hand_indices: self.state.hand_indices,
                field_index,
            }.into())
        }
    }
}

struct Field;
struct FieldMonsterSelected {
    monster_index: usize,
}
struct FieldSpellSelected {
    spell_index: usize,
}
impl CommandBuilder<Field> {
    fn select_monster(self, monster_index: usize) -> Result<CommandBuilder<FieldMonsterSelected>, CommandBuilderError> {
        if monster_index >= self.duel.get_player().monster_row.len() {
            Err(CommandBuilderError::OutOfBoundsFieldSelection)
        } else {
            let monster = &self.duel.get_player().monster_row[monster_index];
            if let Some(monster) = monster {
                if monster.disabled {
                    return Err(CommandBuilderError::CannotSelectDisabledMonster);
                }
            }
            Ok(CommandBuilder {
                state: FieldMonsterSelected {
                    monster_index,
                },
                duel: self.duel,
            })
        }
    }

    fn play_spell(self, spell_index: usize) -> Result<DuelCommandEnum, CommandBuilderError> {
        let player = self.duel.get_player();
        let spell_card_pos = player.spell_row.get(spell_index).ok_or(CommandBuilderError::OutOfBoundsFieldSelection)?;

        match spell_card_pos {
            Some(spell_pos) => {
                if let CardVariant::Equip{ .. } = spell_pos.card.variant {
                    Ok(FieldSelectEquipCmd {
                        spell_row_index: spell_index,
                    }.into())
                } else {
                    Ok(FieldPlaySpellCmd {
                        spell_row_index: spell_index,
                    }.into())
                }
            },
            None => Err(CommandBuilderError::OutOfBoundsFieldSelection),
        }
    }

    fn end_turn(self) -> DuelCommandEnum {
        EndTurnCmd.into()
    }
}

impl CommandBuilder<FieldMonsterSelected> {
    fn attack(self, enemy_monster_index: usize) -> Result<DuelCommandEnum, CommandBuilderError> {
        let player = self.duel.get_player();
        let monster = &player.monster_row[self.state.monster_index];
        if let Some(monster) = monster {
            if monster.card_mode == CardMode::Defense {
                return Err(CommandBuilderError::CannotAttackWithMonsterInDefense);
            }
        }
        let enemy = self.duel.get_enemy();
        if enemy_monster_index >= enemy.monster_row.len() {
            return Err(CommandBuilderError::OutOfBoundsFieldSelection);
        }
        if enemy.monster_row[enemy_monster_index].is_none() {
            if enemy.monster_row.iter().any(|monster| monster.is_some()) {
                return Err(CommandBuilderError::CannotAttackEmptyPositionWhileMonstersPresent);
            }
        }
        Ok(FieldAttackCmd {
            monster_row_index: self.state.monster_index,
            enemy_monster_row_index: enemy_monster_index,
        }.into())
    }

    fn change_mode(self) -> DuelCommandEnum {
        FieldChangeModeCmd {
            monster_index: self.state.monster_index,
        }.into()
    }
}