// use crate::CardVariant;

// use super::{
//     command::*,
//     field::{FaceDirection, GuardianStarChoice},
//     state::DuelStateEnum,
//     Duel,
// };

// use thiserror::Error;

// #[derive(Error, Debug)]
// pub enum DuelCommandBuilderError {
//     #[error("Invalid Duel State")]
//     NotExpectingParam,
//     #[error("Invalid Hand Index")]
//     InvalidHandIndex,
//     #[error("Invalid Field Index")]
//     InvalidFieldIndex,
//     #[error("Can't attack an empty monster position if the enemy has monsters on the field")]
//     InvalidAttackTarget,
//     #[error("Command has already been built")]
//     CommandAlreadyBuilt,
// }

// struct CommandParam<T> {
//     pub settable: bool,
//     pub value: Option<T>,
// }

// impl<T> Default for CommandParam<T> {
//     fn default() -> Self {
//         Self {
//             settable: false,
//             value: None,
//         }
//     }
// }

// // - if the duel is in the Hand state:
// //     - expect vector of indices.
// //     - if the vector has multiple indices supplied, check that each index is in the range 0 to 4 inclusive, otherwise return an error. also check that all the indices are unique, i.e. no number appears more than once, otherwise throw error. then expect a field position, again checked to be between 0 and 4, to build a HandPlayMultipleCmd.
// //     - if a single hand_index (within 0 to 4). get the card at that index in the hand, and check its enum variant type:
// //         - monster: expect a facing direction (FaceUp/FaceDown enum) and field_index (checked within 0-4 range) for the monster row to create a HandPlaySingleMonsterCmd
// //         - trap: expect a facing direction (FaceUp/FaceDown enum) and field_index (checked within 0-4 range) for the spell row to create a HandPlaySingleTrapCmd
// //         - magic: expect a FaceUp/FaceDown. if FaceUp, create a HandPlaySingleMagicUpCmd. Otherwise, expect a field_index (checked within 0-4 range) to build a HandPlaySingleMagicDownCmd.
// //         - equip: expecte a FaceUp/FaceDown and field_index (checked within 0-4 range) to build a HandPlaySingleEquipCmd. if FaceUp is selected, then the supplied field_index must be used to index the monster row and check that a monster is present there, otherwise an error will be thrown.
// //         - ritual: basically the same as magic. expect a FaceUp/FaceDown. if FaceUp, create a HandPlaySingleRitualUpCmd. Otherwise, expect a field_index (checked within 0-4 range) to build a HandPlaySingleRitualDownCmd.
// // - if the duel is in the Field state:
// //     - expect monster_row_index or spell_row_index to be set
// //     - if monster_row_index is provided:
// //         - expect an index checked within 0-4. also check that the index on the monster row has a monster present, otherwise return error. also check that the specified field position is not disabled, otherwise return an error.
// //         - then expect either enemy_monster_index between 0-4 or a CardMode enum (Attack or Defense).
// //             - if enemy_monster_index is provided: check if the enemy’s monster row is all None. if so, then create a FieldAttackCmd. if not, check that the enemy’s monster row contains a monster at the specified index. if so, then create a FieldAttackCmd. otherwise, return an error.
// //             - if a CardMode enum is provided: create a FieldChangeModeCmd.
// //             - note that im not sure yet about this handling for the FieldChangeModeCmd.
// //     - if spell_row_ubdex is provided:
// //         - expect an index checked within 0-4. also check that the index on the spell row has a spell present, otherwise return error.
// //         - check the variant type of the spell.
// //             - if it is equip: expect a monster_row_index checked within 0-4. check that the specified index on the player’s monster row contains a monster. if not, throw an error. otherwise, create a FieldPlaySpellCmd.
// //         - if it is any other type: create a FieldPlaySpellCmd
// // - if the duel is in the SetGuardianStar state:
// //     - expect a GuardianStarChoice enum (either A or B) to create a SetGuardianStarCmd.

// pub struct CommandBuilder<'a> {
//     hand_indices: CommandParam<Vec<usize>>,
//     face_direction: CommandParam<FaceDirection>, // possible terminal if faceup and hand_indices[0] is a magic or ritual
//     field_index: CommandParam<usize>,            // terminal

//     spell_row_index: CommandParam<usize>, // possible terminal if not equip
//     monster_row_index: CommandParam<usize>, // possible terminal if spell_row_index is set to an equip
//     enemy_monster_row_index: CommandParam<usize>, // terminal

//     guardian_star_choice: CommandParam<GuardianStarChoice>, // terminal

//     command: Option<DuelCommandEnum>,

//     duel: &'a Duel,
// }

// // CommandBuilder's new function should take a duel reference.
// // It will then examine the duel's state and determine with CommandParams should initially be settable.
// // It will then return a CommandBuilder with those CommandParams settable.
// // The CommandBuilder will then have a function to set each CommandParam.
// // When a CommandParam is set, it will check if it is settable.
// // If it is not settable, it will return an error.
// // If it is settable, it will set the value and return the updated CommandBuilder. It may also change which CommandParams are settable.
// // CommandBuilder will have a function to build the command.
// // It will try to match the current state of the CommandBuilder to a command that can be built.
// // If it can't, it will return an error.
// // If it can, it will build the command and return it.
// impl<'a> CommandBuilder<'a> {
//     pub fn new(duel: &'a Duel) -> Self {
//         let mut builder = Self {
//             hand_indices: Default::default(),
//             face_direction: Default::default(),
//             field_index: Default::default(),
//             monster_row_index: Default::default(),
//             enemy_monster_row_index: Default::default(),
//             spell_row_index: Default::default(),
//             guardian_star_choice: Default::default(),
//             command: None,
//             duel,
//         };

//         match duel.state {
//             DuelStateEnum::HandState(_) => {
//                 builder.hand_indices.settable = true;
//             }
//             DuelStateEnum::FieldState(_) => {
//                 builder.monster_row_index.settable = true;
//                 builder.spell_row_index.settable = true;
//             }
//             DuelStateEnum::SetGuardianStarState(_) => {
//                 builder.guardian_star_choice.settable = true;
//             }
//             DuelStateEnum::EndState(_) => {}
//         }

//         builder
//     }

//     pub fn hand_indices(
//         &mut self,
//         indices: Vec<usize>,
//     ) -> Result<&mut Self, DuelCommandBuilderError> {
//         self.check_command_set()?;

//         if !self.hand_indices.settable {
//             return Err(DuelCommandBuilderError::NotExpectingParam);
//         }

//         for index in &indices {
//             if *index > 4 {
//                 return Err(DuelCommandBuilderError::InvalidHandIndex);
//             }
//         }

//         if indices.len() > 1 {
//             let mut unique_indices = indices.clone();
//             unique_indices.sort();
//             unique_indices.dedup();
//             if unique_indices.len() != indices.len() {
//                 return Err(DuelCommandBuilderError::InvalidHandIndex);
//             }
//             self.field_index.settable = true;
//         } else {
//             self.face_direction.settable = true;
//         }

//         self.hand_indices.value = Some(indices);
//         self.hand_indices.settable = false;
//         Ok(self)
//     }

//     pub fn face_direction(
//         &mut self,
//         face_direction: FaceDirection,
//     ) -> Result<&mut Self, DuelCommandBuilderError> {
//         self.check_command_set()?;

//         if !self.face_direction.settable {
//             return Err(DuelCommandBuilderError::NotExpectingParam);
//         }

//         let hand_index = self.hand_indices.value.as_ref().unwrap()[0];
//         let card = &self.duel.get_player().hand[hand_index];
//         match card.variant {
//             CardVariant::Monster { .. } | CardVariant::Trap { .. } | CardVariant::Equip { .. } => {
//                 self.field_index.settable = true;
//             }
//             CardVariant::Magic { .. } => {
//                 if face_direction == FaceDirection::Down {
//                     self.field_index.settable = true;
//                 } else {
//                     self.command = Some(HandPlaySingleMagicUpCmd { hand_index }.into());
//                 }
//             }
//             CardVariant::Ritual { .. } => {
//                 if face_direction == FaceDirection::Down {
//                     self.field_index.settable = true;
//                 } else {
//                     self.command = Some(HandPlaySingleRitualUpCmd { hand_index }.into());
//                 }
//             }
//         }

//         self.face_direction.value = Some(face_direction);
//         self.face_direction.settable = false;
//         Ok(self)
//     }

//     pub fn field_index(
//         &mut self,
//         field_index: usize,
//     ) -> Result<&mut Self, DuelCommandBuilderError> {
//         self.check_command_set()?;
        
//         if !self.field_index.settable {
//             return Err(DuelCommandBuilderError::NotExpectingParam);
//         }

//         if field_index > 4 {
//             return Err(DuelCommandBuilderError::InvalidFieldIndex);
//         }

//         self.field_index.value = Some(field_index);
//         self.field_index.settable = false;

//         // clone the player, and call play_hand
//         let mut player = self.duel.get_player().clone();
//         let hand_indices = self.hand_indices.value.as_ref().unwrap();
//         let result_card = player.play_hand(
//             &self.hand_indices.value.as_ref().unwrap(),
//             self.field_index.value.unwrap(),
//         );

//         match (hand_indices.len() > 1, result_card.variant) {
//             (true, _) => {
//                 self.command = Some(HandPlayMultipleCmd {
//                     hand_indices: hand_indices.clone(),
//                     field_index,
//                 }
//                 .into());
//             }
//             (false, CardVariant::Monster { .. }) => {
//                 self.command = Some(HandPlaySingleMonsterCmd {
//                     hand_index: hand_indices[0],
//                     field_index,
//                     face_direction: self.face_direction.value.unwrap(),
//                 }
//                 .into());
//             }
//             (false, CardVariant::Trap { .. }) => {
//                 self.command = Some(HandPlaySingleTrapCmd {
//                     hand_index: hand_indices[0],
//                     field_index,
//                     face_direction: self.face_direction.value.unwrap(),
//                 }
//                 .into());
//             }
//             (false, CardVariant::Magic { .. }) => {
//                 self.command = Some(HandPlaySingleMagicDownCmd {
//                     hand_index: hand_indices[0],
//                     field_index,
//                 }
//                 .into());
//             }
//             (false, CardVariant::Ritual { .. }) => {
//                 self.command = Some(HandPlaySingleRitualDownCmd {
//                     hand_index: hand_indices[0],
//                     field_index,
//                 }
//                 .into());
//             }
//             (false, CardVariant::Equip { .. }) => {
//                 self.command = Some(HandPlaySingleEquipCmd {
//                     hand_index: hand_indices[0],
//                     field_index,
//                     face_direction: self.face_direction.value.unwrap(),
//                 }
//                 .into());
//             }
//         }

//         Ok(self)
//     }

//     pub fn guardian_star_choice(
//         &mut self,
//         guardian_star_choice: GuardianStarChoice,
//     ) -> Result<&mut Self, DuelCommandBuilderError> {
//         self.check_command_set()?;

//         if !self.guardian_star_choice.settable {
//             return Err(DuelCommandBuilderError::NotExpectingParam);
//         }

//         self.guardian_star_choice.value = Some(guardian_star_choice);
//         self.guardian_star_choice.settable = false;

//         self.command = Some(SetGuardianStarCmd {
//             guardian_star_choice,
//         }.into());

//         Ok(self)
//     }

//     pub fn spell_row_index(
//         &mut self,
//         spell_row_index: usize,
//     ) -> Result<&mut Self, DuelCommandBuilderError> {
//         self.check_command_set()?;

//         if !self.spell_row_index.settable {
//             return Err(DuelCommandBuilderError::NotExpectingParam);
//         }

//         if spell_row_index > 4 {
//             return Err(DuelCommandBuilderError::InvalidFieldIndex);
//         }

//         let player = self.duel.get_player();
//         if player.spell_row[spell_row_index].is_none() {
//             return Err(DuelCommandBuilderError::InvalidFieldIndex);
//         }

//         let card_variant = &player.spell_row[spell_row_index]
//             .as_ref()
//             .unwrap()
//             .card
//             .variant;

//         if matches!(card_variant, CardVariant::Equip { .. }) {
//             self.monster_row_index.settable = true;
//         } else {
//             self.command = Some(FieldPlaySpellCmd {
//                 spell_row_index,
//             }
//             .into());
//         }

//         self.spell_row_index.value = Some(spell_row_index);
//         self.spell_row_index.settable = false;
//         Ok(self)
//     }

//     pub fn monster_row_index(
//         &mut self,
//         monster_row_index: usize,
//     ) -> Result<&mut Self, DuelCommandBuilderError> {
//         self.check_command_set()?;

//         if !self.monster_row_index.settable {
//             return Err(DuelCommandBuilderError::NotExpectingParam);
//         }

//         if monster_row_index > 4 {
//             return Err(DuelCommandBuilderError::InvalidFieldIndex);
//         }

//         // different behaviour based on whether spell_row_index is set
//         // if it is not, then we are expecting enemy_monster_row_index
//         // otherwise, we are not expecting anything
//         let is_spell_row_index_set = self.spell_row_index.value.is_some();
//         if !is_spell_row_index_set && self.duel.get_enemy().monster_row[monster_row_index].is_none()
//         {
//             return Err(DuelCommandBuilderError::InvalidFieldIndex);
//         }
//         self.enemy_monster_row_index.settable = !is_spell_row_index_set;

//         if is_spell_row_index_set {
//             // FieldPlayEquipCmd
//             self.command = Some(FieldPlayEquipCmd {
//                 spell_row_index: self.spell_row_index.value.unwrap(),
//                 monster_row_index,
//             }.into());
//         }

//         self.monster_row_index.value = Some(monster_row_index);
//         self.monster_row_index.settable = false;
//         Ok(self)
//     }

//     pub fn enemy_monster_row_index(
//         &mut self,
//         enemy_monster_row_index: usize,
//     ) -> Result<&mut Self, DuelCommandBuilderError> {
//         self.check_command_set()?;

//         if !self.enemy_monster_row_index.settable {
//             return Err(DuelCommandBuilderError::NotExpectingParam);
//         }

//         if enemy_monster_row_index > 4 {
//             return Err(DuelCommandBuilderError::InvalidFieldIndex);
//         }

//         let enemy = self.duel.get_enemy();
//         if enemy.monster_row[enemy_monster_row_index].is_none()
//             && !enemy.monster_row.iter().all(|monster| monster.is_none())
//         {
//             return Err(DuelCommandBuilderError::InvalidAttackTarget);
//         }

//         self.enemy_monster_row_index.value = Some(enemy_monster_row_index);
//         self.enemy_monster_row_index.settable = false;

//         self.command = Some(FieldAttackCmd {
//             monster_row_index: self.monster_row_index.value.unwrap(),
//             enemy_monster_row_index,
//         }.into());

//         Ok(self)
//     }

//     pub fn get_command(&self) -> Option<DuelCommandEnum> {
//         self.command.clone()
//     }

//     fn check_command_set(&self) -> Result<(), DuelCommandBuilderError> {
//         if self.command.is_some() {
//             return Err(DuelCommandBuilderError::CommandAlreadyBuilt);
//         }
//         Ok(())
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_hand_play_single_cmd() {
//         let mut duel = Duel::default();
//         let mut command_builder = CommandBuilder::new(&mut duel);

//         command_builder.hand_indices(vec![0]).unwrap();
//         command_builder.face_direction(FaceDirection::Up).unwrap();
//         // if command is now built, it will be a HandPlaySingleRitualUpCmd or HandPlaySingleMagicUpCmd
//         // otherwise, we need to set a field_index. then we will have a HandPlaySingleMonsterCmd/HandPlaySingleTrapCmd/HandPlaySingleEquipCmd/HandPlaySingleRitualDownCmd/HandPlaySingleMagicDownCmd
//         if let Some(command) = command_builder.get_command() {
//             match command {
//                 DuelCommandEnum::HandPlaySingleMagicUpCmd(HandPlaySingleMagicUpCmd { hand_index }) => {
//                     assert_eq!(hand_index, 0);
//                 }
//                 DuelCommandEnum::HandPlaySingleRitualUpCmd(HandPlaySingleRitualUpCmd { hand_index }) => {
//                     assert_eq!(hand_index, 0);
//                 }
//                 _ => panic!("Expected HandPlaySingleMagicUpCmd or HandPlaySingleRitualUpCmd"),
//             }
//         } else {
//             command_builder.field_index(0).unwrap();
//             let command = command_builder.get_command().unwrap();
//             match command {
//                 DuelCommandEnum::HandPlaySingleMonsterCmd(HandPlaySingleMonsterCmd { hand_index, field_index, face_direction }) => {
//                     assert_eq!(hand_index, 0);
//                     assert_eq!(face_direction, FaceDirection::Up);
//                     assert_eq!(field_index, 0);
//                 }
//                 DuelCommandEnum::HandPlaySingleTrapCmd(HandPlaySingleTrapCmd { hand_index, field_index, face_direction }) => {
//                     assert_eq!(hand_index, 0);
//                     assert_eq!(face_direction, FaceDirection::Up);
//                     assert_eq!(field_index, 0);
//                 }
//                 DuelCommandEnum::HandPlaySingleEquipCmd(HandPlaySingleEquipCmd { hand_index, field_index, face_direction }) => {
//                     assert_eq!(hand_index, 0);
//                     assert_eq!(face_direction, FaceDirection::Up);
//                     assert_eq!(field_index, 0);
//                 }
//                 _ => panic!("Expected HandPlaySingleMonsterCmd"),
//             }
//         }
//     }
// }
