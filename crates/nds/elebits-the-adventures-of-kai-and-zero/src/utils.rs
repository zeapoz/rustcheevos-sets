use rustcheevos::types::memory::MemoryRef;
use util::bit_at;

use crate::mem;
use crate::types::location::Location;

pub const PINK_ELEBITS_PER_WORLD: u32 = 3;
pub const YELLOW_PER_WORLD: u32 = 4;
pub const RED_PER_WORLD: u32 = 2;
pub const BATTERIES_PER_WORLD: u32 = YELLOW_PER_WORLD + RED_PER_WORLD;

pub fn world_pink_elebit_bits(world: Location) -> [MemoryRef; 3] {
    let starting_bit = (world.number() - 1) * PINK_ELEBITS_PER_WORLD;
    let mut result = [bit_at(0, 0); 3];
    for i in 0..PINK_ELEBITS_PER_WORLD {
        let bit = starting_bit + i;
        result[i as usize] = bit_at(
            mem::pink_elebit_flags() + (bit / 8) as usize,
            (bit % 8) as usize,
        );
    }
    result
}

pub fn world_battery_bits(world: Location) -> [MemoryRef; 6] {
    let mut result = [bit_at(0, 0); 6];
    let yellow_start = (world.number() - 1) * YELLOW_PER_WORLD;
    for i in 0..YELLOW_PER_WORLD {
        let bit = yellow_start + i;
        result[i as usize] = bit_at(
            mem::yellow_battery_flags() + (bit / 8) as usize,
            (bit % 8) as usize,
        );
    }
    let red_start = (world.number() - 1) * RED_PER_WORLD;
    for i in 0..RED_PER_WORLD {
        let bit = red_start + i;
        result[(YELLOW_PER_WORLD + i) as usize] = bit_at(
            mem::red_battery_flags() + (bit / 8) as usize,
            (bit % 8) as usize,
        );
    }
    result
}
