extern crate diesel;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

// Include the schema
mod schema;
use schema::card_types;
use schema::cards; // Add this line
use schema::fusions; // Add this line

#[derive(Queryable, Debug)]
struct CardType {
    type_id: i32,
    type_: String,
    is_monster: i32,
}

#[derive(Queryable, Debug)] // Add this struct
struct Card {
    card_id: i32,
    name: String,
    description: String,
    guardian_star_a_id: i32,
    guardian_star_b_id: i32,
    level: i32,
    type_id: i32,
    attack: i32,
    defense: i32,
    stars: i32,
    card_code: i32,
    attribute: i32,
    name_color: i32,
    desc_color: i32,
    abc_sort: i32,
    max_sort: i32,
    atk_sort: i32,
    def_sort: i32,
    typ_sort: i32,
    ai_sort: i32,
    ai_gs: Option<i32>,
}

#[derive(Queryable, Debug, Clone)]
struct Fusion {
    fusion_id: i32,
    card1_id: i32,
    card2_id: i32,
    result_card_id: i32,
}

fn get_fusions_for_card(card_id: i32, fusions: &Vec<Fusion>) -> Vec<Fusion> {
    fusions
        .iter()
        .filter(|fusion| fusion.card1_id == card_id || fusion.card2_id == card_id)
        .cloned()
        .collect()
}

fn get_fusions_for_card2(card_id: i32, connection: &mut SqliteConnection) -> QueryResult<Vec<Fusion>> {
    use schema::fusions::dsl::*;
    fusions.filter(card1_id.eq(card_id).or(card2_id.eq(card_id))).load(connection)
}

fn main() {

    let database_url = "data/fm.db";
    let mut connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    use schema::card_types::dsl::*;
    use schema::cards::dsl::*;
    use schema::fusions::dsl::*; // Add this line

    let results = card_types.load::<CardType>(&mut connection)
        .expect("Error loading CardTypes");

    println!("Displaying {} card types", results.len());
    for card_type in results {
        println!("{:?}", card_type);
    }

    let results = cards.load::<Card>(&mut connection)
        .expect("Error loading Cards");

    println!("Displaying {} cards", results.len());
    for card in results {
        println!("{:?}", card);
    }

    let fusions_vec = fusions.load::<Fusion>(&mut connection)
        .expect("Error loading Fusions");

    let td_card_id = 425;
    let fusions_for_card = get_fusions_for_card(td_card_id, &fusions_vec);
    println!("Displaying fusions for card with id {}", td_card_id);
    for fusion in fusions_for_card {
        println!("{:?}", fusion);
    }
    
}
