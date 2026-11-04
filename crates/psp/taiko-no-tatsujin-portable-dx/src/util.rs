use rustcheevos::prelude::*;
use rustcheevos::types::chain::{Chain, ResolvedChain};
use rustcheevos::types::memory::MemoryRef;

use crate::types::profile::Profile;

pub const TOTAL_NUM_DOJO: u32 = 47;
pub const TOTAL_NUM_OMIKOSHI_HEADGEAR: u32 = 25;
pub const TOTAL_NUM_OMIKOSHI_CLOTHES: u32 = 24;
pub const TOTAL_NUM_HEADGEAR: u32 = TOTAL_NUM_OMIKOSHI_HEADGEAR + 4;
pub const TOTAL_NUM_CLOTHES: u32 = TOTAL_NUM_OMIKOSHI_CLOTHES + 5;
pub const TOTAL_NUM_IN_GAME_ACHIEVEMENTS: u32 = 78;

#[derive(Debug, Clone, Copy)]
pub struct ByteArrayCounterBuilder {
    addr: usize,
    size: usize,
    delta: bool,
}

impl ByteArrayCounterBuilder {
    pub fn new(addr: usize, size: usize) -> Self {
        Self {
            addr,
            size,
            delta: false,
        }
    }

    pub fn delta(mut self) -> Self {
        self.delta = true;
        self
    }

    pub fn build(self) -> Chain<MemoryRef> {
        let add_source_chain: ResolvedChain = (0..self.size - 1)
            .map(|i| {
                chain!(Profile::offset_for_current(), {
                    let mut add_source = add_source!(bits8!(self.addr + i));
                    if self.delta {
                        add_source = delta!(add_source);
                    }
                    add_source
                })
            })
            .collect();

        chain!(
            add_source_chain,
            Profile::offset_for_current(),
            bits8!(self.addr + self.size - 1),
        )
    }
}
