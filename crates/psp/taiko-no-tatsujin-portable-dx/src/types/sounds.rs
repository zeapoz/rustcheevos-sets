use std::fmt;

use rustcheevos::prelude::*;
use rustcheevos::types::memory::MemoryRef;
use rustcheevos::types::requirement::Condition;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sound {
    Taiko = 0,
    Deluxe = 1,
    Drum = 2,
    Synthesizer = 3,
    Tambourine = 4,
    WoodenFish = 5,
    Shuriken = 6,
    Fart = 7,
    Donchan = 8,
    Tsuzumi = 9,
    FryPan = 10,
    Bell = 11,
    Quiz = 12,
    Rap = 13,
    Clapping = 14,
    DogAndCat = 15,
    EightBit = 16,
    Knife = 17,
    ChibiDon = 18,
    Cameraman = 19,
    GangsterDon = 20,
    YokozunaDon = 21,
    Chabu = 22,
    Gun = 23,
    Dynamite = 24,
    Conga = 25,
    Yahoo = 26,
    FizzyDrink = 27,
    Sneeze = 28,
    KungFu = 29,
}

impl Sound {
    pub const LOCKED_FLAG: u32 = 0x0;
    pub const UNLOCKED_FLAG: u32 = 0x1;

    pub fn memory_ref(&self) -> MemoryRef {
        const BASE_ADDR: usize = 0x100ef48;

        let bit = *self as usize % 8;
        let byte_offset = *self as usize / 8;
        bit_at(BASE_ADDR + byte_offset, bit)
    }

    pub fn is_locked(&self) -> Condition {
        self.memory_ref().eq(Self::LOCKED_FLAG)
    }

    pub fn is_unlocked(&self) -> Condition {
        self.memory_ref().eq(Self::UNLOCKED_FLAG)
    }
}

impl fmt::Display for Sound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Sound::Taiko => "Taiko",
            Sound::Deluxe => "Deluxe Taiko",
            Sound::Drum => "Drum",
            Sound::Synthesizer => "Synthesizer Drum",
            Sound::Tambourine => "Tambourine",
            Sound::WoodenFish => "Wooden Fish",
            Sound::Shuriken => "Shuriken",
            Sound::Fart => "Fart",
            Sound::Donchan => "Don-chan",
            Sound::Tsuzumi => "Tsuzumi",
            Sound::FryPan => "Fry Pan",
            Sound::Bell => "Bell",
            Sound::Quiz => "Quiz",
            Sound::Rap => "Rap",
            Sound::Clapping => "Clapping",
            Sound::DogAndCat => "Dog and Cat",
            Sound::EightBit => "8-bit Taiko",
            Sound::Knife => "Knife",
            Sound::ChibiDon => "Chibi Don",
            Sound::Cameraman => "Cameraman",
            Sound::GangsterDon => "Gangster Don",
            Sound::YokozunaDon => "Yokozuna Don",
            Sound::Chabu => "Chabu-dai",
            Sound::Gun => "Gun",
            Sound::Dynamite => "Dynamite",
            Sound::Conga => "Conga",
            Sound::Yahoo => "Yahoo",
            Sound::FizzyDrink => "Fizzy Drink",
            Sound::Sneeze => "Sneeze",
            Sound::KungFu => "Kung Fu",
        };
        write!(f, "{s}")
    }
}

// TODO: Make this a shared util.
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
