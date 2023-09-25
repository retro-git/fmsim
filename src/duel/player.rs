use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::Card;

use super::deck::generate_random_deck;

#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
#[builder(setter(into), default)]
pub struct Player {
    pub life_points: i32,
    pub deck: Vec<Card>,
    pub hand: Vec<Card>,
    pub hand_size: usize,
    pub monster_row: Vec<Option<Card>>,
    pub spell_row: Vec<Option<Card>>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            life_points: 8000,
            deck: generate_random_deck(),
            hand: Vec::new(),
            hand_size: 5,
            monster_row: vec![None; 5],
            spell_row: vec![None; 5],
        }
    }
}

impl Player {
    pub fn draw(&mut self) {
        for _ in 0..self.hand_size {
            self.hand.push(self.deck.pop().unwrap());
        }
    }
}
