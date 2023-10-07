use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::CARDS;

fn from_primitive<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: num_traits::FromPrimitive,
{
    let i = u32::deserialize(deserializer)?;
    T::from_u32(i).ok_or_else(|| serde::de::Error::custom("Out of range"))
}

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
#[serde(tag = "type")]
pub enum CardVariant {
    Monster {
        #[serde(deserialize_with = "from_primitive")]
        monster_type: MonsterType,
        attack: u32,
        defense: u32,
        #[serde(deserialize_with = "from_primitive")]
        guardian_star_a: GuardianStarType,
        #[serde(deserialize_with = "from_primitive")]
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
    Magic,
    Trap,
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
            if let CardVariant::Monster {
                attack, defense, ..
            } = &mut monster_clone.variant
            {
                let boost_amount = if equip_card.name == "Megamorph" {
                    1000
                } else {
                    500
                };
                *attack += boost_amount;
                *defense += boost_amount;
            }
        } else {
            return None;
        }
    }
    Some(monster_clone)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, FromPrimitive, ToPrimitive, Copy, Clone)]
pub enum GuardianStarType {
    NoStar = 0,
    Mars = 1,
    Jupiter = 2,
    Saturn = 3,
    Uranus = 4,
    Pluto = 5,
    Neptune = 6,
    Mercury = 7,
    Sun = 8,
    Moon = 9,
    Venus = 10,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, FromPrimitive, ToPrimitive, Copy, Clone)]
pub enum MonsterType {
    Dragon = 0,
    Spellcaster = 1,
    Zombie = 2,
    Warrior = 3,
    BeastWarrior = 4,
    Beast = 5,
    WingedBeast = 6,
    Fiend = 7,
    Fairy = 8,
    Insect = 9,
    Dinosaur = 10,
    Reptile = 11,
    Fish = 12,
    SeaSerpent = 13,
    Machine = 14,
    Thunder = 15,
    Aqua = 16,
    Pyro = 17,
    Rock = 18,
    Plant = 19,
}

#[cfg(test)]
mod tests {}
