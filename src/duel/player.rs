use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::Card;

use super::{deck::generate_random_deck, field::{MonsterRowPosition, SpellRowPosition}};

#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
#[builder(setter(into), default)]
pub struct Player {
    pub life_points: i32,
    pub deck: Vec<Card>,
    pub hand: Vec<Card>,
    pub hand_size: usize,
    pub monster_row: Vec<Option<MonsterRowPosition>>,
    pub spell_row: Vec<Option<SpellRowPosition>>,
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
        // Draw cards until the hand has hand_size cards, or until the deck is empty.
        // Note that after this is done, if the hand does not have at least 5 cards, the player loses by deck out (the caller will check for this)
        while self.hand.len() < self.hand_size && !self.deck.is_empty() {
            let card = self.deck.pop().unwrap();
            self.hand.push(card);
        }
    }
}