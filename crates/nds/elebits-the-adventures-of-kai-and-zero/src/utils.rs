use rustcheevos::{bit0, bit1, bit2, bit3, bit4, bit5, bit6, bit7};

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
        _ => unreachable!(),
    }
}

pub fn world_pink_elebit_bits(world: Location) -> [(usize, u32); 3] {
    let starting_bit = (world.number() - 1) * PINK_ELEBITS_PER_WORLD;
    let mut result = [(0usize, 0u32); 3];
    for i in 0..PINK_ELEBITS_PER_WORLD {
        let bit = starting_bit + i;
        result[i as usize] = (mem::pink_elebit_flags() + (bit / 8) as usize, bit % 8);
    }
    result
}

pub fn world_battery_bits(world: Location) -> [(usize, u32); 6] {
    let mut result = [(0usize, 0u32); 6];
    let yellow_start = (world.number() - 1) * YELLOW_PER_WORLD;
    for i in 0..YELLOW_PER_WORLD {
        let bit = yellow_start + i;
        result[i as usize] = (mem::yellow_battery_flags() + (bit / 8) as usize, bit % 8);
    }
    let red_start = (world.number() - 1) * RED_PER_WORLD;
    for i in 0..RED_PER_WORLD {
        let bit = red_start + i;
        result[(YELLOW_PER_WORLD + i) as usize] =
            (mem::red_battery_flags() + (bit / 8) as usize, bit % 8);
    }
    result
}
