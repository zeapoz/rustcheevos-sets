use std::fmt;

use rustcheevos::{
    add_address, bits16, chain, delta,
    prelude::*,
    types::{
        chain::{Chain, PendingChain},
        memory::MemoryRef,
    },
};

use crate::types::game::{Game, PROFILE_STRIDE};

const BASE_ADDR: usize = 0x1aaee4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DroneBehaviour {
    Attack = 0x0,
    Defend = 0x1,
    Collect = 0x2,
    Snipe = 0x3,
    Sweep = 0x4,
    Ram = 0x5,
    Turret = 0x6,
    Bait = 0x7,
}

impl DroneBehaviour {
    pub const MAX_LEVEL: u32 = 10;

    /// Returns the shorthand name of this drone behaviour.
    pub fn shorthand(self) -> &'static str {
        match self {
            DroneBehaviour::Attack => "ATK",
            DroneBehaviour::Defend => "DEF",
            DroneBehaviour::Collect => "COL",
            DroneBehaviour::Snipe => "SNP",
            DroneBehaviour::Sweep => "SWP",
            DroneBehaviour::Ram => "RAM",
            DroneBehaviour::Turret => "TUR",
            DroneBehaviour::Bait => "BIT",
        }
    }

    /// Returns the full name of this drone behaviour.
    pub fn name(self) -> &'static str {
        match self {
            DroneBehaviour::Attack => "Attack",
            DroneBehaviour::Defend => "Defend",
            DroneBehaviour::Collect => "Collect",
            DroneBehaviour::Snipe => "Snipe",
            DroneBehaviour::Sweep => "Sweep",
            DroneBehaviour::Ram => "Ram",
            DroneBehaviour::Turret => "Turret",
            DroneBehaviour::Bait => "Bait",
        }
    }

    /// Returns all drone behaviours.
    pub fn all() -> &'static [DroneBehaviour] {
        &[
            DroneBehaviour::Attack,
            DroneBehaviour::Defend,
            DroneBehaviour::Collect,
            DroneBehaviour::Snipe,
            DroneBehaviour::Sweep,
            DroneBehaviour::Ram,
            DroneBehaviour::Turret,
            DroneBehaviour::Bait,
        ]
    }

    /// Returns the level of this drone behaviour.
    pub fn level(self) -> PendingChain<MemoryRef> {
        let offset = BASE_ADDR + self as usize * 2;
        chain!(
            add_address!(Game::current_profile().mul(PROFILE_STRIDE)),
            bits16!(offset)
        )
    }

    /// Returns a chain that checks if this drone behaviour is unlocked.
    pub fn is_unlocked(self) -> Chain {
        self.level().gt(0)
    }

    /// Returns a chain that checks if this drone behaviour was just unlocked.
    pub fn unlocked(self) -> Chain {
        chain!(delta!(self.level()).eq(0), self.level().eq(1))
    }
}

impl fmt::Display for DroneBehaviour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
