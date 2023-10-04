// todo remove deprecated
#![allow(non_snake_case)]

use dioxus::events::{KeyCode, KeyboardEvent};
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_tui::TuiContext;
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

fn DuelComponent(cx: Scope) -> Element {
    let duel = use_shared_state::<Duel>(cx).unwrap().read();
    cx.render(rsx! { div {
        display: "flex",
        justify_content: "center",
        align_items: "center",
        div {
            HandComponent {
                hand: duel.get_player().hand.clone()
            }
        }
    }})
}

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || fmsim::Duel::default());

    cx.render(rsx! { DuelComponent {}})
}
