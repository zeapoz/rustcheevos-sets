#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    PowerOmegaBossArena = 0x11,
    EarthOmegaBossArena = 0x12,
    XFireOmegaBossArena = 0x13,
    XIceOmegaBossArena = 0x14,
    XEarthOmegaBossArena = 0x15,
    XWaterOmegaBossArena = 0x16,
    MobiusBossArena = 0x17,
    LeoBossArena = 0x18,
    MobiusSecondPhaseBossArena = 0x19,
    ElebitForest = 0x1a,
    ElebitMines = 0x1b,
    ResortIsland = 0x1c,
    IceWorld = 0x1d,
    RuinedWorld = 0x1e,
    SeaTemple = 0x1f,
    LibraOfCrystal = 0x20,
}

impl Location {
    pub const BOSS_LOOKUP_ENTRIES: &[(u32, &str)] = &[
        (Self::PowerOmegaBossArena as u32, "the Power Omega"),
        (Self::EarthOmegaBossArena as u32, "the Earth Omega"),
        (Self::XFireOmegaBossArena as u32, "the X Fire Omega"),
        (Self::XIceOmegaBossArena as u32, "the X Ice Omega"),
        (Self::XEarthOmegaBossArena as u32, "the X Earth Omega"),
        (Self::XWaterOmegaBossArena as u32, "the X Water Omega"),
        (Self::MobiusBossArena as u32, "Mobius"),
        (Self::LeoBossArena as u32, "a mysterious kid"),
        (
            Self::MobiusSecondPhaseBossArena as u32,
            "an empowered Mobius",
        ),
    ];

    pub fn id(self) -> u32 {
        self as u32
    }

    pub fn world_name(self) -> &'static str {
        match self {
            Self::ElebitForest => "Elebit Forest",
            Self::ElebitMines => "Elebit Mines",
            Self::ResortIsland => "Resort Island",
            Self::IceWorld => "Ice World",
            Self::RuinedWorld => "Ruined World",
            Self::SeaTemple => "Sea Temple",
            Self::LibraOfCrystal => "Libra of Crystal",
            _ => unreachable!(),
        }
    }

    pub fn number(self) -> u32 {
        self.id() - Self::ElebitForest.id() + 1
    }

    pub fn all_worlds() -> &'static [Location] {
        &[
            Location::ElebitForest,
            Location::ElebitMines,
            Location::ResortIsland,
            Location::IceWorld,
            Location::RuinedWorld,
            Location::SeaTemple,
            Location::LibraOfCrystal,
        ]
    }
}
