use derive_builder::Builder;
use enum_dispatch::enum_dispatch;
use itertools::{iproduct, Itertools};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;

use crate::{
    card_from_id, check_all_successful_equips, combine, combine_cards,
    duel::field::{MonsterRowPosition, SpellRowPosition},
    get_amount_of_equip_boosts, guardian_star_relation, AdvantageRelation, Card, CardVariant,
    MagicEffect, TrapEffectEnum,
};

use super::{
    field::{CardMode, FaceDirection, GuardianStarChoice},
    state::*,
    Duel,
};

fn execute_spell(card: Card, duel: &mut Duel) {
    match card.variant {
        CardVariant::Magic(magic_effect) => {
            magic_effect.execute_effect(duel);

            // Check if either player has <= 0 life points. If so, set the duel state to EndState.
            if duel.get_player().life_points <= 0 || duel.get_enemy().life_points <= 0 {
                duel.state = EndState.into();
            } else {
                duel.state = FieldState.into();
            }
        }
        CardVariant::Ritual {
            card1_id,
            card2_id,
            card3_id,
            result_card_id,
        } => {
            // Loop through the player's monster row and check if the three cards are present.
            // If so, remove all of them from the field. Then, enter SetGuardianStarState with the ritual card.
            let mut found_cards = vec![];
            let mut found_card_ids = HashSet::new();
            for (index, monster_row_position) in duel.get_player().monster_row.iter().enumerate() {
                if let Some(monster_row_position) = monster_row_position {
                    let card_id = monster_row_position.card.id;
                    if (card_id == card1_id || card_id == card2_id || card_id == card3_id)
                        && !found_card_ids.contains(&card_id)
                    {
                        found_cards.push(index);
                        found_card_ids.insert(card_id);
                    }
                }
            }

            if found_cards.len() == 3 {
                // Remove the cards from the field
                for index in &found_cards {
                    duel.get_player_mut().monster_row[*index] = None;
                }

                let ritual_card = card_from_id(result_card_id);

                // Create the monster_row_pos to hold the ritual card
                let monster_row_pos = MonsterRowPosition {
                    card: ritual_card,
                    face_direction: FaceDirection::Up,
                    disabled: false,
                    card_mode: CardMode::Attack,
                    guardian_star_choice: GuardianStarChoice::A,
                };

                // Go to SetGuardianStarState
                duel.state = SetGuardianStarState {
                    monster_row_position: monster_row_pos,
                    monster_row_index: found_cards[0],
                    applied_equips_amount: None,
                }
                .into();
            } else {
                duel.state = FieldState.into();
            }
        }
        CardVariant::Equip { .. } | CardVariant::Trap { .. } => {
            duel.state = FieldState.into();
        }
        _ => panic!("execute_spell: Called on a monster card."),
    }
}

fn reverse_trap(monster_index: usize, equip_amount: u32, duel: &mut Duel) {
    // We need to loop through the enemy spell row and check for a trap with effect ReverseTrap.
    // If found, we need to remove the trap (set the spell row position to None).
    // We also need to negate the monster at monster_index's attack by equip_amount * 2.
    // We can do this with modify_stats.

    for spell_row_pos in duel.get_enemy_mut().spell_row.iter_mut() {
        if let Some(spell) = spell_row_pos {
            if let CardVariant::Trap(effect) = &spell.card.variant {
                if *effect == TrapEffectEnum::ReverseTrap {
                    // Remove the trap
                    *spell_row_pos = None;

                    // Negate the monster's attack
                    let monster = &mut duel.get_player_mut().monster_row[monster_index]
                        .as_mut()
                        .unwrap();
                    monster.card.modify_stats(-(equip_amount as i32) * 2);
                    break;
                }
            }
        }
    }
}

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
    #[error("Spell not present at the selected position.")]
    EquipNotPresentAtSelectedPosition,
    #[error("Cannot attack with a monster on the first turn.")]
    CannotAttackOnFirstTurn,
    #[error("Cannot attack while Swords of Revealing Light effect is active.")]
    CannotAttackWhileSORLEffectActive,
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
    fn execute(&self, duel: &mut Duel) -> Result<(), CommandError>;
    fn check_valid(&self, duel: &Duel) -> Result<(), CommandError>;
}

#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
pub struct HandPlaySingleCmd {
    pub hand_index: usize,
    pub face_direction: FaceDirection,
    pub field_index: Option<usize>,
}
impl DuelCommand for HandPlaySingleCmd {
    fn check_valid(&self, duel: &Duel) -> Result<(), CommandError> {
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
            if matches!(card.variant, CardVariant::Monster { .. })
                || (matches!(card.variant, CardVariant::Equip { .. })
                    && self.face_direction == FaceDirection::Up)
            {
                if field_index >= duel.get_player().monster_row.len() {
                    return Err(CommandError::OutOfBoundsFieldSelection);
                }
                // Check if monster is present at field index for Equip card
                if matches!(card.variant, CardVariant::Equip { .. })
                    && duel.get_player().monster_row[field_index].is_none()
                {
                    return Err(CommandError::CannotEquipEmptyPosition);
                }
            } else {
                if field_index >= duel.get_player().spell_row.len() {
                    return Err(CommandError::OutOfBoundsFieldSelection);
                }
            }
        }

        Ok(())
    }

    fn execute(&self, duel: &mut Duel) -> Result<(), CommandError> {
        self.check_valid(duel)?;

        // Check if the hand index is valid and remove the card from the hand
        let card = duel.get_player_mut().hand.remove(self.hand_index);

        // We need to check if a card is already present (there is a Some) at the field index
        // If so, we need to combine with the existing card. The result, wrapped in a MonsterRowPosition/SpellRowPosition, is placed at the field index.
        if let Some(field_index) = self.field_index {
            // if matches monster OR matches equip and face_direction is up
            if matches!(card.variant, CardVariant::Monster { .. })
                || (matches!(card.variant, CardVariant::Equip { .. })
                    && self.face_direction == FaceDirection::Up)
            {
                let existing_position = duel.get_player().monster_row[field_index].clone();
                let mut applied_equip = false;
                let mut card_mode = CardMode::Attack;
                let mut face_direction = self.face_direction;
                let mut equip_amount = None;
                let card_to_play = match existing_position {
                    Some(existing_card) => {
                        face_direction = FaceDirection::Up;
                        let ret = combine(&card, &existing_card.card);
                        // destructure ret and existing_card variants as monster to extract attack
                        if let (
                            CardVariant::Monster {
                                attack: existing_attack,
                                ..
                            },
                            CardVariant::Monster {
                                attack: new_attack, ..
                            },
                        ) = (&existing_card.card.variant, &ret.variant)
                        {
                            // if the new card is the same but with increased attack, we know we successfully applied an equip.
                            // in this case, we need to preserve the mode of the existing card and go to FieldState instead of SetGuardianStarState.
                            if ret.id == existing_card.card.id && new_attack > existing_attack {
                                applied_equip = true;
                                card_mode = existing_card.card_mode;
                                equip_amount = Some(new_attack - existing_attack);
                            }
                        }
                        ret
                    }
                    None => card.clone(),
                };

                // in some rare cases, an equip played faceup can fuse with an existing monster and create a spell.
                // for example, Sky Dragon + Machine Conversion Factory = Harpie's Feather Duster.
                // in this case, we need to call execute_spell and return early.
                match card_to_play.variant {
                    CardVariant::Magic{ .. } | CardVariant::Ritual{ .. } | CardVariant::Trap{ .. } | CardVariant::Equip{ .. } => {
                        duel.get_player_mut().monster_row[field_index] = None;
                        execute_spell(card_to_play, duel);
                        return Ok(());
                    } 
                    _ => {}
                }

                let monster_row_position = MonsterRowPosition {
                    card: card_to_play.clone(),
                    face_direction: face_direction,
                    disabled: false,
                    card_mode: card_mode,
                    guardian_star_choice: GuardianStarChoice::A,
                };

                // Remove the card from the hand.
                // duel.get_player_mut().hand.remove(self.hand_index);

                if !applied_equip {
                    duel.get_player_mut().monster_row[field_index] = None;
                    duel.state = (SetGuardianStarState {
                        monster_row_position,
                        monster_row_index: field_index,
                        applied_equips_amount: None,
                    })
                    .into()
                } else {
                    duel.get_player_mut().monster_row[field_index] = Some(monster_row_position);

                    reverse_trap(field_index, equip_amount.unwrap() as u32, duel);

                    duel.state = FieldState.into();
                };
            } else {
                let existing_position = duel.get_player().spell_row[field_index].clone();
                if existing_position.is_some() {
                    let card = combine(&card, &existing_position.as_ref().unwrap().card);
                    duel.get_player_mut().spell_row[field_index] = None;

                    execute_spell(card, duel);
                } else {
                    duel.get_player_mut().spell_row[field_index] = Some(SpellRowPosition {
                        card: card.clone(),
                        face_direction: self.face_direction,
                    });
                }
            }
        } else {
            // FaceUp Magic/Ritual
            execute_spell(card, duel);
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HandPlayMultipleCmd {
    pub hand_indices: Vec<usize>,
    pub field_index: usize,
}
impl DuelCommand for HandPlayMultipleCmd {
    fn check_valid(&self, duel: &Duel) -> Result<(), CommandError> {
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

        Ok(())
    }

    fn execute(&self, duel: &mut Duel) -> Result<(), CommandError> {
        self.check_valid(duel)?;

        // Collect all the cards at the specified indices into a vector in the order they were selected, removing them from the hand.
        let mut cards: Vec<_> = self
            .hand_indices
            .iter()
            .map(|&index| duel.get_player_mut().hand[index].clone())
            .collect();

        // remove all hand_indices from the hand. note that hand_indices is not sorted.
        self.hand_indices
            .iter()
            .sorted_by(|a, b| b.cmp(a))
            .for_each(|&index| {
                duel.get_player_mut().hand.remove(index);
            });

        // check if the field_index is occupied. if so, take the card and append it to the beginning of the cards vector.
        if let Some(existing_card) = &duel.get_player_mut().monster_row[self.field_index] {
            cards.insert(0, existing_card.card.clone());
        }

        // combine all the cards into a single card
        let combined_cards = combine_cards(cards.clone());
        let combined_card_result = combined_cards.last().unwrap().2.clone();

        // match on whether the card is a monster or otherwise
        match combined_card_result.variant {
            // if the result is a monster, and field_index has an existing card:
            // we need to check if ALL combined_cards were also monsters of the same ID as the existing card and with increased attack.
            // this signifies that every card was a successfully applied equip to the original card.
            // in this case, we go to FieldState, preserving the mode of the existing card.
            // otherwise, we go to SetGuardianStarState.
            // we can check that combined_cards.len() is one greater than self.hand_indices.len() to ensure that an existing card was combined.
            // on the other hand, if the result is not a monster, we do nothing for now and go to FieldState.
            CardVariant::Monster { .. } => {
                let mut card_mode = CardMode::Attack;
                let mut guardian_star_choice = GuardianStarChoice::A;
                let all_successful_equips = check_all_successful_equips(combined_cards.clone());

                if all_successful_equips {
                    if let Some(monster_row) =
                        duel.get_player().monster_row.get(self.field_index).unwrap()
                    {
                        card_mode = monster_row.card_mode;
                        guardian_star_choice = monster_row.guardian_star_choice;
                    }
                }

                // create the MonsterRowPosition
                // if all_successful_equips is true, we place the card and then go to FieldState, preserving the mode of the existing card.
                // otherwise, we go to SetGuardianStarState.
                let monster_row_position = MonsterRowPosition {
                    card: combined_card_result.clone(),
                    face_direction: FaceDirection::Up,
                    disabled: false,
                    card_mode: card_mode,
                    guardian_star_choice: guardian_star_choice,
                };

                // We need to check how much equips were applied during the combination.
                // For each window, we need to check if the previous card was the same as the current card, and if the attack increased.
                // If so, increase applied_equips_amount by the difference in attack.
                let applied_equips_amount = get_amount_of_equip_boosts(combined_cards.clone());

                // Change the duel state based on whether all equips were successful
                if all_successful_equips {
                    duel.get_player_mut().monster_row[self.field_index] =
                        Some(monster_row_position);

                    reverse_trap(self.field_index, applied_equips_amount as u32, duel);

                    duel.state = FieldState.into();
                } else {
                    duel.get_player_mut().monster_row[self.field_index] = None;
                    duel.state = SetGuardianStarState {
                        monster_row_position,
                        monster_row_index: self.field_index,
                        applied_equips_amount: if applied_equips_amount > 0 {
                            Some(applied_equips_amount as u32)
                        } else {
                            None
                        },
                    }
                    .into();
                }
            }
            _ => {
                execute_spell(combined_card_result.clone(), duel);
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetGuardianStarCmd {
    pub guardian_star_choice: GuardianStarChoice,
}
impl DuelCommand for SetGuardianStarCmd {
    fn check_valid(&self, duel: &Duel) -> Result<(), CommandError> {
        // Check if the duel is in the correct state (SetGuardianStarState)
        if !matches!(duel.state, DuelStateEnum::SetGuardianStarState(_)) {
            return Err(CommandError::InvalidDuelState);
        }

        Ok(())
    }

    fn execute(&self, duel: &mut Duel) -> Result<(), CommandError> {
        self.check_valid(duel)?;

        // Edit the monster_row_position's guardian_star_choice to be the selected choice
        // Then simply place it in the monster_row at the monster_row_index
        if let DuelStateEnum::SetGuardianStarState(state) = duel.state.clone() {
            let mut monster_row_position = state.monster_row_position.clone();
            monster_row_position.guardian_star_choice = self.guardian_star_choice;
            duel.get_player_mut().monster_row[state.monster_row_index] = Some(monster_row_position);

            match state.applied_equips_amount {
                Some(amount) => {
                    reverse_trap(state.monster_row_index, amount, duel);
                }
                None => {
                    // Do nothing
                }
            }

            duel.state = FieldState.into();
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldAttackCmd {
    pub monster_row_index: usize,
    pub enemy_monster_row_index: usize,
}
impl DuelCommand for FieldAttackCmd {
    fn check_valid(&self, duel: &Duel) -> Result<(), CommandError> {
        // Check if the duel is in the correct state (FieldState)
        if !matches!(duel.state, DuelStateEnum::FieldState(_)) {
            return Err(CommandError::InvalidDuelState);
        }

        // Check if it's the first turn. If so, the player cannot attack.
        if duel.turn == 0 {
            return Err(CommandError::CannotAttackOnFirstTurn);
        }

        // Check that the sorl_effect_countdown is None. If not, the player cannot attack.
        if duel.get_player().sorl_effect_countdown.is_some() {
            return Err(CommandError::CannotAttackWhileSORLEffectActive);
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

        if self.enemy_monster_row_index >= duel.get_enemy().monster_row.len() {
            return Err(CommandError::OutOfBoundsFieldSelection);
        }

        if duel.get_enemy().monster_row[self.enemy_monster_row_index].is_none() {
            if duel
                .get_enemy()
                .monster_row
                .iter()
                .any(|monster| monster.is_some())
            {
                return Err(CommandError::CannotAttackEmptyPositionWhileMonstersPresent);
            }
        }

        Ok(())
    }

    fn execute(&self, duel: &mut Duel) -> Result<(), CommandError> {
        self.check_valid(duel)?;

        // Loop through the enemy spell row to check for traps.
        // We are looking for TrapEffectEnum::FakeTrap or TrapEffectEnum::DestroyAttacker { attack_threshold }
        // If we find a DestroyAttacker, we need to check if the attack of the attacking monster is <= attack_threshold.
        // If so, we need to destroy the attacking monster and the trap.
        // In the case of FakeTrap, we just need to destroy the trap and cancel the attack, setting the attacker to disabled.
        for (index, spell_row_position) in duel.get_enemy().spell_row.iter().enumerate() {
            if let Some(spell) = spell_row_position {
                if let CardVariant::Trap(trap_effect) = spell.card.variant {
                    match trap_effect {
                        TrapEffectEnum::FakeTrap => {
                            duel.get_enemy_mut().spell_row[index] = None;
                            let monster = duel.get_player_mut().monster_row[self.monster_row_index]
                                .as_mut()
                                .unwrap();

                            monster.disabled = true;
                            return Ok(());
                        }
                        TrapEffectEnum::DestroyAttacker {
                            attack_factor_threshold,
                        } => {
                            let (attacker_attack, _) = duel.get_player().monster_row
                                [self.monster_row_index]
                                .as_ref()
                                .unwrap()
                                .card
                                .get_stats_with_terrain(duel.terrain_type)
                                .unwrap();
                            if attack_factor_threshold.is_none()
                                || attacker_attack <= attack_factor_threshold.unwrap() as i32
                            {
                                duel.get_enemy_mut().spell_row[index] = None;
                                duel.get_player_mut().monster_row[self.monster_row_index] = None;
                                return Ok(());
                            }
                        }
                        _ => {
                            // Do nothing
                        }
                    }
                }
            }
        }

        if duel.get_enemy_mut().monster_row[self.enemy_monster_row_index].is_none() {
            if duel
                .get_enemy_mut()
                .monster_row
                .iter()
                .any(|monster| monster.is_some())
            {
                return Err(CommandError::CannotAttackEmptyPositionWhileMonstersPresent);
            } else {
                // Extract the CardVariant from the attacking monster.
                // Negate the life points of the enemy player by the attack of the attacking monster.
                // Set the attacking monster's disabled to true.
                let attacking_monster = duel.get_player_mut().monster_row[self.monster_row_index]
                    .clone()
                    .unwrap();
                if let CardVariant::Monster { attack, .. } = attacking_monster.card.variant {
                    // Attack can be negative. If so, we need to round up to 0 before inflicting damage.
                    let damage = -(attack.max(0));
                    duel.get_enemy_mut().modify_life_points(damage);
                }
            }
        } else {
            // - Monster attack:
            // - Select position of a monster to attack with, and position of an enemy monster to attack.
            // - If the enemy card is in attack mode:
            //     - Compare attack stats. If stats are equal, both cards are destroyed. If one card is weaker, the weaker card is destroyed, and the difference in attack stat is taken as life point damage.
            // - If the enemy card is in defense mode:
            //     - Compare the attacker’s attack stat with the defender’s defense stat.
            //     - If the attacker has a higher stat, then the defender is destroyed. No life point damage is taken.
            //     - If the defender has a higher stat, then the attacker is destroyed. The difference between the two stats is taken as life point damage.
            //     - If the stats are equal, neither card is destroyed.
            //     - Any surviving cards after any type of monster attack will be left face up.
            let enemy_monster = duel.get_enemy_mut().monster_row[self.enemy_monster_row_index]
                .clone()
                .unwrap();
            let attacking_monster = duel.get_player_mut().monster_row[self.monster_row_index]
                .clone()
                .unwrap();

            let (mut attacker_attack, mut _attacker_defense) = attacking_monster
                .card
                .get_stats_with_terrain(duel.terrain_type)
                .unwrap();
            let (mut enemy_attack, mut enemy_defense) = enemy_monster
                .card
                .get_stats_with_terrain(duel.terrain_type)
                .unwrap();

            // All stats can be negative. Round them all to 0.
            attacker_attack = attacker_attack.max(0);
            _attacker_defense = _attacker_defense.max(0);
            enemy_attack = enemy_attack.max(0);
            enemy_defense = enemy_defense.max(0);

            // for both monsters, use guardian_star_relation function to check if advantageous (+500), disadvantageous (-500), or neutral (no change)
            let (attacker_gs, enemy_gs) = (
                attacking_monster.get_selected_gs(),
                enemy_monster.get_selected_gs(),
            );
            match guardian_star_relation(attacker_gs, enemy_gs) {
                AdvantageRelation::Advantaged => {
                    attacker_attack += 500;
                    _attacker_defense += 500;
                }
                AdvantageRelation::Disadvantaged => {
                    enemy_attack += 500;
                    enemy_defense += 500;
                }
                AdvantageRelation::Neutral => {
                    // no change
                }
            }

            match enemy_monster.card_mode {
                CardMode::Attack => {
                    let damage = (attacker_attack - enemy_attack).abs();
                    if attacker_attack > enemy_attack {
                        // Attacker wins, enemy monster is destroyed and difference in attack is taken as life point damage
                        duel.get_enemy_mut().monster_row[self.enemy_monster_row_index] = None;
                        duel.get_enemy_mut().modify_life_points(-damage);
                    } else if attacker_attack < enemy_attack {
                        // Enemy wins, attacking monster is destroyed and difference in attack is taken as life point damage
                        duel.get_player_mut().monster_row[self.monster_row_index] = None;
                        duel.get_player_mut().modify_life_points(-damage);
                    } else {
                        // Both monsters are destroyed
                        duel.get_enemy_mut().monster_row[self.enemy_monster_row_index] = None;
                        duel.get_player_mut().monster_row[self.monster_row_index] = None;
                    }
                }
                CardMode::Defense => {
                    if attacker_attack > enemy_defense {
                        // Attacker wins, enemy monster is destroyed
                        duel.get_enemy_mut().monster_row[self.enemy_monster_row_index] = None;
                    } else {
                        // Defender wins (or draw)
                        // attacking monster is disabled and difference of enemy_defense - attacker_attack is taken as life point damage
                        // if it's a draw, that means the difference is 0, so no damage is taken.
                        let damage = (enemy_defense - attacker_attack).abs();
                        // duel.get_player_mut().monster_row[self.monster_row_index] = None;
                        duel.get_player_mut().modify_life_points(-damage);
                    }
                }
            }
        }

        // Any surviving cards after any type of monster attack will be left face up. Also, the attacking monster is disabled.
        if let Some(monster) = duel.get_player_mut().monster_row[self.monster_row_index].as_mut() {
            monster.face_direction = FaceDirection::Up;
            monster.disabled = true;
        }
        if let Some(monster) =
            duel.get_enemy_mut().monster_row[self.enemy_monster_row_index].as_mut()
        {
            monster.face_direction = FaceDirection::Up;
        }

        // Check both players to see if either has <= 0 life points. If so, set the duel state to EndState.
        if duel.get_player().life_points <= 0 || duel.get_enemy().life_points <= 0 {
            duel.state = EndState.into();
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldChangeModeCmd {
    pub monster_index: usize,
}
impl DuelCommand for FieldChangeModeCmd {
    fn check_valid(&self, duel: &Duel) -> Result<(), CommandError> {
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

        Ok(())
    }
    fn execute(&self, duel: &mut Duel) -> Result<(), CommandError> {
        self.check_valid(duel)?;

        // Change the card mode of the monster at monster_index
        // If the card mode was attack, change it to defense. If it was defense, change it to attack.
        if let Some(monster) = duel.get_player_mut().monster_row[self.monster_index].as_mut() {
            monster.card_mode = match monster.card_mode {
                CardMode::Attack => CardMode::Defense,
                CardMode::Defense => CardMode::Attack,
            };
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct FieldPlaySpellCmd {
    pub spell_row_index: usize,
}
impl DuelCommand for FieldPlaySpellCmd {
    fn check_valid(&self, duel: &Duel) -> Result<(), CommandError> {
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

        Ok(())
    }
    fn execute(&self, duel: &mut Duel) -> Result<(), CommandError> {
        self.check_valid(duel)?;

        // Get a reference to the spell without taking it
        let spell = duel.get_player().spell_row[self.spell_row_index]
            .as_ref()
            .unwrap();

        // Match on the spell type. If it is a magic/ritual/trap, do nothing for now.
        // If it is an equip, go to FieldEquipSelectedState.
        match spell.card.variant {
            CardVariant::Equip { .. } => {
                duel.state = FieldEquipSelectedState {
                    spell_row_index: self.spell_row_index,
                }
                .into();
            }
            _ => {
                // leave None in the spell row
                let card = spell.card.clone();
                duel.get_player_mut().spell_row[self.spell_row_index] = None;

                execute_spell(card, duel);
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldCancelPlayEquipCmd;
impl DuelCommand for FieldCancelPlayEquipCmd {
    fn check_valid(&self, duel: &Duel) -> Result<(), CommandError> {
        // Check for FieldEquipSelectedState
        if !matches!(duel.state, DuelStateEnum::FieldEquipSelectedState(_)) {
            return Err(CommandError::InvalidDuelState);
        }

        Ok(())
    }
    fn execute(&self, duel: &mut Duel) -> Result<(), CommandError> {
        self.check_valid(duel)?;

        // return the duel unmodified, except the state is set to FieldState.
        duel.state = FieldState.into();
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldPlayEquipPickMonsterCmd {
    pub monster_row_index: usize,
}
impl DuelCommand for FieldPlayEquipPickMonsterCmd {
    fn check_valid(&self, duel: &Duel) -> Result<(), CommandError> {
        // Check for FieldEquipSelectedState
        // extract the state from the FieldEquipSelectedState
        let state = match duel.state.clone() {
            DuelStateEnum::FieldEquipSelectedState(state) => state,
            _ => return Err(CommandError::InvalidDuelState),
        };

        // Check that the state.spell_row_index contains an equip
        let spell = duel
            .get_player()
            .spell_row
            .get(state.spell_row_index)
            .ok_or(CommandError::OutOfBoundsFieldSelection)?;

        if spell.is_none() {
            return Err(CommandError::EquipNotPresentAtSelectedPosition);
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

        Ok(())
    }
    fn execute(&self, duel: &mut Duel) -> Result<(), CommandError> {
        self.check_valid(duel)?;

        // Apply the equip to the monster at monster_row_index
        // If it succeeds, go to FieldState.
        // Otherwise, go to SetGuardianStarState.
        // Get the current state
        let state = match duel.state.clone() {
            DuelStateEnum::FieldEquipSelectedState(state) => state,
            _ => unreachable!(),
        };

        // Take the equip card from the spell row, leaving None in its place
        let equip_card = duel.get_player_mut().spell_row[state.spell_row_index]
            .take()
            .unwrap();

        // Apply the equip card to the monster by calling combine
        let mut monster = duel.get_player_mut().monster_row[self.monster_row_index]
            .clone()
            .unwrap();
        let combined_card = combine(&monster.card, &equip_card.card);

        // If the combined card has a higher attack than the original monster, then the equip was successful.
        // We need to destructure the card.variant as monster to extract the attack.
        if let CardVariant::Monster {
            attack: combined_attack,
            ..
        } = combined_card.variant
        {
            if let CardVariant::Monster {
                attack: original_attack,
                ..
            } = monster.card.variant
            {
                monster.face_direction = FaceDirection::Up;
                monster.card = combined_card;
                if combined_attack > original_attack {
                    duel.get_player_mut().monster_row[self.monster_row_index] = Some(monster);

                    reverse_trap(
                        self.monster_row_index,
                        (combined_attack - original_attack) as u32,
                        duel,
                    );

                    duel.state = FieldState.into();
                } else {
                    // Equip was not successful, go to SetGuardianStarState
                    duel.get_player_mut().monster_row[self.monster_row_index] = None;
                    duel.state = SetGuardianStarState {
                        monster_row_position: monster.clone(),
                        monster_row_index: self.monster_row_index,
                        applied_equips_amount: None,
                    }
                    .into();
                }
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndTurnCmd;
impl DuelCommand for EndTurnCmd {
    fn check_valid(&self, duel: &Duel) -> Result<(), CommandError> {
        // Check for FieldState
        if !matches!(duel.state, DuelStateEnum::FieldState(_)) {
            return Err(CommandError::InvalidDuelState);
        }

        Ok(())
    }
    fn execute(&self, duel: &mut Duel) -> Result<(), CommandError> {
        self.check_valid(duel)?;

        duel.turn += 1;

        // for the current player, if sorl_effect_countdown is Some, decrement it. if it is 0, set it to None.
        if let Some(countdown) = duel.get_player_mut().sorl_effect_countdown {
            if countdown == 0 {
                duel.get_player_mut().sorl_effect_countdown = None;
            } else {
                duel.get_player_mut().sorl_effect_countdown = Some(countdown - 1);
            }
        }

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

impl DuelCommandEnum {
    pub fn generate_all_valid(duel: &Duel) -> Vec<DuelCommandEnum> {
        // To generate all HandPlaySingleCmds, we need to generate all possible combinations (cartesian product) of hand_index, face_direction, and field_index.
        // hand_index ranges between 0 and the number of cards in the hand.
        // FaceDirection is FaceDirection::Up or FaceDirection::Down.
        // field_index ranges between 0 and 4 inclusive wrapped in Some, or None.
        // then, we need to call check_valid on each of these combinations. some will be invalid, so we need to filter those out.
        // for cartesian product, we can use itertools

        let hand_indices = 0..duel.get_player().hand.len();
        let face_directions = vec![FaceDirection::Up, FaceDirection::Down];
        let field_indices = (0..5).map(Some).chain(std::iter::once(None));

        let hand_play_single_cmds = iproduct!(hand_indices, face_directions, field_indices)
            .filter_map(|(hand_index, face_direction, field_index)| {
                let cmd = HandPlaySingleCmd {
                    hand_index,
                    face_direction,
                    field_index,
                };
                if cmd.check_valid(duel).is_ok() {
                    Some(DuelCommandEnum::HandPlaySingleCmd(cmd))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // To generate all HandPlayMultipleCmd:
        // We need to start by generating all possible combinations of hand indices.
        // To do this, we need to get the hand length. Then, we need to generate all possible combinations of hand indices of length 2 to 5 inclusive.
        // For example, if the hand length is 5, we need to generate all combinations of length 2, 3, 4, and 5.
        // This would include [0, 1], [0, 2], [0, 3], [0, 4], [1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4], [0, 1, 2], [0, 1, 3], [0, 1, 4], [0, 2, 3], [0, 2, 4], [0, 3, 4], [1, 2, 3], [1, 2, 4], [1, 3, 4], [2, 3, 4], [0, 1, 2, 3], [0, 1, 2, 4], [0, 1, 3, 4], [0, 2, 3, 4], [1, 2, 3, 4], [0, 1, 2, 3, 4], etc
        let hand_length = duel.get_player().hand.len();
        let hand_indices = 0..hand_length;
        let hand_indices_combinations = (2..=5)
            .flat_map(|n| hand_indices.clone().permutations(n))
            .collect::<Vec<_>>();

        // now, get the cartesian product of hand_indices_combinations and field_indices
        // this type, None is not an option for field_index
        let field_indices = 0..5;
        let hand_play_multiple_cmds = iproduct!(hand_indices_combinations, field_indices)
            .filter_map(|(hand_indices, field_index)| {
                let cmd = HandPlayMultipleCmd {
                    hand_indices,
                    field_index,
                };
                if cmd.check_valid(duel).is_ok() {
                    Some(DuelCommandEnum::HandPlayMultipleCmd(cmd))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // To generate all SetGuardianStarCmd:
        // We need to generate all possible combinations of guardian_star_choice.
        // This is simply Up or Down.
        let guardian_star_choices = vec![GuardianStarChoice::A, GuardianStarChoice::B];

        let set_guardian_star_cmds = guardian_star_choices
            .into_iter()
            .filter_map(|guardian_star_choice| {
                let cmd = SetGuardianStarCmd {
                    guardian_star_choice,
                };
                if cmd.check_valid(duel).is_ok() {
                    Some(DuelCommandEnum::SetGuardianStarCmd(cmd))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // To generate all FieldAttackCmd:
        // We need to generate all possible combinations of monster_row_index and enemy_monster_row_index.
        // monster_row_index ranges between 0 and 4 inclusive.
        // enemy_monster_row_index ranges between 0 and 4 inclusive.
        // We need to filter out invalid combinations.
        let monster_row_indices = 0..5;
        let enemy_monster_row_indices = 0..5;

        let field_attack_cmds = iproduct!(monster_row_indices, enemy_monster_row_indices)
            .filter_map(|(monster_row_index, enemy_monster_row_index)| {
                let cmd = FieldAttackCmd {
                    monster_row_index,
                    enemy_monster_row_index,
                };
                if cmd.check_valid(duel).is_ok() {
                    Some(DuelCommandEnum::FieldAttackCmd(cmd))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // To generate all FieldChangeModeCmd:
        // We need to generate all possible combinations of monster_row_index.
        // monster_row_index ranges between 0 and 4 inclusive.
        // We need to filter out invalid combinations.
        let monster_row_indices = 0..5;
        let field_change_mode_cmds = monster_row_indices
            .into_iter()
            .filter_map(|monster_row_index| {
                let cmd = FieldChangeModeCmd {
                    monster_index: monster_row_index,
                };
                if cmd.check_valid(duel).is_ok() {
                    Some(DuelCommandEnum::FieldChangeModeCmd(cmd))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // To generate all FieldPlaySpellCmd:
        // We need to generate all possible combinations of spell_row_index.
        // spell_row_index ranges between 0 and 4 inclusive.
        // We need to filter out invalid combinations.
        let spell_row_indices = 0..5;
        let field_play_spell_cmds = spell_row_indices
            .into_iter()
            .filter_map(|spell_row_index| {
                let cmd = FieldPlaySpellCmd { spell_row_index };
                if cmd.check_valid(duel).is_ok() {
                    Some(DuelCommandEnum::FieldPlaySpellCmd(cmd))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // To generate all FieldCancelPlayEquipCmd:
        // There is only one possibility, but we also need to check if it is valid
        let field_cancel_play_equip_cmds = vec![DuelCommandEnum::FieldCancelPlayEquipCmd(
            FieldCancelPlayEquipCmd,
        )];
        let field_cancel_play_equip_cmds = field_cancel_play_equip_cmds
            .into_iter()
            .filter_map(|cmd| {
                if cmd.check_valid(duel).is_ok() {
                    Some(cmd)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // To generate all FieldPlayEquipPickMonsterCmd:
        // We need to generate all possible combinations of monster_row_index.
        // monster_row_index ranges between 0 and 4 inclusive.
        // We need to filter out invalid combinations.
        let monster_row_indices = 0..5;
        let field_play_equip_pick_monster_cmds = monster_row_indices
            .into_iter()
            .filter_map(|monster_row_index| {
                let cmd = FieldPlayEquipPickMonsterCmd { monster_row_index };
                if cmd.check_valid(duel).is_ok() {
                    Some(DuelCommandEnum::FieldPlayEquipPickMonsterCmd(cmd))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // To generate all EndTurnCmd:
        // There is only one possibility, but we also need to check if it is valid
        let end_turn_cmds = vec![DuelCommandEnum::EndTurnCmd(EndTurnCmd)];
        let end_turn_cmds = end_turn_cmds
            .into_iter()
            .filter_map(|cmd| {
                if cmd.check_valid(duel).is_ok() {
                    Some(cmd)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // Combine all the commands into a single vector
        let mut commands = Vec::new();
        commands.extend(hand_play_single_cmds);
        commands.extend(hand_play_multiple_cmds);
        commands.extend(set_guardian_star_cmds);
        commands.extend(field_attack_cmds);
        commands.extend(field_change_mode_cmds);
        commands.extend(field_play_spell_cmds);
        commands.extend(field_cancel_play_equip_cmds);
        commands.extend(field_play_equip_pick_monster_cmds);
        commands.extend(end_turn_cmds);

        commands
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    // create a default duel, generate all valid moves, and dbg print them
    #[test]
    fn test_generate_all_valid() {
        let duel = Duel::default();
        let commands = DuelCommandEnum::generate_all_valid(&duel);
        // dbg!(&commands);
        // dbg!(&commands.len());
        // print first 100 commands
        for command in commands.iter().take(100) {
            dbg!(command);
        }
        dbg!(&commands.len());

        // if the commands len is 1650, assert that all the cards in the hand must be monsters
        if commands.len() == 1650 {
            for card in duel.get_player().hand.iter() {
                if let CardVariant::Monster { .. } = card.variant {
                } else if let CardVariant::Trap { .. } = card.variant {
                } else {
                    // dbg print the hand
                    dbg!(&duel.get_player().hand);
                    assert!(false);
                }
            }
        }
    }

    // create a default duel, then benchmark the generation of all valid commands
    #[bench]
    fn bench_generate_all_valid(b: &mut Bencher) {
        let duel = Duel::default();
        b.iter(|| {
            let commands = DuelCommandEnum::generate_all_valid(&duel);
            let mut max_commands_len = 1650;
            // for each magic/ritual/equip card in the hand, reduce the max_commands_len by 5
            for card in duel.get_player().hand.iter() {
                if let CardVariant::Magic { .. } = card.variant {
                    max_commands_len -= 5;
                }
                if let CardVariant::Ritual { .. } = card.variant {
                    max_commands_len -= 5;
                }
                if let CardVariant::Equip { .. } = card.variant {
                    // since its the first turn, there cannot be any valid monsters to equip to yet.
                    // therefore equip can only be placed facedown in the spell row, so reduce the max_commands_len by 5.
                    max_commands_len -= 5;
                }
            }
            //dbg print the hand
            dbg!(&duel.get_player().hand);
            assert_eq!(commands.len(), max_commands_len);
        });
    }
}
