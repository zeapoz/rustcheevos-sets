use rustcheevos::{
    add_address, bits16, bits24, bits32, chain,
    prelude::*,
    types::{
        chain::{Chain, PendingChain},
        memory::MemoryRef,
    },
};

use crate::mem;

pub struct Boss;

impl Boss {
    pub fn data_base_pointer() -> PendingChain<MemoryRef> {
        chain!(
            add_address!(bits24!(mem::game_data_struct())),
            bits24!(0x7c),
        )
    }

    pub fn null_pointer_check() -> Chain {
        Boss::data_base_pointer().ne(0).into()
    }

    pub fn data_pointer() -> Chain {
        chain!(
            add_address!(Boss::data_base_pointer()),
            add_address!(bits24!(0xa0)),
        )
    }

    pub fn health_numerator() -> PendingChain<MemoryRef> {
        chain!(Boss::data_pointer(), bits16!(0x10))
    }

    pub fn timer() -> PendingChain<MemoryRef> {
        chain!(
            add_address!(bits24!(mem::game_data_struct())),
            add_address!(bits24!(0x6c)),
            bits32!(0x84)
        )
    }
}

pub struct PowerOmegaBoss;

impl PowerOmegaBoss {
    pub fn attack_state() -> PendingChain<MemoryRef> {
        chain!(Boss::data_pointer(), bits32!(0x2ac))
    }

    pub fn bounces_left() -> PendingChain<MemoryRef> {
        chain!(Boss::data_pointer(), bits16!(0x2b6))
    }
}

pub struct XFireOmegaBoss;

impl XFireOmegaBoss {
    pub fn attack_state() -> PendingChain<MemoryRef> {
        chain!(Boss::data_pointer(), bits16!(0x2f2))
    }
}

pub struct XIceOmegaBoss;

impl XIceOmegaBoss {
    pub fn snake_attack_counter() -> PendingChain<MemoryRef> {
        chain!(add_address!(Boss::data_base_pointer()), bits16!(0x112))
    }
}

pub struct LeoBoss;

impl LeoBoss {
    pub fn next_attack_pattern() -> PendingChain<MemoryRef> {
        chain!(add_address!(Boss::data_base_pointer()), bits16!(0xf4))
    }
}
