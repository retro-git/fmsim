use derive_builder::Builder;
use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

use self::player::Player;
use self::state::{DuelStateEnum, HandState};

pub mod command;
pub mod command_builder;
pub mod deck;
pub mod field;
pub mod player;
pub mod state;

#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
#[builder(setter(into), default)]
pub struct Duel {
    player1: Player,
    player2: Player,
    field_type: FieldType,
    turn: u32,
    state: DuelStateEnum,
}

impl Default for Duel {
    fn default() -> Self {
        let mut duel = Self {
            player1: Player::default(),
            player2: Player::default(),
            field_type: FieldType::Neutral,
            turn: 0,
            state: HandState.into(),
        };
        duel.get_player_mut().draw();
        duel
    }
}

impl Duel {
    fn get_player(&self) -> &Player {
        if self.turn % 2 == 0 {
            &self.player1
        } else {
            &self.player2
        }
    }

    fn get_enemy(&self) -> &Player {
        if self.turn % 2 == 0 {
            &self.player2
        } else {
            &self.player1
        }
    }

    fn get_player_mut(&mut self) -> &mut Player {
        if self.turn % 2 == 0 {
            &mut self.player1
        } else {
            &mut self.player2
        }
    }

    fn get_enemy_mut(&mut self) -> &mut Player {
        if self.turn % 2 == 0 {
            &mut self.player2
        } else {
            &mut self.player1
        }
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
        duel.get_player_mut().life_points -= 1000;
        assert_eq!(duel.get_player().life_points, 7000);
        duel.turn += 1;
        assert_eq!(duel.get_player().life_points, 8000);
    }
}