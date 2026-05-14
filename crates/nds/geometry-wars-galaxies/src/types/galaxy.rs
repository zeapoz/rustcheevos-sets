use std::fmt;

use rustcheevos::{
    add_address, add_source, bit0, bitcount, chain, delta, prelude::*, types::chain::Chain,
};

use crate::types::game::{Game, MenuState, PROFILE_STRIDE};

use crate::types::planet::Planet;

pub const RETRO_EVOLVED_ID: u32 = 0xffffffff;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Galaxy {
    Alpha = 0x0,
    Beta = 0x1,
    Gamma = 0x2,
    Delta = 0x3,
    Epsilon = 0x4,
    Zeta = 0x5,
    Eta = 0x6,
    Theta = 0x7,
    Kappa = 0x8,
}

impl Galaxy {
    /// Returns the shorthand name of this galaxy.
    pub fn shorthand(self) -> &'static str {
        match self {
            Galaxy::Alpha => "α",
            Galaxy::Beta => "β",
            Galaxy::Gamma => "γ",
            Galaxy::Delta => "δ",
            Galaxy::Epsilon => "ε",
            Galaxy::Zeta => "ζ",
            Galaxy::Eta => "η",
            Galaxy::Theta => "θ",
            Galaxy::Kappa => "κ",
        }
    }

    /// Returns the full name of this galaxy.
    pub fn name(self) -> &'static str {
        match self {
            Galaxy::Alpha => "Alpha",
            Galaxy::Beta => "Beta",
            Galaxy::Gamma => "Gamma",
            Galaxy::Delta => "Delta",
            Galaxy::Epsilon => "Epsilon",
            Galaxy::Zeta => "Zeta",
            Galaxy::Eta => "Eta",
            Galaxy::Theta => "Theta",
            Galaxy::Kappa => "Kappa",
        }
    }

    /// Returns the base address of the medal data for this galaxy.
    pub fn medal_base_addr(self) -> u32 {
        match self {
            Galaxy::Alpha => 0x2e60cc,
            Galaxy::Beta => 0x2e6208,
            Galaxy::Gamma => 0x2e63bc,
            Galaxy::Delta => 0x2e65f0,
            Galaxy::Epsilon => 0x2e688c,
            Galaxy::Zeta => 0x2e6b90,
            Galaxy::Eta => 0x2e6e9c,
            Galaxy::Theta => 0x2e7220,
            Galaxy::Kappa => 0x2e75f8,
        }
    }

    /// Returns the number of planets before this galaxy.
    pub fn planets_before(self) -> u32 {
        match self {
            Galaxy::Alpha => 0,
            Galaxy::Beta => 3,
            Galaxy::Gamma => 7,
            Galaxy::Delta => 12,
            Galaxy::Epsilon => 18,
            Galaxy::Zeta => 25,
            Galaxy::Eta => 32,
            Galaxy::Theta => 40,
            Galaxy::Kappa => 49,
        }
    }

    /// Returns the number of planets in this galaxy.
    pub fn planet_count(self) -> u32 {
        match self {
            Galaxy::Alpha => 3,
            Galaxy::Beta => 4,
            Galaxy::Gamma => 5,
            Galaxy::Delta => 6,
            Galaxy::Epsilon | Galaxy::Zeta => 7,
            Galaxy::Theta => 9,
            Galaxy::Eta | Galaxy::Kappa => 8,
        }
    }

    /// Returns the planets in this galaxy.
    pub fn planets(self) -> &'static [Planet] {
        let start = self.planets_before() as usize;
        let count = self.planet_count() as usize;
        &Planet::all()[start..start + count]
    }

    /// Returns all galaxies.
    pub fn all() -> &'static [Galaxy] {
        &[
            Galaxy::Alpha,
            Galaxy::Beta,
            Galaxy::Gamma,
            Galaxy::Delta,
            Galaxy::Epsilon,
            Galaxy::Zeta,
            Galaxy::Eta,
            Galaxy::Theta,
            Galaxy::Kappa,
        ]
    }

    /// Returns a chain that checks if all galaxies are unlocked.
    pub fn unlocked_all_cond() -> Chain {
        const GALAXY_UNLOCK_FLAGS_ADDR: usize = 0x1aaf88;
        chain!(
            add_address!(Game::current_profile().mul(PROFILE_STRIDE)),
            add_source!(delta!(bitcount!(GALAXY_UNLOCK_FLAGS_ADDR))),
            add_address!(Game::current_profile().mul(PROFILE_STRIDE)),
            delta!(bit0!(GALAXY_UNLOCK_FLAGS_ADDR + 1)).gt(0),
            add_address!(Game::current_profile().mul(PROFILE_STRIDE)),
            add_source!(bitcount!(GALAXY_UNLOCK_FLAGS_ADDR)),
            add_address!(Game::current_profile().mul(PROFILE_STRIDE)),
            bit0!(GALAXY_UNLOCK_FLAGS_ADDR + 1).eq(0),
            Game::menu_state().eq(MenuState::GalaxySelect as u32)
        )
    }
}

impl fmt::Display for Galaxy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
