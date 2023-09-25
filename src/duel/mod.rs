use derive_builder::Builder;
use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

use crate::Card;

use self::command::DuelCommand;
use self::player::Player;

pub mod command;
pub mod deck;
pub mod player;
pub mod field;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DuelState {
    Hand, // The user is selecting a card, or multiple cards, from the hand.
    // HandPlaySingle { card: Card }, // The user selected a single card to play from the hand. Now awaiting further info depending on the card type (e.g. face up/down, field position).
    Field, // The hand phase is done and the user can perform field actions (such as attacking, toggling between attack/defense, etc.)
    // FieldPlayEquip { position: usize }, // The user selected an equip card on the spell row. Now awaiting them to pick the monster to equip to.
    SetGuardianStar { card: Card, field_index: usize }, // Happens when a monster is played from the hand, or when an equip is played on an existing monster but the equip fails.
    End,                                                // The game is over.
}

#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
#[builder(setter(into), default)]
pub struct Duel {
    player1: Player,
    player2: Player,
    field_type: FieldType,
    turn: u32,
    state: DuelState,
}

impl Default for Duel {
    fn default() -> Self {
        Self {
            player1: Player::default(),
            player2: Player::default(),
            field_type: FieldType::Neutral,
            turn: 0,
            state: DuelState::Hand,
        }
    }
}

impl Duel {
    fn get_player(&mut self) -> &mut Player {
        if self.turn % 2 == 0 {
            &mut self.player1
        } else {
            &mut self.player2
        }
    }

    fn get_opponent(&mut self) -> &mut Player {
        if self.turn % 2 == 0 {
            &mut self.player2
        } else {
            &mut self.player1
        }
    }

    fn execute_command(&mut self, command: command::DuelCommandEnum) {
        command.execute(self);
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum FieldType {
    Neutral = 0,
    Forest = 1,
    Mountain = 2,
    Sogen = 3,
    Umi = 4,
    Wasteland = 5,
    Yami = 6,
}

#[cfg(test)]
mod tests {
    use crate::duel::Duel;

    #[test]
    fn test_duel_turns() {
        let mut duel = Duel::default();

        assert_eq!(duel.get_player().life_points, 8000);
        duel.get_player().life_points -= 1000;
        assert_eq!(duel.get_player().life_points, 7000);
        duel.turn += 1;
        assert_eq!(duel.get_player().life_points, 8000);
    }
}
