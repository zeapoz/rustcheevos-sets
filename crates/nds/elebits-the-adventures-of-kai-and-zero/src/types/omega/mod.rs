pub mod active;

use std::fmt;

use rustcheevos::{bits8, types::memory::MemoryRef};

use crate::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OmegaState {
    NotObtained = 0x00,
    Obtained = 0x11,
    Evolved = 0x22,
}

pub struct OmegaData {
    pub id: u32,
    pub name: &'static str,
    pub lookup: &'static str,
    pub standard: bool,
    pub evolvable: bool,
}

macro_rules! gen_omega_defs {
    ($( $variant:ident => $data:expr ),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Omega {
            $( $variant ),*
        }

        impl Omega {
            pub const STRUCT_SIZE_BYTES: u32 = 12;
            pub const OBTAINED_OFFSET: usize = 6;
            pub const ID_OFFSET: usize = 8;

            pub const ALL: &[Omega] = &[ $( Omega::$variant ),* ];

            pub const LOOKUP_ENTRIES: &[(u32, &str)] = &[
                $( ($data.id, $data.lookup) ),*
            ];

            fn data(self) -> &'static OmegaData {
                match self {
                    $( Omega::$variant => &$data ),*
                }
            }

            pub fn id(self) -> u32 {
                self.data().id
            }

            pub fn name(self) -> &'static str {
                self.data().name
            }

            /// Returns the base address for this Omega.
            pub fn base_addr(self) -> usize {
                mem::vector_of_owned_omegas() + (self.id() * Self::STRUCT_SIZE_BYTES) as usize
            }

            pub fn obtained_addr(self) -> usize {
                self.base_addr() + Self::OBTAINED_OFFSET
            }

            pub fn id_addr(self) -> usize {
                self.base_addr() + Self::ID_OFFSET
            }

            /// Returns the address holding the obtained state for this Omega.
            pub fn obtained_state(self) -> MemoryRef {
                bits8!(self.obtained_addr())
            }

            /// Returns a chain calculating the obtained state given the current file.
            pub fn save_data_obtained_state(self) -> MemoryRef {
                let offset = mem::omega_vector_save_data_file_1()
                    + (self.id() * Self::STRUCT_SIZE_BYTES) as usize;
                bits8!(offset + Self::OBTAINED_OFFSET)
            }

            pub fn all_standard() -> Vec<Omega> {
                Self::ALL
                    .iter()
                    .copied()
                    .filter(|o| o.data().standard)
                    .collect()
            }

            pub fn all_evolvable() -> Vec<Omega> {
                Self::ALL
                    .iter()
                    .copied()
                    .filter(|o| o.data().evolvable)
                    .collect()
            }
        }

        impl fmt::Display for Omega {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.name())
            }
        }
    };
}

gen_omega_defs! {
    Zero     => OmegaData { id: 0x00, name: "Zero",          lookup: "Zero",              standard: true,  evolvable: false },
    Mobius   => OmegaData { id: 0x01, name: "Mobius",        lookup: "Mobius",            standard: true,  evolvable: false },
    Ice      => OmegaData { id: 0x02, name: "Ice Omega",     lookup: "the Ice Omega",     standard: true,  evolvable: true  },
    XIce     => OmegaData { id: 0x03, name: "X Ice Omega",   lookup: "the X Ice Omega",   standard: true,  evolvable: true  },
    Mirror   => OmegaData { id: 0x04, name: "Mirror Omega",  lookup: "the Mirror Omega",  standard: true,  evolvable: true  },
    Fire     => OmegaData { id: 0x05, name: "Fire Omega",    lookup: "the Fire Omega",    standard: true,  evolvable: true  },
    XFire    => OmegaData { id: 0x06, name: "X Fire Omega",  lookup: "the X Fire Omega",  standard: true,  evolvable: true  },
    Water    => OmegaData { id: 0x07, name: "Water Omega",   lookup: "the Water Omega",   standard: true,  evolvable: true  },
    XWater   => OmegaData { id: 0x08, name: "X Water Omega", lookup: "the X Water Omega", standard: true,  evolvable: true  },
    Sponge   => OmegaData { id: 0x09, name: "Sponge Omega",  lookup: "the Sponge Omega",  standard: true,  evolvable: true  },
    Earth    => OmegaData { id: 0x0a, name: "Earth Omega",   lookup: "the Earth Omega",   standard: true,  evolvable: true  },
    XEarth   => OmegaData { id: 0x0b, name: "X Earth Omega", lookup: "the X Earth Omega", standard: true,  evolvable: true  },
    Magnet   => OmegaData { id: 0x0c, name: "Magnet Omega",  lookup: "the Magnet Omega",  standard: true,  evolvable: true  },
    XMagnet  => OmegaData { id: 0x0d, name: "X Magnet Omega",lookup: "the X Magnet Omega",standard: true,  evolvable: true  },
    Wind     => OmegaData { id: 0x0e, name: "Wind Omega",    lookup: "the Wind Omega",    standard: true,  evolvable: true  },
    XWind    => OmegaData { id: 0x0f, name: "X Wind Omega",  lookup: "the X Wind Omega",  standard: true,  evolvable: true  },
    Time     => OmegaData { id: 0x10, name: "Time Omega",    lookup: "the Time Omega",    standard: true,  evolvable: true  },
    Power    => OmegaData { id: 0x11, name: "Power Omega",   lookup: "the Power Omega",   standard: true,  evolvable: true  },
    XPower   => OmegaData { id: 0x12, name: "X Power Omega", lookup: "the X Power Omega", standard: true,  evolvable: true  },
    Surf     => OmegaData { id: 0x13, name: "Surf Omega",    lookup: "the Surf Omega",    standard: true,  evolvable: true  },
    Flight   => OmegaData { id: 0x14, name: "Flight Omega",  lookup: "the Flight Omega",  standard: true,  evolvable: true  },
    Speed    => OmegaData { id: 0x15, name: "Speed Omega",   lookup: "the Speed Omega",   standard: true,  evolvable: true  },
    Melody   => OmegaData { id: 0x16, name: "Melody Omega",  lookup: "the Melody Omega",  standard: true,  evolvable: true  },
    XMelody  => OmegaData { id: 0x17, name: "X Melody Omega",lookup: "the X Melody Omega",standard: true,  evolvable: true  },
    Warp     => OmegaData { id: 0x18, name: "Warp Omega",    lookup: "the Warp Omega",    standard: true,  evolvable: true  },
    Radar    => OmegaData { id: 0x19, name: "Radar Omega",   lookup: "the Radar Omega",   standard: true,  evolvable: true  },
    Blizzard => OmegaData { id: 0x1a, name: "Blizzard Omega",lookup: "the Blizzard Omega",standard: true,  evolvable: false },
    Flame    => OmegaData { id: 0x1b, name: "Flame Omega",   lookup: "the Flame Omega",   standard: true,  evolvable: false },
    Aqua     => OmegaData { id: 0x1c, name: "Aqua Omega",    lookup: "the Aqua Omega",    standard: true,  evolvable: false },
    Land     => OmegaData { id: 0x1d, name: "Land Omega",    lookup: "the Land Omega",    standard: true,  evolvable: false },
    Storm    => OmegaData { id: 0x1e, name: "Storm Omega",   lookup: "the Storm Omega",   standard: true,  evolvable: false },
    Strong   => OmegaData { id: 0x1f, name: "Strong Omega",  lookup: "the Strong Omega",  standard: true,  evolvable: false },
    Night    => OmegaData { id: 0x20, name: "Night",         lookup: "Night",             standard: false, evolvable: false },
    TwinBee  => OmegaData { id: 0x21, name: "TwinBee",       lookup: "TwinBee",           standard: false, evolvable: false },
    Penta    => OmegaData { id: 0x22, name: "Penta",         lookup: "Penta",             standard: false, evolvable: false },
    Takosuke => OmegaData { id: 0x23, name: "Takosuke",      lookup: "Takosuke",          standard: false, evolvable: false },
    Dewy     => OmegaData { id: 0x24, name: "Dewy",          lookup: "Dewy",              standard: false, evolvable: false },
    Moai     => OmegaData { id: 0x25, name: "Moai",          lookup: "Moai",              standard: false, evolvable: false },
    BigGreen => OmegaData { id: 0x26, name: "Big Green",     lookup: "Big Green",         standard: false, evolvable: false },
    BigRed   => OmegaData { id: 0x27, name: "Big Red",       lookup: "Big Red",           standard: false, evolvable: false },
}
