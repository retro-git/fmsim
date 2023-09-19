pub mod base_card;
pub mod duelist;

pub use base_card::*;
pub use duelist::*;

use std::sync::LazyLock;

pub static CARDS: LazyLock<Vec<BaseCard>> = LazyLock::new(|| {
    let card_data = include_bytes!("../../data/cards.json");
    serde_json::from_slice(card_data).expect("Error while reading cards")
});

pub static DUELISTS: LazyLock<Vec<Duelist>> = LazyLock::new(|| {
    let card_data = include_bytes!("../../data/duelists.json");
    serde_json::from_slice(card_data).expect("Error while reading cards")
});

pub fn card_from_id(id: usize) -> &'static BaseCard {
    CARDS.get(id - 1).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_from_id() {
        let pegm = card_from_id(67);
        assert_eq!(pegm.name, "Perfectly Ultimate Great Moth");
        // Check Variant is Monster
        match pegm.variant {
            CardVariant::Monster { .. } => assert!(true),
            _ => assert!(false)
        }
    }
}
