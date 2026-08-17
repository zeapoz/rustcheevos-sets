use rustcheevos::types::chain::{Chain, PendingChain};
use rustcheevos::{
    add_address, bit0, bit1, bit2, bit3, bit4, bit5, bit6, bit7, bits16, bits24, chain, delta,
    prelude::*,
};

use rustcheevos::types::memory::MemoryRef;

use crate::mem;
use crate::types::location::Location;

pub const PINK_ELEBITS_PER_WORLD: u32 = 3;
pub const YELLOW_PER_WORLD: u32 = 4;
pub const RED_PER_WORLD: u32 = 2;
pub const BATTERIES_PER_WORLD: u32 = YELLOW_PER_WORLD + RED_PER_WORLD;

pub fn bit_at(address: usize, bit: u32) -> MemoryRef {
    match bit {
        0 => bit0!(address),
        1 => bit1!(address),
        2 => bit2!(address),
        3 => bit3!(address),
        4 => bit4!(address),
        5 => bit5!(address),
        6 => bit6!(address),
        7 => bit7!(address),
        _ => panic!("invalid bit index"),
    }
}

pub fn world_pink_elebit_bits(world: Location) -> [MemoryRef; 3] {
    let starting_bit = (world.number() - 1) * PINK_ELEBITS_PER_WORLD;
    let mut result = [bit_at(0, 0); 3];
    for i in 0..PINK_ELEBITS_PER_WORLD {
        let bit = starting_bit + i;
        result[i as usize] = bit_at(mem::pink_elebit_flags() + (bit / 8) as usize, bit % 8);
    }
    result
}

pub fn world_battery_bits(world: Location) -> [MemoryRef; 6] {
    let mut result = [bit_at(0, 0); 6];
    let yellow_start = (world.number() - 1) * YELLOW_PER_WORLD;
    for i in 0..YELLOW_PER_WORLD {
        let bit = yellow_start + i;
        result[i as usize] = bit_at(mem::yellow_battery_flags() + (bit / 8) as usize, bit % 8);
    }
    let red_start = (world.number() - 1) * RED_PER_WORLD;
    for i in 0..RED_PER_WORLD {
        let bit = red_start + i;
        result[(YELLOW_PER_WORLD + i) as usize] =
            bit_at(mem::red_battery_flags() + (bit / 8) as usize, bit % 8);
    }
    result
}

pub fn combo_text_pointer() -> Chain {
    const HUD_POINTER_OFFSET: usize = 0x90;
    const COMBO_TEXT_OFFSET: usize = 0x18;
    chain!(
        add_address!(bits24!(mem::game_data_struct())),
        add_address!(bits24!(HUD_POINTER_OFFSET)),
        add_address!(bits24!(COMBO_TEXT_OFFSET)),
    )
}

pub fn combo_text_pointer_not_null() -> Chain {
    const HUD_POINTER_OFFSET: usize = 0x90;
    const COMBO_TEXT_OFFSET: usize = 0x18;
    chain!(
        add_address!(bits24!(mem::game_data_struct())),
        bits24!(HUD_POINTER_OFFSET).ne(0),
        add_address!(bits24!(mem::game_data_struct())),
        add_address!(bits24!(HUD_POINTER_OFFSET)),
        bits24!(COMBO_TEXT_OFFSET).ne(0),
    )
}

pub fn boss_timer_pointer() -> Chain {
    const PLAYER_DATA_OFFET: usize = 0x6c;
    chain!(
        add_address!(bits24!(mem::game_data_struct())),
        add_address!(bits24!(PLAYER_DATA_OFFET)),
    )
}

pub fn boss_data_base_pointer() -> Chain {
    const BOSS_DATA_POINTER_OFFSET_1: usize = 0x7c;
    chain!(
        add_address!(bits24!(mem::game_data_struct())),
        add_address!(bits24!(BOSS_DATA_POINTER_OFFSET_1)),
    )
}

pub fn boss_data_pointer() -> Chain {
    const BOSS_DATA_POINTER_OFFSET_2: usize = 0xa0;
    chain!(
        boss_data_base_pointer(),
        add_address!(bits24!(BOSS_DATA_POINTER_OFFSET_2)),
    )
}

pub fn boss_health_numerator(with_delta: bool) -> PendingChain<MemoryRef> {
    const BOSS_HEALTH_NUMERATOR_OFFST: usize = 0x10;
    chain!(
        boss_data_pointer(),
        if with_delta {
            // In Rustcheevos, we should _probably_ not iterate over all conditions and apply a flag if we can.
            delta!(bits16!(BOSS_HEALTH_NUMERATOR_OFFST))
        } else {
            bits16!(BOSS_HEALTH_NUMERATOR_OFFST)
        },
    )
}

pub fn boss_data_null_pointer_check() -> Chain {
    const BOSS_DATA_OFFSET: usize = 0x7c;
    chain!(
        add_address!(bits24!(mem::game_data_struct())),
        bits24!(BOSS_DATA_OFFSET).ne(0),
    )
}
