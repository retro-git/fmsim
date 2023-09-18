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

#[derive(Serialize, Deserialize, Debug, FromPrimitive, ToPrimitive)]
pub enum CardTypes {
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
pub enum GuardianStars {
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

#[derive(Serialize, Deserialize, Debug)]
struct Card {
    id: i32,
    name: String,
    description: String,
    #[serde(deserialize_with = "from_primitive")]
    guardian_star_a: GuardianStars,
    #[serde(deserialize_with = "from_primitive")]
    guardian_star_b: GuardianStars,
    level: i32,
    #[serde(rename = "type")]
    #[serde(deserialize_with = "from_primitive")]
    card_type: CardTypes,
    base_attack: i32,
    base_defense: i32,
    stars: i32,
    card_code: i32,
    attribute: i32,
    name_color: i32,
    desc_color: i32,
    abc_sort: i32,
    max_sort: i32,
    atk_sort: i32,
    def_sort: i32,
    typ_sort: i32,
    ai_sort: i32,
    ai_gs: Option<i32>,
    fusions: HashMap<i32, i32>,
    equips: Vec<i32>,
    ritual: Option<Ritual>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Ritual {
    card1_id: i32,
    card2_id: i32,
    card3_id: i32,
    result_card_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Duelist {
    id: i32,
    name: String,
    is_mage: i32,
    hand_size: i32,
    low_lp_threshold: i32,
    critical_deck_size: i32,
    max_fusion_length: i32,
    max_improve_length: i32,
    spell_probability: String,
    attack_probability: String,
    find_best_combo_td: i32,
    improve_monster_td: i32,
    set_magic_td: i32,
    find_best_combo_no_td: i32,
    improve_monster_no_td: i32,
    set_magic_no_td: i32,
    deck_pool: HashMap<i32, i32>,
    sa_pow_pool: HashMap<i32, i32>,
    bcd_pool: HashMap<i32, i32>,
    sa_tec_pool: HashMap<i32, i32>,
}

fn main() {
    // use include_bytes! to load the JSON data at compile time
    let card_data = include_bytes!("../data/cards.json");
    let duelist_data = include_bytes!("../data/duelists.json");

    // use serde to parse all cards into a vector
    let cards: Vec<Card> = serde_json::from_slice(card_data).expect("Error while reading cards");

    // use serde to parse all duelists into a vector
    let duelists: Vec<Duelist> =
        serde_json::from_slice(duelist_data).expect("Error while reading duelists");

    // print all cards
    for card in &cards {
        println!("{:?}\n", card);
    }

    // print all duelists
    for duelist in &duelists {
        println!("{:?}\n", duelist);
    }
}
