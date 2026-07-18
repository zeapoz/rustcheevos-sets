use rustcheevos::{
    add_address, bits8, bits32, chain, measured,
    prelude::*,
    types::{
        chain::{Chain, PendingChain},
        memory::MemoryRef,
    },
};

use crate::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OmegaField {
    ChargedWatts,
    Metadata,
    Id,
}

impl OmegaField {
    /// Returns the offset of this field relative to the base address.
    pub fn offset(&self) -> usize {
        match self {
            OmegaField::ChargedWatts => 0,
            OmegaField::Metadata => 4,
            OmegaField::Id => 8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OmegaState {
    NotObtained = 0x00,
    Obtained = 0x11,
    Evolved = 0x22,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Omega {
    Zero = 0x00,
    Mobius = 0x01,
    Ice = 0x02,
    XIce = 0x03,
    Mirror = 0x04,
    Fire = 0x05,
    XFire = 0x06,
    Water = 0x07,
    XWater = 0x08,
    Sponge = 0x09,
    Earth = 0x0a,
    XEarth = 0x0b,
    Magnet = 0x0c,
    XMagnet = 0x0d,
    Wind = 0x0e,
    XWind = 0x0f,
    Time = 0x10,
    Power = 0x11,
    XPower = 0x12,
    Surf = 0x13,
    Flight = 0x14,
    Speed = 0x15,
    Melody = 0x16,
    XMelody = 0x17,
    Warp = 0x18,
    Radar = 0x19,
    Blizzard = 0x1a,
    Flame = 0x1b,
    Aqua = 0x1c,
    Land = 0x1d,
    Storm = 0x1e,
    Strong = 0x1f,
    Night = 0x20,
    TwinBee = 0x21,
    Penta = 0x22,
    Takosuke = 0x23,
    Dewy = 0x24,
    Moai = 0x25,
    BigGreen = 0x26,
    BigRed = 0x27,
}

impl Omega {
    pub const STRUCT_SIZE_BYTES: u32 = 12;

    pub const LOOKUP_ENTRIES: &[(u32, &str)] = &[
        (Self::Zero as u32, "Zero"),
        (Self::Mobius as u32, "Mobius"),
        (Self::Ice as u32, "the Ice Omega"),
        (Self::XIce as u32, "the X Ice Omega"),
        (Self::Mirror as u32, "the Mirror Omega"),
        (Self::Fire as u32, "the Fire Omega"),
        (Self::XFire as u32, "the X Fire Omega"),
        (Self::Water as u32, "the Water Omega"),
        (Self::XWater as u32, "the X Water Omega"),
        (Self::Sponge as u32, "the Sponge Omega"),
        (Self::Earth as u32, "the Earth Omega"),
        (Self::XEarth as u32, "the X Earth Omega"),
        (Self::Magnet as u32, "the Magnet Omega"),
        (Self::XMagnet as u32, "the X Magnet Omega"),
        (Self::Wind as u32, "the Wind Omega"),
        (Self::XWind as u32, "the X Wind Omega"),
        (Self::Time as u32, "the Time Omega"),
        (Self::Power as u32, "the Power Omega"),
        (Self::XPower as u32, "the X Power Omega"),
        (Self::Surf as u32, "the Surf Omega"),
        (Self::Flight as u32, "the Flight Omega"),
        (Self::Speed as u32, "the Speed Omega"),
        (Self::Melody as u32, "the Melody Omega"),
        (Self::XMelody as u32, "the X Melody Omega"),
        (Self::Warp as u32, "the Warp Omega"),
        (Self::Radar as u32, "the Radar Omega"),
        (Self::Blizzard as u32, "the Blizzard Omega"),
        (Self::Flame as u32, "the Flame Omega"),
        (Self::Aqua as u32, "the Aqua Omega"),
        (Self::Land as u32, "the Land Omega"),
        (Self::Storm as u32, "the Storm Omega"),
        (Self::Strong as u32, "the Strong Omega"),
        (Self::Night as u32, "Night"),
        (Self::TwinBee as u32, "TwinBee"),
        (Self::Penta as u32, "Penta"),
        (Self::Takosuke as u32, "Takosuke"),
        (Self::Dewy as u32, "Dewy"),
        (Self::Moai as u32, "Moai"),
        (Self::BigGreen as u32, "Big Green"),
        (Self::BigRed as u32, "Big Red"),
    ];

    pub fn id(self) -> u32 {
        self as u32
    }

    /// Returns the base address for this Omega.
    pub fn base_addr(self) -> usize {
        mem::vector_of_owned_omegas() + (self.id() * Self::STRUCT_SIZE_BYTES) as usize
    }

    /// Returns the address holding the obtained state for this Omega.
    pub fn obtained_state(self) -> MemoryRef {
        bits8!(self.base_addr() + 6)
    }

    /// Returns a chain that corresponds to the ID of the currently active Omega.
    pub fn active_id() -> PendingChain<MemoryRef> {
        chain!(
            add_address!(mem::active_omega_index().mul(Self::STRUCT_SIZE_BYTES)),
            bits32!(mem::active_omega_party() + OmegaField::Id.offset())
        )
    }

    pub fn active_form() -> Chain {
        chain!(
            add_address!(mem::active_omega_index().mul(Self::STRUCT_SIZE_BYTES)),
            measured!(bits8!(mem::active_omega_party() + 6))
        )
    }

    pub fn all_standard() -> &'static [Omega] {
        &[
            Omega::Zero,
            Omega::Mobius,
            Omega::Ice,
            Omega::XIce,
            Omega::Mirror,
            Omega::Fire,
            Omega::XFire,
            Omega::Water,
            Omega::XWater,
            Omega::Sponge,
            Omega::Earth,
            Omega::XEarth,
            Omega::Magnet,
            Omega::XMagnet,
            Omega::Wind,
            Omega::XWind,
            Omega::Time,
            Omega::Power,
            Omega::XPower,
            Omega::Surf,
            Omega::Flight,
            Omega::Speed,
            Omega::Melody,
            Omega::XMelody,
            Omega::Warp,
            Omega::Radar,
            Omega::Blizzard,
            Omega::Flame,
            Omega::Aqua,
            Omega::Land,
            Omega::Storm,
            Omega::Strong,
            // Omega::Night, TODO: Check this
        ]
    }

    pub fn all_evolvable() -> &'static [Omega] {
        &[
            Omega::Ice,
            Omega::XIce,
            Omega::Mirror,
            Omega::Fire,
            Omega::XFire,
            Omega::Water,
            Omega::XWater,
            Omega::Sponge,
            Omega::Earth,
            Omega::XEarth,
            Omega::Magnet,
            Omega::XMagnet,
            Omega::Wind,
            Omega::XWind,
            Omega::Time,
            Omega::Power,
            Omega::XPower,
            Omega::Surf,
            Omega::Flight,
            Omega::Speed,
            Omega::Melody,
            Omega::XMelody,
            Omega::Warp,
            Omega::Radar,
        ]
    }

    pub fn all() -> &'static [Omega] {
        &[
            Omega::Zero,
            Omega::Mobius,
            Omega::Ice,
            Omega::XIce,
            Omega::Mirror,
            Omega::Fire,
            Omega::XFire,
            Omega::Water,
            Omega::XWater,
            Omega::Sponge,
            Omega::Earth,
            Omega::XEarth,
            Omega::Magnet,
            Omega::XMagnet,
            Omega::Wind,
            Omega::XWind,
            Omega::Time,
            Omega::Power,
            Omega::XPower,
            Omega::Surf,
            Omega::Flight,
            Omega::Speed,
            Omega::Melody,
            Omega::XMelody,
            Omega::Warp,
            Omega::Radar,
            Omega::Blizzard,
            Omega::Flame,
            Omega::Aqua,
            Omega::Land,
            Omega::Storm,
            Omega::Strong,
            Omega::Night,
            Omega::TwinBee,
            Omega::Penta,
            Omega::Takosuke,
            Omega::Dewy,
            Omega::Moai,
            Omega::BigGreen,
            Omega::BigRed,
        ]
    }
}
