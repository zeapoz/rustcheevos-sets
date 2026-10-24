use rustcheevos::prelude::*;
use rustcheevos::types::leaderboard::Leaderboard;

use crate::{
    mem,
    types::{
        difficulty::Difficulty,
        menu::{FreeplayState, MenuState},
        song::{ALL_STANDARD_SONGS, Song},
    },
};

pub fn generate_leaderboards() -> Vec<Leaderboard> {
    Difficulty::ALL
        .iter()
        .rev()
        .flat_map(|&difficulty| {
            ALL_STANDARD_SONGS
                .iter()
                .map(move |song| leaderboard(song, difficulty))
        })
        .collect()
}

fn leaderboard(song: &Song, difficulty: Difficulty) -> Leaderboard {
    Leaderboard::builder(format!("{} - {}", song.title(), difficulty))
        .description(format!(
            "Get the highest score on \"{}\" on {} difficulty",
            song.title(),
            difficulty
        ))
        .start(chain!(
            delta!(MenuState::is(MenuState::Freeplay(FreeplayState::Playing))),
            MenuState::is(MenuState::Freeplay(FreeplayState::Results)),
            mem::current_song_id().eq(song.id()),
            mem::results_score().ne(0)
        ))
        .value(measured!(mem::results_score()))
        .build()
}
