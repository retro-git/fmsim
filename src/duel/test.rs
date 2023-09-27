use std::any::Any;

use super::{field::FaceDirection, command::HandPlaySingleMonsterCmd};

pub struct HandPlaySingleMonsterCmdBuilder<S> {
    state: S,
    hand_index: Option<usize>,
    face_direction: Option<FaceDirection>,
    field_index: Option<usize>,
}

pub struct Start;
pub struct HandIndexSet;
pub struct FaceDirectionSet;

impl HandPlaySingleMonsterCmdBuilder<Start> {
    pub fn new() -> Self {
        HandPlaySingleMonsterCmdBuilder {
            state: Start,
            hand_index: None,
            face_direction: None,
            field_index: None,
        }
    }

    pub fn hand_index(self, hand_index: usize) -> HandPlaySingleMonsterCmdBuilder<HandIndexSet> {
        HandPlaySingleMonsterCmdBuilder {
            state: HandIndexSet,
            hand_index: Some(hand_index),
            face_direction: self.face_direction,
            field_index: self.field_index,
        }
    }
}

impl HandPlaySingleMonsterCmdBuilder<HandIndexSet> {
    pub fn face_direction(self, face_direction: FaceDirection) -> HandPlaySingleMonsterCmdBuilder<FaceDirectionSet> {
        HandPlaySingleMonsterCmdBuilder {
            state: FaceDirectionSet,
            hand_index: self.hand_index,
            face_direction: Some(face_direction),
            field_index: self.field_index,
        }
    }
}

impl HandPlaySingleMonsterCmdBuilder<FaceDirectionSet> {
    pub fn field_index(self, field_index: usize) -> HandPlaySingleMonsterCmd {
        HandPlaySingleMonsterCmd {
            hand_index: self.hand_index.unwrap(),
            face_direction: self.face_direction.unwrap(),
            field_index,
        }
    }
}

struct Magic;
struct Ritual;
struct Equip;
struct Empty;

enum CardCommand {
    PlayMagic,
    PlayRitual,
    PlayEquip(usize),
}

impl Magic {
    fn execute(&self) -> CardCommand {
        CardCommand::PlayMagic
    }
}

impl Ritual {
    fn execute(&self) -> CardCommand {
        CardCommand::PlayRitual
    }
}

impl Equip {
    fn execute(&self, index: usize) -> CardCommand {
        CardCommand::PlayEquip(index)
    }
}

struct PlayerCards<Card1, Card2, Card3> {
    card1: Card1,
    card2: Card2,
    card3: Card3,
}

impl PlayerCards<Empty, Empty, Empty> {
    fn new() -> Self {
        PlayerCards {
            card1: Empty,
            card2: Empty,
            card3: Empty,
        }
    }
}

impl<C1, C2, C3> PlayerCards<C1, C2, C3> {
    fn get_card1(&self) -> &C1 {
        &self.card1
    }

    fn get_card2(&self) -> &C2 {
        &self.card2
    }

    fn get_card3(&self) -> &C3 {
        &self.card3
    }

    // set_card1. this can change the type.
    fn set_card1<C>(self, card: C) -> PlayerCards<C, C2, C3> {
        PlayerCards {
            card1: card,
            card2: self.card2,
            card3: self.card3,
        }
    }

    // set_card2. this can change the type.
    fn set_card2<C>(self, card: C) -> PlayerCards<C1, C, C3> {
        PlayerCards {
            card1: self.card1,
            card2: card,
            card3: self.card3,
        }
    }

    // set_card3. this can change the type.
    fn set_card3<C>(self, card: C) -> PlayerCards<C1, C2, C> {
        PlayerCards {
            card1: self.card1,
            card2: self.card2,
            card3: card,
        }
    }
}

enum PlayerCardCommand {
    PlayMagic,
    PlayRitual,
    PlayEquip(usize),
}

struct HandState;
struct FieldState {
    selected_hand_index: usize,
}

struct Duel<T> {
    state: T,
    previous_states: Vec<Box<dyn Any>>, // Stack of previous states
}

impl Duel<HandState> {
    fn new() -> Self {
        Self {
            state: HandState {
                // Initialize HandState fields here
            },
            previous_states: Vec::new(),
        }
    }

    // Modify select_from_hand to take a hand_index parameter
    fn select_from_hand(mut self, hand_index: usize) -> Duel<FieldState> {
        // Push the current state onto the stack before transitioning
        self.previous_states.push(Box::new(self.state));

        Duel {
            state: FieldState {
                selected_hand_index: hand_index,
                // Initialize other FieldState fields here
            },
            previous_states: self.previous_states,
        }
    }
}

impl Duel<FieldState> {
    // FieldState-specific methods here

    fn undo(mut self) -> Duel<HandState> {
        // Pop the last state off the stack and revert to it
        let previous_state = self.previous_states.pop().unwrap().downcast::<HandState>().unwrap();

        Duel {
            state: *previous_state,
            previous_states: self.previous_states,
        }
    }

    fn end_turn(mut self) -> Duel<HandState> {
        // Push the current state onto the stack before transitioning
        self.previous_states.push(Box::new(self.state));

        Duel {
            state: HandState {
                // Initialize HandState fields here
            },
            previous_states: self.previous_states,
        }
    }
}

// PlayerCardCommandBuilder
// contains a function to select a card by index from a PlayerCards.
// if the type of the card is Ritual, then return PlayRitual.
// if the type of the card is Magic, then return PlayMagic.
// if the type of the card is Equip, then return a different builder that expects a further index.
// once that index is provided, return PlayEquip.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_commands_from_file() {
        // Simulate a file with two commands: "select_from_hand 5" and "end_turn"
        let file_contents = "select_from_hand 5\nend_turn";

        // Split the file into lines
        let mut lines = file_contents.lines();

        // Start a new duel
        let duel = Duel::new();

        let first_line = lines.next().unwrap();
        let parts = first_line.split_whitespace().collect::<Vec<&str>>();

        let duel: Duel<FieldState> = match parts.as_slice() {
            ["select_from_hand", hand_index] => {
                let hand_index = hand_index.parse().expect("Invalid hand index");
                duel.select_from_hand(hand_index)
            }
            _ => panic!("Invalid command"),
        };

        let second_line = lines.next().unwrap();
        let parts = second_line.split_whitespace().collect::<Vec<&str>>();

        let duel = match parts.as_slice() {
            ["end_turn"] => {
                duel.end_turn()
            }
            _ => panic!("Invalid command"),
        };
    }

    #[test]
    fn test_duel_new_select_hand_undo() {
        let duel = Duel::new();
        let duel = duel.select_from_hand(5);
        let duel = duel.undo();
        let duel = duel.select_from_hand(6);
        let duel = duel.end_turn();
    }

    #[test]
    fn test_create_hand_play_single_monster_cmd_with_builder() {
        let hand_index = 0;
        let field_index = 1;
        let face_direction = FaceDirection::Up;

       let command = HandPlaySingleMonsterCmdBuilder::new()
            .hand_index(hand_index)
            .face_direction(face_direction)
            .field_index(field_index);

        assert_eq!(command.hand_index, hand_index);
        assert_eq!(command.field_index, field_index);
        assert_eq!(command.face_direction, face_direction);
    }

    #[test]
    fn test_equip_card_command_builder() {
        let card1 = Magic;
        let card2 = Ritual;
        let card3 = Equip;

        let cards = PlayerCards::new()
            .set_card1(card1)
            .set_card2(card2)
            .set_card3(card3);

        let command = cards.get_card3().execute(0);
        let command2 = cards.get_card2().execute();
        assert!(matches!(command, CardCommand::PlayEquip(0)));
        assert!(matches!(command2, CardCommand::PlayRitual));

        let cards = cards.set_card3(Empty);

        // type safety ensures that this is not possible.
        // cards.get_card3().execute();
    }
}