// struct ActiveCard {
//     id: u32,
//     attack: u32,
//     defense: u32,
// }

enum ActiveCard {
    Monster { id: u32, attack: u32, defense: u32 },
    Magic { id: u32 },
    Trap { id: u32 },
    Ritual { id: u32 },
}

struct Player {
    life_points: i32,
    deck: Vec<u32>,
    hand: Vec<ActiveCard>,
}

struct Duel {}
