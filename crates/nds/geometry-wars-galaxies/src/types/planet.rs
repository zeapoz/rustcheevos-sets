use std::fmt;

use rustcheevos::{
    add_address, bits32, chain,
    prelude::*,
    types::{
        chain::{Chain, PendingChain},
        memory::MemoryRef,
    },
    upper4,
};

use crate::types::game::Game;

use crate::types::galaxy::Galaxy;
use crate::types::status::MedalStatus;

pub const LEVEL_DATA_BASE_ADDR: usize = 0x1a9c24;
pub const LEVEL_ADDR_STRIDE: usize = 0x30;
pub const MEDAL_ADDR_STRIDE: usize = 0x58;

#[derive(Debug, Clone, Copy)]
pub struct Planet {
    pub name: &'static str,
    pub index: u32,
    pub galaxy: Galaxy,
}

impl Planet {
    /// Returns the lookup key for this planet.
    pub fn lookup_key(&self) -> u32 {
        self.galaxy as u32 * 10 + self.index
    }

    /// Returns the 0-based index of this planet.
    fn index_usize(&self) -> usize {
        self.index as usize - 1
    }

    /// Returns the address of the level file data for this planet.
    pub fn file_data_addr(&self) -> usize {
        LEVEL_DATA_BASE_ADDR
            + self.galaxy.planets_before() * LEVEL_ADDR_STRIDE
            + self.index_usize() * LEVEL_ADDR_STRIDE
    }

    /// Returns the address of the medal for this planet.
    fn medal_addr(&self, medal: MedalStatus) -> usize {
        self.galaxy.medal_base_addr() + self.index_usize() * MEDAL_ADDR_STRIDE + medal.offset()
    }

    /// Returns the required score for this planet.
    pub fn required_score(&self, status: MedalStatus) -> MemoryRef {
        bits32!(self.medal_addr(status))
    }

    /// Returns a chain that checks if the player is in this planet.
    pub fn player_in_planet(&self) -> Chain {
        chain!(
            Game::current_galaxy_index().eq(self.galaxy as u32),
            Game::current_level_index().eq(self.index),
        )
    }

    /// Returns the address of the status of this planet.
    pub fn status_addr(&self) -> usize {
        self.file_data_addr() + 0x29
    }

    /// Returns the status of this planet as a pending chain.
    pub fn status(&self) -> PendingChain<MemoryRef> {
        chain!(
            add_address!(Game::current_profile().mul(2)),
            upper4!(self.status_addr())
        )
    }

    /// Returns a chain that checks if the status of this planet is at least the given status.
    pub fn status_is_at_least(&self, status: MedalStatus) -> Chain {
        chain!(
            self.status().ge(status as u32),
            self.status().ne(MedalStatus::Locked as u32)
        )
    }

    /// Returns all planets.
    pub fn all() -> &'static [Planet] {
        ALL
    }
}

impl fmt::Display for Planet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[rustfmt::skip]
mod planets {
    use super::Planet;
    use super::Galaxy;

    pub const CLAEIS: Planet = Planet { name: "Claeis", index: 1, galaxy: Galaxy::Alpha };
    pub const TRAINER_DUO: Planet = Planet { name: "Trainer Duo", index: 2, galaxy: Galaxy::Alpha };
    pub const TRAINER_EIS: Planet = Planet { name: "Trainer Eis", index: 3, galaxy: Galaxy::Alpha };
    pub const BATEIS: Planet = Planet { name: "Bateis", index: 1, galaxy: Galaxy::Beta };
    pub const SUREIS: Planet = Planet { name: "Sureis", index: 2, galaxy: Galaxy::Beta };
    pub const CLADUO: Planet = Planet { name: "Claduo", index: 3, galaxy: Galaxy::Beta };
    pub const MASEIS: Planet = Planet { name: "Maseis", index: 4, galaxy: Galaxy::Beta };
    pub const ORBEIS: Planet = Planet { name: "Orbeis", index: 1, galaxy: Galaxy::Gamma };
    pub const BATDUO: Planet = Planet { name: "Batduo", index: 2, galaxy: Galaxy::Gamma };
    pub const MINEIS: Planet = Planet { name: "Mineis", index: 3, galaxy: Galaxy::Gamma };
    pub const VAREIS: Planet = Planet { name: "Vareis", index: 4, galaxy: Galaxy::Gamma };
    pub const FLIEIS: Planet = Planet { name: "Flieis", index: 5, galaxy: Galaxy::Gamma };
    pub const SURPENTE: Planet = Planet { name: "Surpente", index: 1, galaxy: Galaxy::Delta };
    pub const BATTRIS: Planet = Planet { name: "Battris", index: 2, galaxy: Galaxy::Delta };
    pub const FLIDUO: Planet = Planet { name: "Fliduo", index: 3, galaxy: Galaxy::Delta };
    pub const ZOOEIS: Planet = Planet { name: "Zooeis", index: 4, galaxy: Galaxy::Delta };
    pub const MASDUO: Planet = Planet { name: "Masduo", index: 5, galaxy: Galaxy::Delta };
    pub const CLATRIS: Planet = Planet { name: "Clatris", index: 6, galaxy: Galaxy::Delta };
    pub const CLAHEX: Planet = Planet { name: "Clahex", index: 1, galaxy: Galaxy::Epsilon };
    pub const BATTETRA: Planet = Planet { name: "Battetra", index: 2, galaxy: Galaxy::Epsilon };
    pub const SURDUO: Planet = Planet { name: "Surduo", index: 3, galaxy: Galaxy::Epsilon };
    pub const LOSEIS: Planet = Planet { name: "Loseis", index: 4, galaxy: Galaxy::Epsilon };
    pub const MINDUO: Planet = Planet { name: "Minduo", index: 5, galaxy: Galaxy::Epsilon };
    pub const VARDUO: Planet = Planet { name: "Varduo", index: 6, galaxy: Galaxy::Epsilon };
    pub const ROCEIS: Planet = Planet { name: "Roceis", index: 7, galaxy: Galaxy::Epsilon };
    pub const VIRDUO: Planet = Planet { name: "Virduo", index: 1, galaxy: Galaxy::Zeta };
    pub const BATPENTE: Planet = Planet { name: "Batpente", index: 2, galaxy: Galaxy::Zeta };
    pub const SURTRIS: Planet = Planet { name: "Surtris", index: 3, galaxy: Galaxy::Zeta };
    pub const FLITRIS: Planet = Planet { name: "Flitris", index: 4, galaxy: Galaxy::Zeta };
    pub const ZOODOO: Planet = Planet { name: "Zooduo", index: 5, galaxy: Galaxy::Zeta };
    pub const MASTRIS: Planet = Planet { name: "Mastris", index: 6, galaxy: Galaxy::Zeta };
    pub const VARTRIS: Planet = Planet { name: "Vartris", index: 7, galaxy: Galaxy::Zeta };
    pub const SURTETRA: Planet = Planet { name: "Surtetra", index: 1, galaxy: Galaxy::Eta };
    pub const BATHEX: Planet = Planet { name: "Bathex", index: 2, galaxy: Galaxy::Eta };
    pub const VIREIS: Planet = Planet { name: "Vireis", index: 3, galaxy: Galaxy::Eta };
    pub const CLATETRA: Planet = Planet { name: "Clatetra", index: 4, galaxy: Galaxy::Eta };
    pub const MASTETRA: Planet = Planet { name: "Mastetra", index: 5, galaxy: Galaxy::Eta };
    pub const MINTRIS: Planet = Planet { name: "Mintris", index: 6, galaxy: Galaxy::Eta };
    pub const LOSDUO: Planet = Planet { name: "Losduo", index: 7, galaxy: Galaxy::Eta };
    pub const FLITETRA: Planet = Planet { name: "Flitetra", index: 8, galaxy: Galaxy::Eta };
    pub const VARPENTE: Planet = Planet { name: "Varpente", index: 1, galaxy: Galaxy::Theta };
    pub const BATHEPTA: Planet = Planet { name: "Bathepta", index: 2, galaxy: Galaxy::Theta };
    pub const CLAPENTE: Planet = Planet { name: "Clapente", index: 3, galaxy: Galaxy::Theta };
    pub const ORBDUO: Planet = Planet { name: "Orbduo", index: 4, galaxy: Galaxy::Theta };
    pub const ROCDUO: Planet = Planet { name: "Rocduo", index: 5, galaxy: Galaxy::Theta };
    pub const FLIPENTE: Planet = Planet { name: "Flipente", index: 6, galaxy: Galaxy::Theta };
    pub const POREIS: Planet = Planet { name: "Poreis", index: 7, galaxy: Galaxy::Theta };
    pub const VARTETRA: Planet = Planet { name: "Vartetra", index: 8, galaxy: Galaxy::Theta };
    pub const LOSTRIS: Planet = Planet { name: "Lostris", index: 9, galaxy: Galaxy::Theta };
    pub const FLIHEX: Planet = Planet { name: "Flihex", index: 1, galaxy: Galaxy::Kappa };
    pub const ORBTRIS: Planet = Planet { name: "Orbtris", index: 2, galaxy: Galaxy::Kappa };
    pub const MASPENTE: Planet = Planet { name: "Maspente", index: 3, galaxy: Galaxy::Kappa };
    pub const SURHEX: Planet = Planet { name: "Surhex", index: 4, galaxy: Galaxy::Kappa };
    pub const LOSTETRA: Planet = Planet { name: "Lostetra", index: 5, galaxy: Galaxy::Kappa };
    pub const ROCTRIS: Planet = Planet { name: "Roctris", index: 6, galaxy: Galaxy::Kappa };
    pub const ZOOTRIS: Planet = Planet { name: "Zootris", index: 7, galaxy: Galaxy::Kappa };
    pub const VIRTRIS: Planet = Planet { name: "Virtris", index: 8, galaxy: Galaxy::Kappa };
}

pub use planets::*;

pub const ALL: &[Planet] = &[
    // Alpha
    CLAEIS,
    TRAINER_DUO,
    TRAINER_EIS,
    // Beta
    BATEIS,
    SUREIS,
    CLADUO,
    MASEIS,
    // Gamma
    ORBEIS,
    BATDUO,
    MINEIS,
    VAREIS,
    FLIEIS,
    // Delta
    SURPENTE,
    BATTRIS,
    FLIDUO,
    ZOOEIS,
    MASDUO,
    CLATRIS,
    // Epsilon
    CLAHEX,
    BATTETRA,
    SURDUO,
    LOSEIS,
    MINDUO,
    VARDUO,
    ROCEIS,
    // Zeta
    VIRDUO,
    BATPENTE,
    SURTRIS,
    FLITRIS,
    ZOODOO,
    MASTRIS,
    VARTRIS,
    // Eta
    SURTETRA,
    BATHEX,
    VIREIS,
    CLATETRA,
    MASTETRA,
    MINTRIS,
    LOSDUO,
    FLITETRA,
    // Theta
    VARPENTE,
    BATHEPTA,
    CLAPENTE,
    ORBDUO,
    ROCDUO,
    FLIPENTE,
    POREIS,
    VARTETRA,
    LOSTRIS,
    // Kappa
    FLIHEX,
    ORBTRIS,
    MASPENTE,
    SURHEX,
    LOSTETRA,
    ROCTRIS,
    ZOOTRIS,
    VIRTRIS,
];
