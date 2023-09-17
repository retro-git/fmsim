mod models;
mod schema;

use std::sync::LazyLock;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use models::*;
use schema::*;

macro_rules! lazy_load {
    ($name:ident, $type:ty, $table:expr) => {
        static $name: LazyLock<Vec<$type>> = LazyLock::new(|| {
            let mut conn = SqliteConnection::establish("data/fm.db").unwrap();
            $table.load::<$type>(&mut conn).expect(concat!("Error loading ", stringify!($name)))
        });
    };
}

lazy_load!(CARDS, Card, cards::table);
lazy_load!(FUSIONS, Fusion, fusions::table);
lazy_load!(CARD_TYPES, CardType, card_types::table);

pub fn get_card_by_id(id: usize) -> Option<&'static Card> {
    CARDS.get(id - 1)
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(card_id))]
pub struct Card {
    pub card_id: i32,
    pub name: String,
    pub description: String,
    pub guardian_star_a_id: i32,
    pub guardian_star_b_id: i32,
    pub level: i32,
    pub type_id: i32,
    pub attack: i32,
    pub defense: i32,
    pub stars: i32,
    pub card_code: i32,
    pub attribute: i32,
    pub name_color: i32,
    pub desc_color: i32,
    pub abc_sort: i32,
    pub max_sort: i32,
    pub atk_sort: i32,
    pub def_sort: i32,
    pub typ_sort: i32,
    pub ai_sort: i32,
    pub ai_gs: Option<i32>,
}

static CARDS_RUSQLITE: LazyLock<Vec<Card>> = LazyLock::new(|| {
    let conn = rusqlite::Connection::open("data/fm.db").unwrap();
    let mut stmt = conn.prepare("SELECT * FROM cards").unwrap();
    let mut cards = stmt.query_map([], |row| {
        Ok(Card {
            id: row.get(0)?,
            name: row.get(1)?,
            // Continue for all fields of Card
            
        })
    }).unwrap();
    cards.collect().unwrap()
    
});