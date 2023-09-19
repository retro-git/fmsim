use num_derive::{ToPrimitive, FromPrimitive};
use serde::{Serialize, Deserialize};

use crate::Card;

#[derive(Serialize, Deserialize, Debug, FromPrimitive, ToPrimitive)]
pub enum FieldType {
    Neutral = 0,
    Forest = 1,
    Mountain = 2,
    Sogen = 3,
    Umi = 4,
    Wasteland = 5,
    Yami = 6,
}

struct Player {
    life_points: i32,
    deck: Vec<Card>,
    hand: Vec<Card>,
}

struct Duel {
    players: [Player; 2],
    field: [[Option<Card>; 5]; 4],
    field_type: FieldType,
}
