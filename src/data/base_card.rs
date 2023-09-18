use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn from_primitive<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: num_traits::FromPrimitive,
{
    let i = i32::deserialize(deserializer)?;
    T::from_i32(i).ok_or_else(|| serde::de::Error::custom("Out of range"))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseCard {
    pub id: i32,
    pub name: String,
    pub description: String,
    #[serde(deserialize_with = "from_primitive")]
    pub guardian_star_a: GuardianStar,
    #[serde(deserialize_with = "from_primitive")]
    pub guardian_star_b: GuardianStar,
    pub level: i32,
    #[serde(rename = "type")]
    #[serde(deserialize_with = "from_primitive")]
    pub card_type: CardType,
    pub base_attack: i32,
    pub base_defense: i32,
    pub stars: i32,
    pub card_code: i32,
    pub attribute: i32,
    pub name_color: i32,
    pub desc_color: i32,
    pub abc_sort: i32,
    pub max_sort: i32,
    pub atk_sort: i32,
    pub def_sort: i32,
    pub typ_sort: i32,
    pub ai_sort: i32,
    pub ai_gs: Option<i32>,
    pub fusions: HashMap<i32, i32>,
    pub equips: Vec<i32>,
    pub ritual: Option<Ritual>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ritual {
    pub card1_id: i32,
    pub card2_id: i32,
    pub card3_id: i32,
    pub result_card_id: i32,
}

#[derive(Serialize, Deserialize, Debug, FromPrimitive, ToPrimitive)]
pub enum CardType {
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
    Magic = 20,
    Trap = 21,
    Ritual = 22,
    Equip = 23,
}

#[derive(Serialize, Deserialize, Debug, FromPrimitive, ToPrimitive)]
pub enum GuardianStar {
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
pub enum FieldType {
    Neutral = 0,
    Forest = 1,
    Mountain = 2,
    Sogen = 3,
    Umi = 4,
    Wasteland = 5,
    Yami = 6,
}
