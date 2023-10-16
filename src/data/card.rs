use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    monster_terrain_relation, AdvantageRelation, GuardianStarType, MagicEffectEnum, MonsterType,
    TerrainType, TrapEffectEnum, CARDS,
};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Card {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub stars: u32,
    pub password: Option<u32>,
    pub attribute: u32,
    pub abc_sort: u32,
    pub max_sort: u32,
    pub atk_sort: u32,
    pub def_sort: u32,
    pub typ_sort: u32,
    pub ai_sort: u32,
    pub ai_gs: Option<u32>,
    pub fusions: HashMap<usize, usize>,
    pub variant: CardVariant,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum CardVariant {
    Monster {
        monster_type: MonsterType,
        attack: i32,
        defense: i32,
        guardian_star_a: GuardianStarType,
        guardian_star_b: GuardianStarType,
        level: u32,
    },
    Ritual {
        card1_id: usize,
        card2_id: usize,
        card3_id: usize,
        result_card_id: usize,
    },
    Equip {
        equips: Vec<usize>,
    },
    Magic(MagicEffectEnum),
    Trap(TrapEffectEnum),
}

impl Card {
    pub fn get_base_stats(&self) -> Option<(i32, i32)> {
        // we need to get_card_from_id to get the base stats of the card
        // the stats on our card may have been modified e.g. by equips
        let base_card = card_from_id(self.id);
        match &base_card.variant {
            CardVariant::Monster {
                attack, defense, ..
            } => Some((*attack, *defense)),
            _ => None,
        }
    }

    pub fn get_stats_no_terrain(&self) -> Option<(i32, i32)> {
        match &self.variant {
            CardVariant::Monster {
                attack, defense, ..
            } => Some((*attack, *defense)),
            _ => None,
        }
    }

    pub fn get_stats_with_terrain(&self, terrain_type: TerrainType) -> Option<(i32, i32)> {
        // use monster_terrain_relation to check if advantageous (+500), disadvantageous (-500), or neutral (no change)
        match &self.variant {
            CardVariant::Monster {
                attack,
                defense,
                monster_type,
                ..
            } => {
                let terrain_boost = match monster_terrain_relation(*monster_type, terrain_type) {
                    AdvantageRelation::Advantaged => 500,
                    AdvantageRelation::Disadvantaged => -500,
                    AdvantageRelation::Neutral => 0,
                };
                Some((*attack + terrain_boost, *defense as i32 + terrain_boost))
            }
            _ => None,
        }
    }

    pub fn reset_stats_to_base(&mut self) {
        // panic if not a Monster
        // set the stats to the base stats
        let (base_attack, base_defense) = self.get_base_stats().unwrap();
        match &mut self.variant {
            CardVariant::Monster {
                attack, defense, ..
            } => {
                *attack = base_attack;
                *defense = base_defense;
            }
            _ => panic!("Attempted to reset stats of a non-Monster card"),
        }
    }

    pub fn modify_stats(&mut self, delta: i32) {
        // Get the current difference between the base stats and the current stats.
        // We need to assert that the difference is the same in both attack and defense.
        // Then, after the modification is done, we need to assert that the difference is still the same.

        // get the base stats
        let (base_attack, base_defense) = self.get_base_stats().unwrap();
        // get the current stats
        let (current_attack, current_defense) = self.get_stats_no_terrain().unwrap();
        // assert that the difference is the same in both attack and defense
        assert_eq!(base_attack - current_attack, base_defense - current_defense);
        // also assert that the difference is a multiple of 500
        assert_eq!((base_attack - current_attack).abs() % 500, 0);

        // panic if not a Monster
        // modify attack and defense by delta
        match &mut self.variant {
            CardVariant::Monster {
                attack, defense, ..
            } => {
                *attack = *attack + delta;
                *defense = *defense + delta;

                // assert that the difference is still the same
                let (current_attack, current_defense) = self.get_stats_no_terrain().unwrap();
                assert_eq!(base_attack - current_attack, base_defense - current_defense);
                assert_eq!((base_attack - current_attack).abs() % 500, 0);
            }
            _ => panic!("Attempted to modify stats of a non-Monster card"),
        }
    }

    pub fn get_stats_no_terrain_base_delta(&self) -> Option<i32> {
        match &self.variant {
            CardVariant::Monster { .. } => {
                // get the base stats
                let (base_attack, base_defense) = self.get_base_stats().unwrap();
                // get the current stats
                let (current_attack, current_defense) = self.get_stats_no_terrain().unwrap();
                // assert that the difference is the same in both attack and defense
                assert_eq!(base_attack - current_attack, base_defense - current_defense);
                // also assert that the difference is a multiple of 500
                assert_eq!((base_attack - current_attack).abs() % 500, 0);

                Some(base_attack - current_attack)
            }
            _ => None,
        }
    }
}

pub fn card_from_id(id: usize) -> Card {
    CARDS.get(id - 1).unwrap().clone()
}

pub fn card_from_name(name: &str) -> Card {
    CARDS.iter().find(|card| card.name == name).unwrap().clone()
}

fn combine_cards_internal(cards: Vec<Card>) -> Vec<Card> {
    let mut combined_cards = Vec::new();
    let mut combined_card = cards[0].clone();
    // combined_cards.push(combined_card.clone());
    for card in cards.iter().skip(1) {
        combined_card = combine(&combined_card, card);
        combined_cards.push(combined_card.clone());
    }
    combined_cards
}

pub fn combine_cards(cards: Vec<Card>) -> Vec<(Card, Card, Card)> {
    let combined_cards = combine_cards_internal(cards.clone());
    // We need to create a new iterator, using chain, to iterate over both cards and combined_cards,
    // If we represent cards as X and combined_cards as Y, we want the following: X1 X2 Y1 X3 Y2 X4 Y3
    // And so on until only a Y remains.
    // That is, the first two elements should be the first two cards. From then on, we interleave combined_cards with the remaining cards.

    let first_pair = cards.iter().take(2).chain(combined_cards.iter().take(1));
    let remaining_cards = cards.iter().skip(2);
    let remaining_combined_cards = combined_cards.iter().skip(1);

    // alternate between remaining_cards and remaining_combined_cards
    let remaining_elements = remaining_cards
        .zip(remaining_combined_cards)
        .flat_map(|(card, combined_card)| vec![card, combined_card]);

    let chained = first_pair.chain(remaining_elements);
    // to create the final output, combined_cards_io_pairs, we need to loop through the chained iterator.
    // we need to take the first 3 elements to create the first tuple. then skip ahead 2 elements, and repeat.
    // Note that this means every third element will appear twice in the output.
    let mut combined_cards_io_pairs: Vec<(Card, Card, Card)> = Vec::new();
    let mut iter = chained.into_iter().peekable();
    let mut first_iteration = true;
    loop {
        let card1 = if first_iteration {
            iter.next()
        } else {
            // get the last element of the last pair of combined_cards_io_pairs
            Some(&combined_cards_io_pairs.last().unwrap().2)
        };
        let card2 = iter.next();
        let combined_card = iter.next();
        match (card1, card2, combined_card) {
            (Some(card1), Some(card2), Some(combined_card)) => {
                combined_cards_io_pairs.push((card1.clone(), card2.clone(), combined_card.clone()))
            }
            _ => break,
        }
        first_iteration = false;
    }
    combined_cards_io_pairs
}

pub fn check_all_successful_equips(io_pairs: Vec<(Card, Card, Card)>) -> bool {
    // check if all the combined cards are the same as the second card in the io_pair
    // if so, return true
    // otherwise, return false
    io_pairs.iter().all(|(card1, card2, combined_card)| {
        let (_equip_card, monster_card) = match (&card1.variant, &card2.variant) {
            (CardVariant::Equip { .. }, CardVariant::Monster { .. }) => (card1, card2),
            (CardVariant::Monster { .. }, CardVariant::Equip { .. }) => (card2, card1),
            (_, _) => return false,
        };

        if monster_card.id == combined_card.id {
            let (attack, defense) = monster_card.get_stats_no_terrain().unwrap();
            let (combined_attack, combined_defense) = combined_card.get_stats_no_terrain().unwrap();
            assert_eq!(attack - defense, combined_attack - combined_defense);
            return combined_attack > attack;
        } else {
            return false;
        }
    })
}

pub fn get_amount_of_equip_boosts(io_pairs: Vec<(Card, Card, Card)>) -> u32 {
    // for each io_pair, we need to check if equip(card1, card2) is Some
    // if so, that means that out of card1 and card2, one of them is a monster and the other is an equip
    // we need to find which one is the monster, and then check the difference between its stats and the combined card's stats
    // we then need to add this difference to the total amount of boosts
    let mut total_boosts = 0;
    for (card1, card2, combined_card) in io_pairs {
        // check that out of card1 and card2, one of them is a monster and the other is an equip
        // if so, add the difference between the monster's stats and the combined card's stats to total_boosts
        // also assert that the difference between the attack and defense is the same
        let (_equip_card, monster_card) = match (&card1.variant, &card2.variant) {
            (CardVariant::Equip { .. }, CardVariant::Monster { .. }) => (card1, card2),
            (CardVariant::Monster { .. }, CardVariant::Equip { .. }) => (card2, card1),
            (_, _) => continue,
        };

        if monster_card.id == combined_card.id {
            let (attack, defense) = monster_card.get_stats_no_terrain().unwrap();
            let (combined_attack, combined_defense) = combined_card.get_stats_no_terrain().unwrap();
            assert_eq!(attack - defense, combined_attack - combined_defense);
            total_boosts += (attack - combined_attack).abs() as u32;
        }
    }
    total_boosts
}

pub fn combine(card1: &Card, card2: &Card) -> Card {
    // First we attempt to fuse the cards. If this fails, we then attempt to equip.
    // If this fails again, and both cards are monsters, we return the second card.
    // If one of the cards is a monster but the other is not, we return whichever is the monster.
    // In any other case, we return the second card.
    use CardVariant::*;

    fuse(card1, card2)
        .or_else(|| equip(card1, card2))
        .unwrap_or_else(|| match (&card1.variant, &card2.variant) {
            (Monster { .. }, Monster { .. }) => card2.clone(),
            (Monster { .. }, _) => card1.clone(),
            (_, Monster { .. }) => card2.clone(),
            (_, _) => card2.clone(),
        })
}

pub fn fuse(card1: &Card, card2: &Card) -> Option<Card> {
    if card1.fusions.contains_key(&card2.id) {
        Some(card_from_id(card1.fusions[&card2.id]))
    } else if card2.fusions.contains_key(&card1.id) {
        Some(card_from_id(card2.fusions[&card1.id]))
    } else {
        None
    }
}

pub fn equip(card1: &Card, card2: &Card) -> Option<Card> {
    let (equip_card, monster_card) = match (&card1.variant, &card2.variant) {
        (CardVariant::Equip { .. }, CardVariant::Monster { .. }) => (card1, card2),
        (CardVariant::Monster { .. }, CardVariant::Equip { .. }) => (card2, card1),
        (_, _) => return None,
    };

    let mut monster_clone = monster_card.clone();
    if let CardVariant::Equip { equips } = &equip_card.variant {
        if equips.contains(&monster_clone.id) {
            if let CardVariant::Monster { .. } = &mut monster_clone.variant {
                let boost_amount = if equip_card.name == "Megamorph" {
                    1000
                } else {
                    500
                };
                monster_clone.modify_stats(boost_amount);
            }
        } else {
            return None;
        }
    }
    Some(monster_clone)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_cards_io_pairs() {
        let dt = card_from_name("Dragon Treasure");
        let thtd = card_from_name("Twin-headed Thunder Dragon");
        let td = card_from_name("Thunder Dragon");
        let pugm = card_from_name("Perfectly Ultimate Great Moth");
        let sorl = card_from_name("Swords of Revealing Light");
        let mm = card_from_name("Megamorph");
        let cards_to_combine = vec![
            td.clone(),
            td.clone(),
            mm.clone(),
            sorl.clone(),
            pugm.clone(),
            dt.clone(),
            mm.clone(),
            thtd.clone(),
            dt.clone(),
        ];
        let io_pairs = combine_cards(cards_to_combine.clone());
        // dbg print combined_card tuples, but only print the names
        for (card1, card2, combined_card) in &io_pairs {
            println!(
                "Card1: {}, Card2: {}, Combined Card: {}",
                card1.name, card2.name, combined_card.name
            );
        }
        let combined_cards = combine_cards_internal(cards_to_combine);
        for (i, combined_card) in combined_cards.iter().enumerate() {
            println!("Combined Card {}: {}", i, combined_card.name);
        }

        // assert the amoiunt of equip boosts is 2500
        assert_eq!(get_amount_of_equip_boosts(io_pairs), 2500);
    }
}
