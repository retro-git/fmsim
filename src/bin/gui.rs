#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_desktop::WindowBuilder;
use fmsim::duel::command::DuelCommand;
use fmsim::duel::command_strategy::{CommandStrategy, RandomCommandStrategy};
use fmsim::duel::field::{MonsterRowPosition, SpellRowPosition};
use fmsim::duel::state::DuelStateEnum;
use fmsim::{Card, Duel};

pub fn default_window() -> WindowBuilder {
    let builder = WindowBuilder::new();

    builder
        .with_title("fmsim")
        .with_theme(Some(dioxus_desktop::tao::window::Theme::Dark))
}

fn main() {
    let style = r#":root { color-scheme: dark; }"#;
    // make background color black
    let config = dioxus_desktop::Config::new()
        .with_custom_head(format!(
            r#"
    <style>{style}</style>
    "#
        ))
        .with_window(default_window());
    // launch the dioxus app in a webview
    // dioxus_tui::launch(App);
    dioxus_desktop::launch_cfg(App, config);
}

#[inline_props]
fn CardComponent(cx: Scope, card: Card) -> Element {
    cx.render(rsx! {
        div {
            // background_color: "green",
            justify_content: "center",
            align_items: "center",
            border: "2px solid grey",
            margin: "2px",
            div {
                format!("ID: {}", card.id)
            }
            div {
                format!("Name: {}", card.name)
            }
        }
    })
}

#[inline_props]
fn HandComponent(cx: Scope, hand: Vec<Card>) -> Element {
    cx.render(rsx! { div {
        display: "flex",
        justify_content: "center",
        align_items: "center",
        hand.iter().map(|card| {
            rsx! { CardComponent {
                card: card.clone()
            }}
        })
    }})
}

#[inline_props]
fn MonsterRowComponent(cx: Scope, monster_row: Vec<Option<MonsterRowPosition>>) -> Element {
    cx.render(rsx! { div {
        display: "flex",
        justify_content: "center",
        align_items: "center",
        monster_row.iter().map(|monster| {
            let content = match monster {
                Some(monster) => rsx! { div {
                    CardComponent {
                        card: monster.card.clone()
                    }
                }},
                None => {
                    rsx! { div {
                        "empty"
                    }}
                }
            };

            rsx! { div {
                justify_content: "center",
                align_items: "center",
                border: "2px solid grey",
                margin: "2px",
                content
            }}
        })
    }})
}

#[inline_props]
fn SpellRowComponent(cx: Scope, spell_row: Vec<Option<SpellRowPosition>>) -> Element {
    cx.render(rsx! { div {
        display: "flex",
        justify_content: "center",
        align_items: "center",
        spell_row.iter().map(|spell| {
            let content = match spell {
                Some(spell) => rsx! { div {
                    CardComponent {
                        card: spell.card.clone()
                    }
                }},
                None => {
                    rsx! { div {
                        "empty"
                    }}
                }
            };

            rsx! { div {
                justify_content: "center",
                align_items: "center",
                border: "2px solid grey",
                margin: "2px",
                content
            }}
        })
    }})
}

fn DuelComponent(cx: Scope) -> Element {
    let duel = use_shared_state::<Duel>(cx).unwrap();
    cx.render(rsx! {
        div {
            SpellRowComponent {
                spell_row: duel.read().get_enemy().spell_row.clone()
            }
            MonsterRowComponent {
                monster_row: duel.read().get_enemy().monster_row.clone()
            }
            MonsterRowComponent {
                monster_row: duel.read().get_player().monster_row.clone()
            }
            SpellRowComponent {
                spell_row: duel.read().get_player().spell_row.clone()
            }
            HandComponent {
                hand: duel.read().get_player().hand.clone()
            }
            button {
                onclick: move |_| {
                    if !matches!(duel.read().state, DuelStateEnum::EndState(_)) {
                        let strategy = RandomCommandStrategy;
                        let command = strategy.get_command(&duel.read());
                        let mut duel_ref = duel.write();
                        command.execute(&mut *duel_ref).unwrap();
                    }
                }
            }
        }
    })
}

fn App(cx: Scope) -> Element {
    let duel = fmsim::Duel::default();
    use_shared_state_provider(cx, || duel);

    cx.render(rsx! { DuelComponent {}})
}
