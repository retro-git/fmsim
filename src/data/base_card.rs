use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn from_primitive<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: num_traits::FromPrimitive,
{
    let i = u32::deserialize(deserializer)?;
    T::from_u32(i).ok_or_else(|| serde::de::Error::custom("Out of range"))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseCard {
    pub id: u32,
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
    pub fusions: HashMap<u32, u32>,
    pub variant: CardVariant,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum CardVariant {
    Monster {
        #[serde(deserialize_with = "from_primitive")]
        monster_type: MonsterType,
        base_attack: u32,
        base_defense: u32,
        #[serde(deserialize_with = "from_primitive")]
        guardian_star_a: GuardianStarType,
        #[serde(deserialize_with = "from_primitive")]
        guardian_star_b: GuardianStarType,
        level: u32,
    },
    Ritual {
        card1_id: u32,
        card2_id: u32,
        card3_id: u32,
        result_card_id: u32,
    },
    Equip {
        equips: Vec<u32>,
    },
    Magic,
    Trap
}

#[derive(Serialize, Deserialize, Debug, FromPrimitive, ToPrimitive)]
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

#[derive(Serialize, Deserialize, Debug, FromPrimitive, ToPrimitive)]
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

// #[derive(Serialize, Deserialize, Debug)]
// pub struct BaseCard {
//     pub id: u32,
//     pub name: String,
//     pub description: String,
//     #[serde(deserialize_with = "from_primitive")]
//     pub guardian_star_a: GuardianStar,
//     #[serde(deserialize_with = "from_primitive")]
//     pub guardian_star_b: GuardianStar,
//     pub level: u32,
//     #[serde(rename = "type")]
//     #[serde(deserialize_with = "from_primitive")]
//     pub card_type: CardType,
//     pub base_attack: u32,
//     pub base_defense: u32,
//     pub stars: u32,
//     pub card_code: u32,
//     pub attribute: u32,
//     pub name_color: u32,
//     pub desc_color: u32,
//     pub abc_sort: u32,
//     pub max_sort: u32,
//     pub atk_sort: u32,
//     pub def_sort: u32,
//     pub typ_sort: u32,
//     pub ai_sort: u32,
//     pub ai_gs: Option<u32>,
//     pub fusions: HashMap<u32, u32>,
//     pub equips: Vec<u32>,
//     pub ritual: Option<Ritual>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Ritual {
//     pub card1_id: u32,
//     pub card2_id: u32,
//     pub card3_id: u32,
//     pub result_card_id: u32,
// }

// #[derive(Serialize, Deserialize, Debug, FromPrimitive, ToPrimitive)]
// pub enum CardType {
//     Dragon = 0,
//     Spellcaster = 1,
//     Zombie = 2,
//     Warrior = 3,
//     BeastWarrior = 4,
//     Beast = 5,
//     WingedBeast = 6,
//     Fiend = 7,
//     Fairy = 8,
//     Insect = 9,
//     Dinosaur = 10,
//     Reptile = 11,
//     Fish = 12,
//     SeaSerpent = 13,
//     Machine = 14,
//     Thunder = 15,
//     Aqua = 16,
//     Pyro = 17,
//     Rock = 18,
//     Plant = 19,
//     Magic = 20,
//     Trap = 21,
//     Ritual = 22,
//     Equip = 23,
// }

// #[derive(Serialize, Deserialize, Debug, FromPrimitive, ToPrimitive)]
// pub enum FieldType {
//     Neutral = 0,
//     Forest = 1,
//     Mountain = 2,
//     Sogen = 3,
//     Umi = 4,
//     Wasteland = 5,
//     Yami = 6,
// }
