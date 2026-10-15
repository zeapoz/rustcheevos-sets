use std::fmt;

use rustcheevos::prelude::*;
use rustcheevos::types::memory::MemoryRef;
use rustcheevos::types::requirement::Condition;
use util::bit_at;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sound {
    Taiko = 0x0,
    Deluxe = 0x1,
    Drum = 0x2,
    Synthesizer = 0x3,
    Tambourine = 0x4,
    WoodenFish = 0x5,
    Shuriken = 0x6,
    Fart = 0x7,
    Donchan = 0x8,
    Tsuzumi = 0x9,
    FryPan = 0xa,
    Bell = 0xb,
    Quiz = 0xc,
    Rap = 0xd,
    Clapping = 0xe,
    DogAndCat = 0xf,
    EightBit = 0x10,
    Knife = 0x11,
    ChibiDon = 0x12,
    Cameraman = 0x13,
    GangsterDon = 0x14,
    YokozunaDon = 0x15,
    Chabu = 0x16,
    Gun = 0x17,
    Dynamite = 0x18,
    Conga = 0x19,
    Yahoo = 0x1a,
    FizzyDrink = 0x1b,
    Sneeze = 0x1c,
    KungFu = 0x1d,
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
