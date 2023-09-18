use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Duelist {
    pub id: i32,
    pub name: String,
    pub is_mage: i32,
    pub hand_size: i32,
    pub low_lp_threshold: i32,
    pub critical_deck_size: i32,
    pub max_fusion_length: i32,
    pub max_improve_length: i32,
    pub spell_probability: String,
    pub attack_probability: String,
    pub find_best_combo_td: i32,
    pub improve_monster_td: i32,
    pub set_magic_td: i32,
    pub find_best_combo_no_td: i32,
    pub improve_monster_no_td: i32,
    pub set_magic_no_td: i32,
    pub deck_pool: HashMap<i32, i32>,
    pub sa_pow_pool: HashMap<i32, i32>,
    pub bcd_pool: HashMap<i32, i32>,
    pub sa_tec_pool: HashMap<i32, i32>,
}
