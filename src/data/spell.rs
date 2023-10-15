use crate::{Duel, MonsterType, TerrainType};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum TrapEffectEnum {
    ReverseTrap,          // Reverses equip cards to negate stats instead of boosting them
    GoblinFan,            // Reflects damage magic effects e.g. sparks
    BadReactionToSimochi, // Reverses healing magic effects e.g. Mooyan Curry
    FakeTrap,             // Activates when the enemy attacks, but does nothing
    DestroyAttacker {
        // House of Adhesive Tape (500), Eatgaboon (1000), Bear Trap (1500), Invisible Wire (2000), Acid Trap Hole (3000), Widespread Ruin (no threshold)
        attack_factor_threshold: Option<u32>,
    },
}

#[enum_dispatch]
pub trait MagicEffect {
    fn execute_effect(&self, duel: &mut Duel);
}

// Forest, Wasteland, Mountain, Sogen, Umi, Yami
#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct ChangeTerrainEffect {
    pub terrain_type: TerrainType,
}

impl MagicEffect for ChangeTerrainEffect {
    fn execute_effect(&self, duel: &mut Duel) {
        duel.terrain_type = self.terrain_type;
    }
}

// Dragon Capture Jar, Warrior Elimination, Eternal Rest, Stain Storm, Eradicating Aerosol, Breath of Light, Eternal Draught
#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
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

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
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
        duel.get_player_mut()
            .monster_row
            .iter_mut()
            .for_each(|monster| *monster = None);
        duel.get_player_mut()
            .spell_row
            .iter_mut()
            .for_each(|spell| *spell = None);
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct RaigekiEffect;
impl MagicEffect for RaigekiEffect {
    fn execute_effect(&self, duel: &mut Duel) {
        duel.get_enemy_mut()
            .monster_row
            .iter_mut()
            .for_each(|monster| *monster = None);
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct HarpiesFeatherDusterEffect;
impl MagicEffect for HarpiesFeatherDusterEffect {
    fn execute_effect(&self, duel: &mut Duel) {
        duel.get_enemy_mut()
            .spell_row
            .iter_mut()
            .for_each(|spell| *spell = None);
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct CrushCardEffect;
impl MagicEffect for CrushCardEffect {
    fn execute_effect(&self, duel: &mut Duel) {
        let terrain_type = duel.terrain_type.clone();
        // Set all enemy monster cards to None if their attack is 1500 or higher
        duel.get_enemy_mut()
            .monster_row
            .iter_mut()
            .for_each(|monster_row_pos| {
                if let Some(monster) = monster_row_pos {
                    if monster.card.get_stats_with_terrain(terrain_type).unwrap().0 >= 1500 {
                        *monster_row_pos = None;
                    }
                }
            });
    }
}

// Mooyan Curry (200), Red Medicine (500), Goblin's Secret Remedy (1000), Soul of the Pure (2000), Dian Keto the Cure Master (5000)
#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct LifePointHealerEffect {
    pub amount: i32,
}

impl MagicEffect for LifePointHealerEffect {
    fn execute_effect(&self, duel: &mut Duel) {
        // Iterate through all the enemy spell cards, checking if they are a trap of BadReactionToSimochi
        // If so, turn the trap to None. Then negate self.amount.
        let mut amount = self.amount;

        for spell_row_pos in duel.get_enemy_mut().spell_row.iter_mut() {
            if let Some(spell) = spell_row_pos {
                if let crate::CardVariant::Trap(trap_effect) = &spell.card.variant {
                    if let TrapEffectEnum::BadReactionToSimochi = trap_effect {
                        *spell_row_pos = None;
                        amount = -amount;
                        break;
                    }
                }
            }
        }

        duel.get_player_mut().modify_life_points(amount);
    }
}

// Sparks (50), Hinotama (100), Final Flame (500), Ookazi (500), Tremendous Fire (1000)
#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct LifePointDamagerEffect {
    pub amount: i32,
}

impl MagicEffect for LifePointDamagerEffect {
    fn execute_effect(&self, duel: &mut Duel) {
        // Check enemy spells for GoblinFan
        // If present, modify the player's life points instead of the enemy's. Also set the GoblinFan to None.
        let mut goblin_fan_activated = false;
        for spell_row_pos in duel.get_enemy_mut().spell_row.iter_mut() {
            if let Some(spell) = spell_row_pos {
                if let crate::CardVariant::Trap(trap_effect) = &spell.card.variant {
                    if let TrapEffectEnum::GoblinFan = trap_effect {
                        *spell_row_pos = None;
                        goblin_fan_activated = true;
                        break;
                    }
                }
            }
        }

        let player_to_damage = if goblin_fan_activated {
            duel.get_player_mut()
        } else {
            duel.get_enemy_mut()
        };

        player_to_damage.modify_life_points(-self.amount);
    }
}

// Spellbinding Circle (500), Shadow Spell (1000)
#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct StatReducerEffect {
    pub amount: i32,
}

impl MagicEffect for StatReducerEffect {
    fn execute_effect(&self, duel: &mut Duel) {
        // For each enemy monster card, reduce its stats by amount
        duel.get_enemy_mut()
            .monster_row
            .iter_mut()
            .for_each(|monster_row_pos| {
                if let Some(monster) = monster_row_pos {
                    monster.card.modify_stats(-self.amount);
                }
            });
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
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

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
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

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
// For each player monster card, get the base stats of the card (card.get_base_stats())
// If the base stats are higher than the current stats (card.get_stats_no_terrain()), set the current stats to the base stats.
// This can be done with card.modify_stats(delta) where delta is the difference between the current and base stats.
pub struct CursebreakerEffect;
impl MagicEffect for CursebreakerEffect {
    fn execute_effect(&self, duel: &mut Duel) {
        duel.get_player_mut()
            .monster_row
            .iter_mut()
            .for_each(|monster_row_pos| {
                if let Some(monster) = monster_row_pos {
                    let (base_attack, base_defense) = monster.card.get_base_stats().unwrap();
                    let (current_attack, current_defense) =
                        monster.card.get_stats_no_terrain().unwrap();
                    {
                        if base_attack > current_attack {
                            dbg!("CursebreakerEffect: base_attack > current_attack");
                            dbg!(&monster.card);
                            // assert that the difference between base_attack and current_attack is the same as the difference between base_defense and current_defense
                            assert_eq!(
                                base_attack - current_attack,
                                base_defense - current_defense
                            );
                            monster.card.reset_stats_to_base();
                            // monster.card.modify_stats(base_attack - current_attack);
                        }
                    }
                }
            });
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
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
#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum MagicEffectEnum {
    ChangeTerrainEffect,
    MonsterDestroyerEffect,
    DarkHoleEffect,
    RaigekiEffect,
    HarpiesFeatherDusterEffect,
    CrushCardEffect,
    LifePointHealerEffect,
    LifePointDamagerEffect,
    StatReducerEffect,
    StopDefenseEffect,
    DarkPiercingLightEffect,
    CursebreakerEffect,
    SwordsOfRevealingLightEffect,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_magic_effect() {
        dbg!("test_parse_magic_effect executing now");
        let json_data = r#"
            {
                "ChangeTerrainEffect": {
                    "terrain_type": "Forest"
                }
            }
        "#;

        let magic_effect: MagicEffectEnum = serde_json::from_str(json_data).unwrap();
        dbg!(&magic_effect);

        let json_data_2 = r#"
            {
                "RaigekiEffect": null
            }
        "#;

        let magic_effect_2: MagicEffectEnum = serde_json::from_str(json_data_2).unwrap();
        dbg!(&magic_effect_2);
    }
}
