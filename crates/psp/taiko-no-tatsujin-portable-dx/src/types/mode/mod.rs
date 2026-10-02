use rustcheevos::{
    prelude::*,
    types::{chain::Chain, memory::MemoryRef, requirement::Arithmetic},
};

use crate::{mem, util::PSP_POINTER_MASK};

pub mod omikoshi;

pub struct MedleyMode;

impl MedleyMode {
    pub fn hud_pointer() -> Chain<Arithmetic> {
        chain!(
            add_address!(mem::game_mode_specific_data_pointer().bitwise_and(PSP_POINTER_MASK)),
            add_address!(bits32!(0x54).bitwise_and(PSP_POINTER_MASK)),
            add_address!(bits32!(0x5c).bitwise_and(PSP_POINTER_MASK)),
        )
    }

    pub fn current_song() -> Chain<MemoryRef> {
        chain!(Self::hud_pointer(), bits32!(0x0),)
    }

    pub fn num_songs() -> Chain<MemoryRef> {
        chain!(Self::hud_pointer(), bits32!(0x4),)
    }
}
