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

pub fn generate_leaderboards() -> Vec<Leaderboard> {
    // Generate leaderboard for all non-dlc songs for all 4 (enum variants) difficulties
    // Song::ALL
    //     .iter()
    //     .filter_map(|s| s.is_dlc().then_some())
    //     .collect()

    Difficulty::ALL
        .iter()
        .rev()
        .flat_map(|&difficulty| {
            // Skip Taiko Guide Chapter 116 as it's not a standard song
            Song::ALL[..Song::ALL.len() - 1]
                .iter()
                .filter_map(move |&s| (!s.is_dlc()).then_some(leaderboard(s, difficulty)))
        })
        .collect()
}

fn leaderboard(song: Song, difficulty: Difficulty) -> Leaderboard {
    Leaderboard::builder(format!("{} - {}", song.title(), difficulty))
        .description(format!(
            "Get the highest score on {} on {} difficulty",
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
