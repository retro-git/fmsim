use enum_dispatch::enum_dispatch;

use super::Duel;

#[enum_dispatch]
pub trait DuelCommand {
    fn execute(&self, duel: &mut Duel);
}

pub struct HandPlaySingleMonster {
    pub hand_index: usize,
    pub field_index: usize,
    pub direction: bool,
}
impl DuelCommand for HandPlaySingleMonster {
    fn execute(&self, duel: &mut Duel) {
        todo!()
    }
}

pub struct HandPlaySingleMagic {
    pub hand_index: usize,
}
impl DuelCommand for HandPlaySingleMagic {
    fn execute(&self, duel: &mut Duel) {
        todo!()
    }
}

pub struct HandPlaySingleTrap {
    pub hand_index: usize,
}
impl DuelCommand for HandPlaySingleTrap {
    fn execute(&self, duel: &mut Duel) {
        todo!()
    }
}

pub struct HandPlaySingleEquip {
    pub hand_index: usize,
    pub monster_index: usize,
    pub direction: bool,
}
impl DuelCommand for HandPlaySingleEquip {
    fn execute(&self, duel: &mut Duel) {
        todo!()
    }
}

pub struct HandPlayMultiple {
    pub hand_indices: Vec<usize>,
    pub field_index: usize,
}
impl DuelCommand for HandPlayMultiple {
    fn execute(&self, duel: &mut Duel) {
        todo!()
    }
}

pub struct SetGuardianStar {
    pub star: bool,
}
impl DuelCommand for SetGuardianStar {
    fn execute(&self, duel: &mut Duel) {
        todo!()
    }
}

pub struct FieldAttack {
    pub player_monster_index: usize,
    pub opponent_monster_index: usize,
}
impl DuelCommand for FieldAttack {
    fn execute(&self, duel: &mut Duel) {
        todo!()
    }
}

pub struct FieldChangeMode {
    pub monster_index: usize,
}
impl DuelCommand for FieldChangeMode {
    fn execute(&self, duel: &mut Duel) {
        todo!()
    }
}

pub struct FieldPlayEquip {
    pub equip_index: usize,
    pub monster_index: usize,
}
impl DuelCommand for FieldPlayEquip {
    fn execute(&self, duel: &mut Duel) {
        todo!()
    }
}

pub struct FieldPlaySpell {
    pub spell_index: usize,
}
impl DuelCommand for FieldPlaySpell {
    fn execute(&self, duel: &mut Duel) {
        todo!()
    }
}

pub struct EndTurn;
impl DuelCommand for EndTurn {
    fn execute(&self, duel: &mut Duel) {
        todo!()
    }
}

#[enum_dispatch(DuelCommand)]
pub enum DuelCommandEnum {
    HandPlayMultiple,
    SetGuardianStar,
    FieldAttack,
    FieldChangeMode,
    FieldPlayEquip,
    FieldPlaySpell,
    EndTurn,
}
