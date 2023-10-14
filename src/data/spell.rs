use crate::{Duel, MonsterType, TerrainType};
use enum_dispatch::enum_dispatch;

pub enum TrapEffect {
    ReverseTrap, // Reverses equip cards to negate stats instead of boosting them
    GoblinFan, // Reflects damage magic effects e.g. sparks
    BadReactionToSimochi, // Reverses healing magic effects e.g. Mooyan Curry
    FakeTrap, // Activates when the enemy attacks, but does nothing
    DestroyAttacker { // House of Adhesive Tape (500), Eatgaboon (1000), Bear Trap (1500), Invisible Wire (2000), Acid Trap Hole (3000), Widespread Ruin (no threshold)
        attack_factor_threshold: Option<u32>,
    },
}

#[enum_dispatch]
pub trait MagicEffect {
    fn execute_effect(&self, duel: &mut Duel);
}

// Forest, Wasteland, Mountain, Sogen, Umi, Yami
pub struct ChangeTerrainEffect {
    pub terrain_type: TerrainType,
}

impl MagicEffect for ChangeTerrainEffect {
    fn execute_effect(&self, duel: &mut Duel) {
        duel.terrain_type = self.terrain_type;
    }
}

// Dragon Capture Jar, Warrior Elimination, Eternal Rest, Stain Storm, Eradicating Aerosol, Breath of Light, Eternal Draught
pub struct MonsterDestroyerEffect {
    pub monster_type: MonsterType,
}

impl MagicEffect for MonsterDestroyerEffect {
    fn execute_effect(&self, duel: &mut Duel) {
        // set all to None
        for monster_row_pos in duel.get_enemy_mut().monster_row.iter_mut() {
            if let Some(monster) = monster_row_pos {
                if let crate::CardVariant::Monster { monster_type, .. } = &monster.card.variant {
                    if self.monster_type == *monster_type {
                        *monster_row_pos = None;
                    }
                }
            }
        }
    }
}

pub struct DarkHoleEffect;
impl MagicEffect for DarkHoleEffect {
    fn execute_effect(&self, duel: &mut Duel) {
        duel.get_enemy_mut()
            .monster_row
            .iter_mut()
            .for_each(|monster| *monster = None);
        duel.get_enemy_mut()
            .spell_row
            .iter_mut()
            .for_each(|spell| *spell = None);
    }
}

pub struct RaigekiEffect;
impl MagicEffect for RaigekiEffect {
    fn execute_effect(&self, duel: &mut Duel) {
        duel.get_enemy_mut()
            .monster_row
            .iter_mut()
            .for_each(|monster| *monster = None);
    }
}

pub struct HarpiesFeatherDusterEffect;
impl MagicEffect for HarpiesFeatherDusterEffect {
    fn execute_effect(&self, duel: &mut Duel) {
        duel.get_enemy_mut()
            .spell_row
            .iter_mut()
            .for_each(|spell| *spell = None);
    }
}

pub struct CrushCardEffect;
impl MagicEffect for CrushCardEffect {
    fn execute_effect(&self, duel: &mut Duel) {
        // Set all enemy monster cards to None if their attack is 1500 or higher
        duel.get_enemy_mut()
            .monster_row
            .iter_mut()
            .for_each(|monster_row_pos| {
                if let Some(monster) = monster_row_pos {
                    if let crate::CardVariant::Monster { attack, .. } = &monster.card.variant {
                        if *attack >= 1500 {
                            *monster_row_pos = None;
                        }
                    }
                }
            });
    }
}

// Mooyan Curry (200), Red Medicine (500), Goblin's Secret Remedy (1000), Soul of the Pure (2000), Dian Keto the Cure Master (5000)
pub struct LifePointHealerEffect {
    pub amount: u32,
}

impl MagicEffect for LifePointHealerEffect {
    fn execute_effect(&self, duel: &mut Duel) {
        // TODO: Activate trap cards
        duel.get_player_mut().modify_life_points(self.amount as i32);
    }
}

// Sparks (50), Hinotama (100), Final Flame (500), Ookazi (500), Tremendous Fire (1000)
pub struct LifePointDamagerEffect {
    pub amount: u32,
}

impl MagicEffect for LifePointDamagerEffect {
    fn execute_effect(&self, duel: &mut Duel) {
        // TODO: Activate trap cards
        duel.get_enemy_mut()
            .modify_life_points(-(self.amount as i32));
    }
}

// Spellbinding Curse, Shadow Spell
pub struct StatReducerEffect {
    pub amount: u32,
}

impl MagicEffect for StatReducerEffect {
    fn execute_effect(&self, duel: &mut Duel) {
        // For each enemy monster card, reduce its stats by amount
        duel.get_enemy_mut()
            .monster_row
            .iter_mut()
            .for_each(|monster| {
                if let Some(monster) = monster {
                    monster.card.modify_stats(-(self.amount as i32));
                }
            });
    }
}

pub struct StopDefenseEffect;
impl MagicEffect for StopDefenseEffect {
    fn execute_effect(&self, duel: &mut Duel) {
        // For each enemy monster card, set its card_mode to Attack
        duel.get_enemy_mut()
            .monster_row
            .iter_mut()
            .for_each(|monster| {
                if let Some(monster) = monster {
                    monster.card_mode = crate::duel::field::CardMode::Attack;
                }
            });
    }
}

pub struct DarkPiercingLightEffect;
impl MagicEffect for DarkPiercingLightEffect {
    fn execute_effect(&self, duel: &mut Duel) {
        // For each enemy monster card, set its card_mode to Attack
        duel.get_enemy_mut()
            .monster_row
            .iter_mut()
            .for_each(|monster| {
                if let Some(monster) = monster {
                    monster.face_direction = crate::duel::field::FaceDirection::Up;
                }
            });
    }
}

// For each enemy monster card, get the base stats of the card (card.get_base_stats())
// If the base stats are higher than the current stats (card.get_stats_no_terrain()), set the current stats to the base stats.
// This can be done with card.modify_stats(delta) where delta is the difference between the current and base stats.
pub struct CursebreakerEffect;
impl MagicEffect for CursebreakerEffect {
    fn execute_effect(&self, duel: &mut Duel) {
        duel.get_enemy_mut()
            .monster_row
            .iter_mut()
            .for_each(|monster| {
                if let Some(monster) = monster {
                    if let Some((base_attack, base_defense)) = monster.card.get_base_stats() {
                        if let Some((current_attack, current_defense)) =
                            monster.card.get_stats_no_terrain()
                        {
                            if base_attack > current_attack {
                                // assert that the difference between base_attack and current_attack is the same as the difference between base_defense and current_defense
                                assert_eq!(
                                    base_attack as i32 - current_attack as i32,
                                    base_defense as i32 - current_defense as i32
                                );
                                monster.card.modify_stats((base_attack - current_attack) as i32);
                            }
                        }
                    }
                }
            });
    }
}

pub struct SwordsOfRevealingLightEffect;
// Set all enemy monster cards to faceup. Set sorl_effect_countdown to Some(3).
impl MagicEffect for SwordsOfRevealingLightEffect {
    fn execute_effect(&self, duel: &mut Duel) {
        duel.get_enemy_mut()
            .monster_row
            .iter_mut()
            .for_each(|monster| {
                if let Some(monster) = monster {
                    monster.face_direction = crate::duel::field::FaceDirection::Up;
                }
            });
        duel.get_enemy_mut().sorl_effect_countdown = Some(3);
    }
}

#[enum_dispatch(MagicEffect)]
pub enum MagicEffectEnum {
    ChangeTerrainEffect,
    MonsterDestroyerEffect,
    DarkHoleEffect,
    RaigekiEffect,
    HarpiesFeatherDusterEffect,
    LifePointHealerEffect,
    LifePointDamagerEffect,
    StatReducerEffect,
    StopDefenseEffect,
    DarkPiercingLightEffect,
    CursebreakerEffect,
    SwordsOfRevealingLightEffect,
}