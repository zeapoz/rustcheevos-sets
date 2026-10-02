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
    types::{result::Result, song::Song},
};

use super::difficulty::Difficulty;

pub const TOTAL_NUM_HEADGEAR: u32 = 29;
pub const TOTAL_NUM_CLOTHES: u32 = 29;
pub const TOTAL_NUM_IN_GAME_ACHIEVEMENTS: u32 = 78;

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

    /// Returns a chain counting the number of obtained headgear.
    pub fn num_headgear() -> Chain<Arithmetic> {
        let num_headgear = Self::count_byte_array_flags(
            mem::file_1_headgear_unlock_flags(),
            TOTAL_NUM_HEADGEAR as usize,
        );
        measured!(num_headgear)
    }

    /// Returns a chain counting the number of obtained clothes.
    pub fn num_clothes() -> Chain<Arithmetic> {
        let num_clothes = Self::count_byte_array_flags(
            mem::file_1_clothes_unlock_flags(),
            TOTAL_NUM_CLOTHES as usize,
        );
        measured!(num_clothes)
    }

    pub fn num_challenges_completed() -> Chain<Arithmetic> {
        let num_challenges = Self::count_byte_array_flags(
            mem::in_game_achievements_unlock_flags(),
            TOTAL_NUM_IN_GAME_ACHIEVEMENTS as usize,
        );
        measured!(num_challenges)
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

    fn num_crowns_per_difficulty(difficulty: Difficulty, crown: Result) -> ResolvedChain {
        let map_fn = |s: Chain<MemoryRef>| {
            if crown == Result::SilverCrown {
                s.modulo(Result::GoldenCrown as u32)
            } else {
                s.div(Result::GoldenCrown as u32)
            }
        };

        Song::ALL
            .iter()
            .map(|s| add_source!(map_fn(s.best_result(difficulty))))
            .collect()
    }
}
