use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, FromPrimitive, ToPrimitive, Copy, Clone)]
pub enum MonsterType {
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
}
