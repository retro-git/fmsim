#![allow(non_snake_case)]

use std::fs::File;
use std::io::BufReader;
use std::thread;

use dioxus::prelude::*;
use dioxus_desktop::WindowBuilder;
use fmsim::duel::command::{DuelCommand, DuelCommandEnum};
use fmsim::duel::command_strategy::{CommandStrategy, RandomCommandStrategy};
use fmsim::duel::field::{MonsterRowPosition, SpellRowPosition};
use fmsim::duel::state::DuelStateEnum;
use fmsim::{Card, Duel, card_from_name};
use serde::{Deserialize, Serialize};

// derive serde
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Crash {
    starting_duel_state: Duel,
    commands_list: Vec<DuelCommandEnum>,
}

fn main() {
    // load crashes/crash.json
    // it contains two fields: starting_duel_state and commands_list
    // parse the json, and then iterate over the commands_list, executing them all on the duel
    let file = File::open("crashes/crash.json").expect("Unable to open file");
    let reader = BufReader::new(file);
    let crash_data: Crash = serde_json::from_reader(reader).expect("Unable to parse json");

    let mut duel = Duel::from(crash_data.starting_duel_state);

    // // using get_card_from_Name and combine_cards, print the result of Kaiser Dragon + Crimson Sunbird + Spirit of the Mountain + Metal Guardian + Machine Conversion Factory
    // let kaiser_dragon = card_from_name("Kaiser Dragon");
    // let crimson_sunbird = card_from_name("Crimson Sunbird");
    // let spirit_of_the_mountain = card_from_name("Spirit of the Mountain");
    // let metal_guardian = card_from_name("Metal Guardian");
    // let machine_conversion_factory = card_from_name("Machine Conversion Factory");

    // // put them all in an array in order
    // let cards = vec![
    //     kaiser_dragon,
    //     crimson_sunbird,
    //     spirit_of_the_mountain,
    //     metal_guardian,
    //     machine_conversion_factory,
    // ];

    // // combine them
    // let combined_card = fmsim::combine_cards(cards);
    // //combined card is a list of 3-tuples. loop through each tuple, printing the name of each card in the tuple
    // for (card1, card2, card3) in &combined_card {
    //     println!("{} {} {}", card1.name, card2.name, card3.name);
    // }
    // // print the result, which is lsat.unwrap.2
    // println!("{}", combined_card.last().unwrap().2.name);

    for (i, command) in crash_data.commands_list.iter().enumerate() {
        println!("Iteration: {}", i);
        // print executing command and the command debug info
        // print  duel state
        println!("Duel state: {:?}", duel.state);
        // print list of cards in hand by name
        println!(
            "Player hand: {:?}",
            duel.get_player().hand.iter().map(|card| card.name.clone()).collect::<Vec<_>>()
        );
        //print enemy hand
        println!(
            "Enemy hand: {:?}",
            duel.get_enemy().hand.iter().map(|card| card.name.clone()).collect::<Vec<_>>()
        );
        // print all the cards on the enemy monster row by name
        println!(
            "Player monster row: {:?}",
            duel.get_player().monster_row.clone().iter().map(|card_pos| card_pos.as_ref().map(|cp| cp.card.name.clone())).collect::<Vec<_>>()
        );
        println!("Executing command: {:?}", command);
        
        command.execute(&mut duel).unwrap();
        // use random command strategy to get a command
        // let strategy = RandomCommandStrategy;
        // let _ = strategy.get_command(&duel);
    }
}

#[test]
fn test() {
    let mut handles = vec![];

    for _ in 1..=50000 {
        let handle = thread::spawn(|| {
            let mut duel = fmsim::Duel::default();
            let starting_duel_state = duel.clone();

            let strategy = RandomCommandStrategy;

            let mut commands_list = Vec::new();

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                while !matches!(duel.state, DuelStateEnum::EndState(_)) {
                    let command = strategy.get_command(&duel);
                    commands_list.push(command.clone());
                    command.execute(&mut duel).unwrap();
                }
            }));

            if result.is_err() {
                // dbg print the duel state if there is a crash
                println!("{:?}", duel.state);
                // print the name of each card in the hand of the current player
                println!(
                    "{:?}",
                    duel.get_player().hand.iter().map(|card| card.name.clone()).collect::<Vec<_>>()
                );
                // serialise the starting duel state and commands_list to json. then write to a file.
                // the filename should be something unique, like the current timestamp.
                // the file should be written to a folder called "crashes" in the root of the project.
                // the file should be named "crash-<timestamp>.json"
                // if the folder doesn't exist, create it.
                use std::fs::File;
                use std::io::Write;
                use std::path::Path;
                use serde_json::json;

                let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
                let filename = format!("crash-{}.json", timestamp);
                let path = Path::new("crashes");

                std::fs::create_dir_all(&path).unwrap();

                let mut file = File::create(path.join("crash.json")).unwrap();
                let data = json!({
                    "starting_duel_state": starting_duel_state,
                    "commands_list": commands_list
                });

                file.write_all(data.to_string().as_bytes()).unwrap();

                // exit the entire program
                std::process::exit(1);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert!(true);
}
