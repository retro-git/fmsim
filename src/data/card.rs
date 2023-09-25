use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::card_from_id;

fn from_primitive<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: num_traits::FromPrimitive,
{
    let i = u32::deserialize(deserializer)?;
    T::from_u32(i).ok_or_else(|| serde::de::Error::custom("Out of range"))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub stars: u32,
    pub card_code: u32,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

pub fn combine(card1: &mut Card, card2: &mut Card) -> Card {
    // If both cards are monsters, attempt to fuse them. We do this by checking if the card1's ID is in card2's fusions, and vice versa.
    // If we find a match, we fuse them and return the result. Otherwise, we return card2.

    // If one card is a monster and the other is a magic/trap/ritual, we return the monster.
    
    // If one card is a monster and the other is an equip, we attempt to apply the equip.
    // This is done by checking if the equip card has the monster's ID in its equips list.
    // If so, we return the monster card with its attack boosted by 500. UNLESS the equip card is Megamorph, in which case the attack is increased by 1000.

    // If both cards are magic/trap/ritual/equip, we return card2.

    match (&mut card1.variant, &mut card2.variant) {
        (CardVariant::Monster { .. }, CardVariant::Monster { .. }) => {
            if card1.fusions.contains_key(&card2.id) {
                card_from_id(card1.fusions[&card2.id])
            } else if card2.fusions.contains_key(&card1.id) {
                card_from_id(card2.fusions[&card1.id])
            } else {
                card2.clone()
            }
        }

        (CardVariant::Equip { equips: card1_equips }, CardVariant::Monster { attack, .. }) => {
            if card1_equips.contains(&card2.id) {
                *attack += 500;
            }
            card2.clone()
        }

        (CardVariant::Monster { attack, .. }, CardVariant::Equip { equips: card2_equips }) => {
            if card2_equips.contains(&card1.id) {
                *attack += 500;
            }
            card1.clone()
        }

        (CardVariant::Monster { .. }, _) => card1.clone(),
        (_, CardVariant::Monster { .. }) => card2.clone(),

        (_, _) => card2.clone(),
    }
}

#[derive(Serialize, Deserialize, Debug, FromPrimitive, ToPrimitive, Copy, Clone)]
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

#[derive(Serialize, Deserialize, Debug, FromPrimitive, ToPrimitive, Copy, Clone)]
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