use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::TerrainType;

use self::command::DuelCommandEnum;
use self::player::Player;
use self::state::{DuelStateEnum, HandState};

pub mod command;
pub mod command_builder;
pub mod command_strategy;
pub mod deck;
pub mod field;
pub mod player;
pub mod state;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Builder)]
#[builder(setter(into), default)]
pub struct Duel {
    pub player1: Player,
    pub player2: Player,
    pub terrain_type: TerrainType,
    pub turn: u32,
    pub state: DuelStateEnum,
}

impl Default for Duel {
    fn default() -> Self {
        let mut duel = Self {
            player1: Player::default(),
            player2: Player::default(),
            terrain_type: TerrainType::Default,
            turn: 0,
            state: HandState.into(),
        };
        duel.get_player_mut().draw();
        duel
    }
}

impl Duel {
    pub fn command_builder(&self) -> command_builder::CommandBuilder<command_builder::Start> {
        command_builder::CommandBuilder::new(self)
    }

    pub fn generate_all_valid_commands(&self) -> Vec<DuelCommandEnum> {
        DuelCommandEnum::generate_all_valid(self)
    }

    pub fn get_player(&self) -> &Player {
        if self.turn % 2 == 0 {
            &self.player1
        } else {
            &self.player2
        }
    }

    pub fn get_enemy(&self) -> &Player {
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
