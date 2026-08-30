use rustcheevos::{
    prelude::*,
    types::{chain::Chain, memory::MemoryRef, requirement::Condition},
};

use crate::mem;

pub struct Hud;

impl Hud {
    pub fn combo_text_pointer() -> Chain<MemoryRef> {
        const HUD_POINTER_OFFSET: usize = 0x90;
        const COMBO_TEXT_OFFSET: usize = 0x18;
        chain!(
            add_address!(bits24!(mem::game_data_struct())),
            add_address!(bits24!(HUD_POINTER_OFFSET)),
            bits24!(COMBO_TEXT_OFFSET),
        )
    }

    pub fn combo_text_pointer_not_null() -> Chain<Condition> {
        Hud::combo_text_pointer().ne(0)
    }

    pub fn combo_number() -> Chain<MemoryRef> {
        chain!(add_address!(Hud::combo_text_pointer()), bits32!(0x494),)
    }
}
