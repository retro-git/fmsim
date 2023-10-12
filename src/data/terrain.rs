use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

use crate::{AdvantageRelation, MonsterType};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum TerrainType {
    Default = 0,
    Forest = 1,    // Forest
    Wasteland = 2, // Wasteland
    Mountain = 3,  // Mountain
    Meadow = 4,    // Sogen
    Sea = 5,       // Umi
    Dark = 6,      // Yami
}

pub fn monster_terrain_relation(
    monster_type: MonsterType,
    terrain_type: TerrainType,
) -> AdvantageRelation {
    use AdvantageRelation::*;
    use MonsterType::*;
    use TerrainType::*;
    match monster_type {
        BeastWarrior | Insect | Plant | Beast if terrain_type == Forest => Advantaged,
        Dragon | WingedBeast | Thunder if terrain_type == Mountain => Advantaged,
        Warrior | BeastWarrior if terrain_type == Meadow => Advantaged,
        Aqua | Thunder if terrain_type == Sea => Advantaged,
        Zombie | Dinosaur | Rock if terrain_type == Wasteland => Advantaged,
        Spellcaster | Fiend if terrain_type == Dark => Advantaged,
        Machine | Pyro if terrain_type == Sea => Disadvantaged,
        Fairy if terrain_type == Dark => Disadvantaged,
        _ => Neutral,
    }
}
