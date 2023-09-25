use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{combine_cards, Card};

use super::{
    deck::generate_random_deck,
    field::{MonsterRowPosition, SpellRowPosition},
};

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

    pub fn play_hand(&mut self, hand_indices: &Vec<usize>, field_index: usize) -> Card {
        // take all the cards in the hand at the given indices.
        // order them by the same order as the indices.
        // then call combine_cards to combine them into a single card.

        // check monster_row[field_pos]. if there is already a monster there, take it and append it to the start of cards.
        let existing_position = self.monster_row[field_index].take();

        let mut cards = Vec::new();
        if let Some(monster) = existing_position {
            cards.push(monster.card);
        }
        for index in hand_indices {
            cards.push(self.hand.remove(*index));
        }

        combine_cards(cards)
    }
}
