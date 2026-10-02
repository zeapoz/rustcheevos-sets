use rustcheevos::types::achievement::Achievement;

use crate::{
    set::core::songs::clear_song_with_result,
    types::{difficulty::Difficulty, result::Result, song::Song},
};

pub mod songs;

pub fn generate_set() -> Vec<Achievement> {
    vec![
        clear_song_with_result(
            "Test Achievement",
            Song::OneDay,
            Result::SilverCrown,
            Difficulty::Muzukashii,
        ),
        clear_song_with_result(
            "Test Achievement 2",
            Song::OneDay,
            Result::GoldenCrown,
            Difficulty::Muzukashii,
        ),
    ]
}
