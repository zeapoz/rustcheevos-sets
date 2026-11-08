use rustcheevos::prelude::*;
use rustcheevos::types::memory::MemoryRef;
use rustcheevos::types::requirement::Condition;

use crate::mem;
use crate::types::profile::Profile;

pub enum SpeedModifier {
    Normal = 0x0,
    Double = 0x1,
    Triple = 0x2,
    Quadruple = 0x3,
}

impl SpeedModifier {
    fn get() -> Chain<MemoryRef> {
        chain!(
            Profile::offset_for_current(),
            mem::file_1_game_modifiers_speed()
        )
    }

    pub fn is(modifier: SpeedModifier) -> Chain<Condition> {
        Self::get().eq(modifier as u32)
    }

    pub fn is_or_higher(modifier: SpeedModifier) -> Chain<Condition> {
        Self::get().ge(modifier as u32)
    }
}

pub enum SpecialModifier {
    Normal = 0x0,
    Auto = 0x1,
    SuddenDeath = 0x2,
    Invisible = 0x3,
}

impl SpecialModifier {
    fn get() -> Chain<MemoryRef> {
        chain!(
            Profile::offset_for_current(),
            mem::file_1_game_modifiers_special()
        )
    }

    pub fn is(modifier: SpecialModifier) -> Chain<Condition> {
        Self::get().eq(modifier as u32)
    }

    pub fn is_not(modifier: SpecialModifier) -> Chain<Condition> {
        Self::get().ne(modifier as u32)
    }
}

pub enum NoteModifier {
    Normal = 0x0,
    Whimsical = 0x1,
    Random = 0x2,
    Reveresed = 0x3,
}

impl NoteModifier {
    fn get() -> Chain<MemoryRef> {
        chain!(
            Profile::offset_for_current(),
            mem::file_1_game_modifiers_note()
        )
    }

    pub fn is(modifier: NoteModifier) -> Chain<Condition> {
        Self::get().eq(modifier as u32)
    }
}
