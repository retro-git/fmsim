#![allow(non_snake_case)]

use std::thread;

use dioxus::prelude::*;
use dioxus_desktop::WindowBuilder;
use fmsim::duel::command::DuelCommand;
use fmsim::duel::command_strategy::{CommandStrategy, RandomCommandStrategy};
use fmsim::duel::field::{MonsterRowPosition, SpellRowPosition};
use fmsim::duel::state::DuelStateEnum;
use fmsim::{Card, Duel};

fn main() {}

#[test]
fn test() {
    let mut handles = vec![];

    for _ in 1..=50000 {
        let handle = thread::spawn(|| {
            let mut duel = fmsim::Duel::default();
            let strategy = RandomCommandStrategy;

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                while !matches!(duel.state, DuelStateEnum::EndState(_)) {
                    let command = strategy.get_command(&duel);
                    command.execute(&mut duel).unwrap();
                }
            }));

            if result.is_err() {
                // dbg print the duel state if there is a crash
                println!("{:?}", duel.state);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert!(true);
}
