use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    monster_terrain_relation, AdvantageRelation, GuardianStarType, MonsterType, TerrainType, CARDS, MagicEffectEnum, TrapEffectEnum,
};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Card {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub stars: u32,
    pub password: Option<u32>,
    pub attribute: u32,
    pub abc_sort: u32,
    pub max_sort: u32,
    pub atk_sort: u32,
    pub def_sort: u32,
    pub typ_sort: u32,
    pub ai_sort: u32,
    pub ai_gs: Option<u32>,
    pub fusions: HashMap<usize, usize>,
    pub variant: CardVariant,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum CardVariant {
    Monster {
        monster_type: MonsterType,
        attack: u32,
        defense: u32,
        guardian_star_a: GuardianStarType,
        guardian_star_b: GuardianStarType,
        level: u32,
    },
    Ritual {
        card1_id: usize,
        card2_id: usize,
        card3_id: usize,
        result_card_id: usize,
    },
    Equip {
        equips: Vec<usize>,
    },
    Magic(MagicEffectEnum),
    Trap(TrapEffectEnum),
}

impl Card {
    pub fn get_base_stats(&self) -> Option<(u32, u32)> {
        // we need to get_card_from_id to get the base stats of the card
        // the stats on our card may have been modified e.g. by equips
        let base_card = card_from_id(self.id);
        match &base_card.variant {
            CardVariant::Monster {
                attack, defense, ..
            } => Some((*attack, *defense)),
            _ => None,
        }
    }

    pub fn get_stats_no_terrain(&self) -> Option<(u32, u32)> {
        match &self.variant {
            CardVariant::Monster {
                attack, defense, ..
            } => Some((*attack, *defense)),
            _ => None,
        }
    }

    pub fn get_stats_with_terrain(&self, terrain_type: TerrainType) -> Option<(u32, u32)> {
        // use monster_terrain_relation to check if advantageous (+500), disadvantageous (-500), or neutral (no change)
        match &self.variant {
            CardVariant::Monster {
                attack,
                defense,
                monster_type,
                ..
            } => {
                let terrain_boost = match monster_terrain_relation(*monster_type, terrain_type) {
                    AdvantageRelation::Advantaged => 500,
                    AdvantageRelation::Disadvantaged => -500,
                    AdvantageRelation::Neutral => 0,
                };
                Some((
                    (*attack as i32 + terrain_boost).max(0) as u32,
                    (*defense as i32 + terrain_boost).max(0) as u32,
                ))
            }
            _ => None,
        }
    }

    pub fn modify_stats(&mut self, delta: i32) {
        // panic if not a Monster
        // modify attack and defense by delta, but do not let them go below 0
        match &mut self.variant {
            CardVariant::Monster {
                attack, defense, ..
            } => {
                *attack = (*attack as i32 + delta).max(0) as u32;
                *defense = (*defense as i32 + delta).max(0) as u32;
            }
            _ => panic!("Attempted to modify stats of a non-Monster card"),
        }
    }
}

pub fn card_from_id(id: usize) -> Card {
    CARDS.get(id - 1).unwrap().clone()
}

pub fn combine_cards(cards: Vec<Card>) -> Vec<Card> {
    let mut combined_cards = Vec::new();
    let mut combined_card = cards[0].clone();
    combined_cards.push(combined_card.clone());
    for card in cards.iter().skip(1) {
        combined_card = combine(&combined_card, card);
        combined_cards.push(combined_card.clone());
    }
    combined_cards
}

pub fn combine(card1: &Card, card2: &Card) -> Card {
    // First we attempt to fuse the cards. If this fails, we then attempt to equip.
    // If this fails again, and both cards are monsters, we return the second card.
    // If one of the cards is a monster but the other is not, we return whichever is the monster.
    // In any other case, we return the second card.
    use CardVariant::*;

    fuse(card1, card2)
        .or_else(|| equip(card1, card2))
        .unwrap_or_else(|| match (&card1.variant, &card2.variant) {
            (Monster { .. }, Monster { .. }) => card2.clone(),
            (Monster { .. }, _) => card1.clone(),
            (_, Monster { .. }) => card2.clone(),
            (_, _) => card2.clone(),
        })
}

pub fn fuse(card1: &Card, card2: &Card) -> Option<Card> {
    if card1.fusions.contains_key(&card2.id) {
        Some(card_from_id(card1.fusions[&card2.id]))
    } else if card2.fusions.contains_key(&card1.id) {
        Some(card_from_id(card2.fusions[&card1.id]))
    } else {
        None
    }
}

pub fn equip(card1: &Card, card2: &Card) -> Option<Card> {
    let (equip_card, monster_card) = match (&card1.variant, &card2.variant) {
        (CardVariant::Equip { .. }, CardVariant::Monster { .. }) => (card1, card2),
        (CardVariant::Monster { .. }, CardVariant::Equip { .. }) => (card2, card1),
        (_, _) => return None,
    };

    let mut monster_clone = monster_card.clone();
    if let CardVariant::Equip { equips } = &equip_card.variant {
        if equips.contains(&monster_clone.id) {
            if let CardVariant::Monster { .. } = &mut monster_clone.variant {
                let boost_amount = if equip_card.name == "Megamorph" {
                    1000
                } else {
                    500
                };
                monster_clone.modify_stats(boost_amount);
            }
        } else {
            return None;
        }
    }
    Some(monster_clone)
}

#[cfg(test)]
mod tests {}
