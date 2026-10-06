use rustcheevos::types::achievement::Achievement;

use crate::{
    set::core::omikoshi::{
        control_dojo, gather_drum_disciples, unlock_all_clothes, unlock_all_headgear,
    },
    types::sounds::Sound,
    util::TOTAL_NUM_DOJO,
};

pub mod omikoshi;
pub mod songs;

#[rustfmt::skip]
pub fn generate_set() -> Vec<Achievement> {
    vec![
        // clear_song_with_result("Test Achievement", Song::OneDay, Result::SilverCrown, Difficulty::Muzukashii),
        // clear_song_with_result("Test Achievement 2", Song::OneDay, Result::GoldenCrown, Difficulty::Muzukashii),

        // Omikoshi
        control_dojo(10),
        control_dojo(20),
        control_dojo(30),
        control_dojo(40),
        control_dojo(TOTAL_NUM_DOJO),
        gather_drum_disciples(100, Sound::ChibiDon),
        gather_drum_disciples(200, Sound::GangsterDon),
        gather_drum_disciples(300, Sound::YokozunaDon),
        unlock_all_headgear(),
        unlock_all_clothes(),
    ]
}
