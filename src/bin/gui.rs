#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_desktop::{LogicalSize, WindowBuilder};
use fmsim::duel::command::DuelCommand;
use fmsim::duel::command_strategy::{CommandStrategy, RandomCommandStrategy};
use fmsim::duel::field::{MonsterRowPosition, SpellRowPosition};
use fmsim::duel::state::DuelStateEnum;
use fmsim::{Card, Duel};
use fmsim::duel::PlayerEnum;

pub fn default_window() -> WindowBuilder {
    let builder = WindowBuilder::new();

    builder
        .with_title("fmsim")
        .with_theme(Some(dioxus_desktop::tao::window::Theme::Dark))
        .with_inner_size(LogicalSize::new(1200, 768))
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
fn HandComponent(cx: Scope, hand: Vec<Card>) -> Element {
    let duel = use_shared_state::<Duel>(cx).unwrap();

    let cards = hand.iter().map(|card| {
        rsx! { div {
            // background_color: "green",
            justify_content: "center",
            align_items: "center",
            min_height: "100px",
            max_height: "100px",
            min_width: "220px",
            max_width: "220px",
            border: "2px solid grey",
            margin: "2px",
            div {
                format!("{} ({})", card.name, card.id)
            }
            div {
                if let fmsim::CardVariant::Monster{ .. } = card.variant {
                    format!("Attack: {}", card.get_stats_with_terrain(duel.read().terrain_type).unwrap().0)
                } else {
                    "".to_string()
                }
            }
            div {
                if let fmsim::CardVariant::Monster{ .. } = card.variant {
                    format!("Defense: {}", card.get_stats_with_terrain(duel.read().terrain_type).unwrap().1)
                } else {
                    "".to_string()
                }
            }
        }}
    });

    cx.render(rsx! { div {
        display: "flex",
        justify_content: "center",
        align_items: "center",
        cards

        // if less than 5 cards in hand, add empty cards
        if hand.len() < 5 {
            let empty_cards = (0..(5 - hand.len())).map(|_| {
                rsx! { div {
                    justify_content: "center",
                    align_items: "center",
                    min_height: "100px",
                    max_height: "100px",
                    min_width: "220px",
                    max_width: "220px",
                    border: "2px solid grey",
                    margin: "2px",
                    "empty"
                }}
            });
            rsx! { empty_cards }
        }
    }})
}

#[inline_props]
fn MonsterRowComponent(cx: Scope, monster_row: Vec<Option<MonsterRowPosition>>) -> Element {
    let duel = use_shared_state::<Duel>(cx).unwrap();

    cx.render(rsx! { div {
        display: "flex",
        justify_content: "center",
        align_items: "center",
        monster_row.iter().map(|monster| {
            let mut border_color = "grey";
            let content = match monster {
                Some(monster) => {
                    border_color = if monster.disabled { "red" } else { "green" };
                    border_color = if duel.read().get_player().sorl_effect_countdown.is_some() { "blue" } else { border_color };
                    rsx! { 
                        div {
                            border: "{border_color}",
                            div {
                                format!("{} ({})", monster.card.name, monster.card.id)
                            }
                            div {
                                format!("Attack: {:?}", monster.card.get_stats_with_terrain(duel.read().terrain_type).unwrap().0)
                            }
                            div {
                                format!("Defense: {:?}", monster.card.get_stats_with_terrain(duel.read().terrain_type).unwrap().1)
                            }
                            div {
                                format!("Star: {:?}", monster.get_selected_gs())
                            }
                            div {
                                format!("Mode: {:?}", monster.card_mode)
                            }
                        }
                    } 
                },
                None => {
                    rsx! { div {
                        "empty"
                    }}
                }
            };

            let border_color = format!("{}{}", "2px solid ", border_color);

            rsx! { div {
                justify_content: "center",
                align_items: "center",
                border: "{border_color}",
                min_height: "100px",
                max_height: "100px",
                max_height: "100px",
                min_width: "200px",
                max_width: "200px",
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
                    div {
                        format!("{} ({})", spell.card.name, spell.card.id)
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
                min_height: "100px",
                max_height: "100px",
                min_width: "200px",
                max_width: "200px",
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
            div {
                // print current player by duel.get_player_enum()
                style: "text-align: center;",
                format!("Current player: {:?}", duel.read().get_player_enum())
            }
            // in the top left, a div to display the terrain_type
            div {
                style: "display: flex; justify-content: space-between;",
                div {
                    style: "flex: 1;",
                    div {
                        format!("Terrain: {:?}", duel.read().terrain_type)
                    }
                    div {
                        format!("Turn: {}", duel.read().turn)
                    }
                }
                div {
                    style: "text-align: right; flex: 1;",
                    div {
                        format!("Player 2 Life Points: {}", duel.read().get_player_by_enum(PlayerEnum::Player2).life_points)
                    }
                    div {
                        format!("Player 1 Life Points: {}", duel.read().get_player_by_enum(PlayerEnum::Player1).life_points)
                    }
                }
            }
            div { style: "height: 5px;" }
            HandComponent {
                hand: duel.read().get_enemy().hand.clone()
            }
            div { style: "height: 5px;" }
            SpellRowComponent {
                spell_row: duel.read().get_enemy().spell_row.clone()
            }
            MonsterRowComponent {
                monster_row: duel.read().get_enemy().monster_row.clone()
            }
            div { style: "height: 20px;" }
            MonsterRowComponent {
                monster_row: duel.read().get_player().monster_row.clone()
            }
            SpellRowComponent {
                spell_row: duel.read().get_player().spell_row.clone()
            }
            div { style: "height: 5px;" }
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
    let duel = fmsim::Duel::random();
    use_shared_state_provider(cx, || duel);

    cx.render(rsx! { DuelComponent {}})
}
