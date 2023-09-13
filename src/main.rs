extern crate diesel;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

// Include the schema
mod schema;
use schema::CardTypes;
use schema::Cards; // Add this line
use schema::Fusions; // Add this line

#[derive(Queryable, Debug)]
struct CardType {
    TypeID: i32,
    Type: String,
    IsMonster: i32,
}

#[derive(Queryable, Debug)] // Add this struct
struct Card {
    CardID: i32,
    Name: String,
    Description: String,
    GuardianStarAID: i32,
    GuardianStarBID: i32,
    Level: i32,
    TypeID: i32,
    Attack: i32,
    Defense: i32,
    Stars: i32,
    CardCode: i32,
    Attribute: i32,
    NameColor: i32,
    DescColor: i32,
    AbcSort: i32,
    MaxSort: i32,
    AtkSort: i32,
    DefSort: i32,
    TypSort: i32,
    AISort: i32,
    AiGs: Option<i32>,
}

#[derive(Queryable, Debug, Clone)]
struct Fusion {
    FusionID: i32,
    Card1ID: i32,
    Card2ID: i32,
    ResultCardID: i32,
}

fn get_fusions_for_card(card_id: i32, fusions: &Vec<Fusion>) -> Vec<Fusion> {
    fusions
        .iter()
        .filter(|fusion| fusion.Card1ID == card_id || fusion.Card2ID == card_id)
        .cloned()
        .collect()
}

fn main() {

    let database_url = "data/FMDatabase.db";
    let mut connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    use schema::CardTypes::dsl::*;
    use schema::Cards::dsl::*;
    use schema::Fusions::dsl::*; // Add this line

    let results = CardTypes.load::<CardType>(&mut connection)
        .expect("Error loading CardTypes");

    println!("Displaying {} card types", results.len());
    for card_type in results {
        println!("{:?}", card_type);
    }

    let results = Cards.load::<Card>(&mut connection)
        .expect("Error loading Cards");

    println!("Displaying {} cards", results.len());
    for card in results {
        println!("{:?}", card);
    }

    let fusions = Fusions.load::<Fusion>(&mut connection)
        .expect("Error loading Fusions");

    let card_id = 425;
    let fusions_for_card = get_fusions_for_card(card_id, &fusions);
    println!("Displaying fusions for card with id {}", card_id);
    for fusion in fusions_for_card {
        println!("{:?}", fusion);
    }
    
}
