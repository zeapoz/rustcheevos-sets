use rustcheevos::types::memory::MemoryRef;

use crate::mem;

pub struct Player;

impl Player {
    pub fn current_watts() -> MemoryRef {
        mem::current_watts()
    }

    pub fn max_watts() -> MemoryRef {
        mem::max_watts()
    }

    pub fn current_health() -> MemoryRef {
        mem::current_health()
    }

    pub fn max_health() -> MemoryRef {
        mem::max_health()
    }
}
