use rustcheevos::{prelude::*, types::memory::MemoryRef};

pub mod psp {
    pub const PSP_POINTER_MASK: u32 = 0x1ffffff;
}

pub fn bit_at(address: usize, bit: usize) -> MemoryRef {
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
