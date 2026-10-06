use rustcheevos::prelude::*;
use rustcheevos::types::requirement::Condition;

use crate::mem;

pub enum SpecialModifier {
    Normal = 0x0,
    Auto = 0x1,
    SuddenDeath = 0x2,
    Invisible = 0x3,
}

impl SpecialModifier {
    pub fn is_not(modifier: SpecialModifier) -> Condition {
        mem::file_1_game_modifiers_special().ne(modifier as u32)
    }
}
