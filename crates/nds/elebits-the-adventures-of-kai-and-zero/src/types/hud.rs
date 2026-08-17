use rustcheevos::{
    add_address, bits24, bits32, chain,
    prelude::*,
    types::{
        chain::{Chain, PendingChain},
        memory::MemoryRef,
    },
};

use crate::mem;

pub struct Hud;

impl Hud {
    pub fn combo_text_pointer() -> PendingChain<MemoryRef> {
        const HUD_POINTER_OFFSET: usize = 0x90;
        const COMBO_TEXT_OFFSET: usize = 0x18;
        chain!(
            add_address!(bits24!(mem::game_data_struct())),
            add_address!(bits24!(HUD_POINTER_OFFSET)),
            bits24!(COMBO_TEXT_OFFSET),
        )
    }

    pub fn combo_text_pointer_not_null() -> Chain {
        Hud::combo_text_pointer().ne(0).into()
    }

    pub fn combo_number() -> PendingChain<MemoryRef> {
        chain!(add_address!(Hud::combo_text_pointer()), bits32!(0x494),)
    }
}
