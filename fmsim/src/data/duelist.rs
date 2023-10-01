use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Duelist {
    pub id: u32,
    pub name: String,
    pub is_mage: u32,
    pub hand_size: u32,
    pub low_lp_threshold: u32,
    pub critical_deck_size: u32,
    pub max_fusion_length: u32,
    pub max_improve_length: u32,
    pub spell_probability: String,
    pub attack_probability: String,
    pub find_best_combo_td: u32,
    pub improve_monster_td: u32,
    pub set_magic_td: u32,
    pub find_best_combo_no_td: u32,
    pub improve_monster_no_td: u32,
    pub set_magic_no_td: u32,
    pub deck_pool: HashMap<u32, u32>,
    pub sa_pow_pool: HashMap<u32, u32>,
    pub bcd_pool: HashMap<u32, u32>,
    pub sa_tec_pool: HashMap<u32, u32>,
}
