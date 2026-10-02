use rustcheevos::{
    prelude::*,
    types::{
        chain::Chain,
        memory::MemoryRef,
        requirement::{Arithmetic, Condition},
    },
};

use crate::{mem, util::PSP_POINTER_MASK};

pub struct InGameStats;

impl InGameStats {
    pub fn stats_pointer() -> Chain<Arithmetic> {
        chain!(
            add_address!(mem::in_game_structs_0x1ffffff().bitwise_and(PSP_POINTER_MASK)),
            add_address!(bits32!(0xc).bitwise_and(PSP_POINTER_MASK)),
            add_address!(bits32!(0x4).bitwise_and(PSP_POINTER_MASK)),
            add_address!(bits32!(0x4).bitwise_and(PSP_POINTER_MASK)),
        )
    }

    pub fn current_combo() -> Chain<MemoryRef> {
        chain!(Self::stats_pointer(), bits32!(0x4))
    }

    pub fn highest_combo() -> Chain<MemoryRef> {
        chain!(Self::stats_pointer(), bits32!(0x8))
    }

    pub fn great_hits() -> Chain<MemoryRef> {
        chain!(Self::stats_pointer(), bits32!(0xc))
    }

    pub fn good_hits() -> Chain<MemoryRef> {
        chain!(Self::stats_pointer(), bits32!(0x10))
    }

    pub fn misses() -> Chain<MemoryRef> {
        chain!(Self::stats_pointer(), bits32!(0x14))
    }

    pub fn total_hits() -> Chain<MemoryRef> {
        chain!(Self::stats_pointer(), bits32!(0x18))
    }

    pub fn current_score() -> Chain<MemoryRef> {
        chain!(Self::stats_pointer(), bits32!(0x1c))
    }

    pub fn quoata_guage() -> Chain<MemoryRef> {
        chain!(Self::stats_pointer(), float!(0x20))
    }

    pub fn pointer_not_null() -> Condition {
        mem::in_game_structs_0x1ffffff().ne(0)
    }
}
