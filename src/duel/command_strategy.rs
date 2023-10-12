use rand::Rng;

use crate::Duel;

use super::command::DuelCommandEnum;

pub trait CommandStrategy {
    fn get_command(&self, duel: &Duel) -> DuelCommandEnum;
}

pub struct RandomCommandStrategy;
impl CommandStrategy for RandomCommandStrategy {
    fn get_command(&self, duel: &Duel) -> DuelCommandEnum {
        let mut commands = duel.generate_all_valid_commands();
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..commands.len());
        commands.remove(random_index)
    }
}
