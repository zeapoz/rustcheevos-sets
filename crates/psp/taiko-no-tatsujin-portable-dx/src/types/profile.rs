use std::fmt;

use rustcheevos::{
    prelude::*,
    types::{
        chain::{Chain, ResolvedChain},
        memory::MemoryRef,
        requirement::{Arithmetic, Condition},
    },
};

use crate::{
    mem,
    types::{result::Result, song::ALL_SONGS},
    util::{TOTAL_NUM_CLOTHES, TOTAL_NUM_HEADGEAR, TOTAL_NUM_IN_GAME_ACHIEVEMENTS},
};

use super::difficulty::Difficulty;

pub struct Profile;

impl Profile {
    pub const STRIDE: u32 = 0xDE34;

    /// Returns the offset for the current profile.
    pub fn offset_for_current() -> Arithmetic {
        add_address!(mem::currently_selected_profile().mul(Self::STRIDE))
    }

    /// Returns a chain evaluating that profiles have been loaded.
    pub fn is_loaded() -> Condition {
        mem::save_data_loaded_flag().eq(0x1)
    }

    /// Returns a chain counting the number of silver crowns.
    pub fn num_silver_crowns() -> Chain<Arithmetic> {
        Self::num_total_crowns(Result::SilverCrown)
    }

    /// Returns a chain counting the number of golden crowns.
    pub fn num_golden_crowns() -> Chain<Arithmetic> {
        Self::num_total_crowns(Result::GoldenCrown)
    }

    /// Returns a chain counting the number of drum hits on the Taiko counter.
    pub fn total_drum_hits() -> Chain<MemoryRef> {
        chain!(Self::offset_for_current(), mem::file_1_taiko_counter())
    }

    pub fn rank() -> Chain<MemoryRef> {
        chain!(Self::offset_for_current(), mem::file_1_current_rank())
    }

    /// Returns a chain counting the number of obtained headgear.
    pub fn num_headgear() -> Chain<MemoryRef> {
        Self::count_byte_array_flags(
            mem::file_1_headgear_unlock_flags(),
            TOTAL_NUM_HEADGEAR as usize,
        )
    }

    /// Returns a chain counting the number of obtained clothes.
    pub fn num_clothes() -> Chain<MemoryRef> {
        Self::count_byte_array_flags(
            mem::file_1_clothes_unlock_flags(),
            TOTAL_NUM_CLOTHES as usize,
        )
    }

    pub fn num_challenges_completed() -> Chain<MemoryRef> {
        Self::count_byte_array_flags(
            mem::in_game_achievements_unlock_flags(),
            TOTAL_NUM_IN_GAME_ACHIEVEMENTS as usize,
        )
    }

    fn count_byte_array_flags(addr: usize, size: usize) -> Chain<MemoryRef> {
        let add_source_chain: ResolvedChain = (0..size - 1)
            .map(|i| chain!(Self::offset_for_current(), add_source!(bits8!(addr + i))))
            .collect();

        chain!(
            add_source_chain,
            Self::offset_for_current(),
            bits8!(addr + size - 1),
        )
    }

    fn num_total_crowns(crown: Result) -> Chain<Arithmetic> {
        chain!(
            Self::num_crowns_per_difficulty(Difficulty::Kantan, crown),
            Self::num_crowns_per_difficulty(Difficulty::Futsuu, crown),
            Self::num_crowns_per_difficulty(Difficulty::Muzukashii, crown),
            Self::num_crowns_per_difficulty(Difficulty::Oni, crown),
            measured!(0u32)
        )
    }

    pub fn num_crowns_per_difficulty(difficulty: Difficulty, crown: Result) -> ResolvedChain {
        let map_fn = |s: Chain<MemoryRef>| {
            if crown == Result::SilverCrown {
                s.modulo(Result::GoldenCrown as u32)
            } else {
                s.div(Result::GoldenCrown as u32)
            }
        };

        ALL_SONGS
            .iter()
            .map(|s| add_source!(map_fn(s.best_result(difficulty))))
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rank {
    Apprentice = 0x01,
    Novice = 0x02,
    HalfAMan = 0x03,
    Qualified = 0x04,
    Capable = 0x05,
    GoGetter = 0x06,
    Professional = 0x07,
    Expert = 0x08,
    IronMan = 0x09,
    Master = 0x0a,
    GrandMaster = 0x0b,
    TaikoGod = 0x0c,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Rank::Apprentice => "Apprentice",
            Rank::Novice => "Novice",
            Rank::HalfAMan => "Half a Man",
            Rank::Qualified => "Qualified",
            Rank::Capable => "Capable",
            Rank::GoGetter => "Go-getter",
            Rank::Professional => "Professional",
            Rank::Expert => "Expert",
            Rank::IronMan => "Iron Man",
            Rank::Master => "Master",
            Rank::GrandMaster => "Grand Master",
            Rank::TaikoGod => "Taiko God",
        };
        write!(f, "{s}")
    }
}
