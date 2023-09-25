use crate::{card_from_id, Card};

pub fn generate_random_deck() -> Vec<Card> {
    // a deck is a list of 40 cards. cards can be attained with fmsim::card_from_id function, where ID ranges between 1 and 722.
    // the same card cannot appear more than 3 times.
    // use rand crate to generate a random deck.
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let mut deck = Vec::new();
    let mut card_counts = [0; 722];
    let mut available_cards: Vec<usize> = (1..723).collect();

    while deck.len() < 40 {
        let card_index = rng.gen_range(0..available_cards.len());
        let card_id = available_cards[card_index];
        deck.push(card_from_id(card_id));
        card_counts[card_id - 1] += 1;
        if card_counts[card_id - 1] >= 3 {
            available_cards.remove(card_index);
        }
    }

    // deck.shuffle(&mut rng);
    deck
}

pub fn deck_is_valid(deck: &[Card]) -> bool {
    // a deck is valid if no card appears more than 3 times.
    let mut card_counts = [0; 722];
    for card in deck {
        card_counts[(card.id - 1) as usize] += 1;
    }
    card_counts.iter().all(|&count| count <= 3)
}

// test generate_random_deck
#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::DUELISTS;

    #[test]
    fn test_generate_random_deck() {
        let deck = generate_random_deck();
        assert_eq!(deck.len(), 40);
        dbg!(&deck);
        assert!(deck_is_valid(&deck));
    }
}