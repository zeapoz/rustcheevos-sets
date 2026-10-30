use rustcheevos::prelude::*;
use rustcheevos::types::leaderboard::Leaderboard;

use crate::{
    mem,
    types::{
        difficulty::Difficulty,
        menu::{FreeplayState, MenuState},
        song::Song,
    },
};

pub fn generate_leaderboards(songs: &[&'static Song]) -> Vec<Leaderboard> {
    Difficulty::ALL
        .iter()
        .rev()
        .flat_map(|&difficulty| {
            songs
                .iter()
                .map(move |song| new_leaderboard(song, difficulty))
        })
        .collect()
}

pub fn new_leaderboard(song: &Song, difficulty: Difficulty) -> Leaderboard {
    Leaderboard::builder(format!("\"{}\" - {}", song.title(), difficulty))
        .description(format!(
            "Get the highest score on \"{}\" on {} difficulty!",
            song.title(),
            difficulty
        ))
        .start(chain!(
            delta!(MenuState::is(FreeplayState::Playing as u32)),
            MenuState::is(FreeplayState::Results as u32),
            mem::current_song_id().eq(song.id()),
            mem::results_score().ne(0)
        ))
        .value(measured!(mem::results_score()))
        .build()
}
