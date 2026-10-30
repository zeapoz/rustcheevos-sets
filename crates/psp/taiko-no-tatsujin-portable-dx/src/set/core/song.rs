use rustcheevos::{
    prelude::*,
    types::{achievement::Achievement, chain::Chain, requirement::Condition},
};

use crate::types::{
    menu::GameMode,
    song::{in_game_stats::InGameStats, result::SongResult},
};

pub fn clear_song_with_all_great(description: &'static str) -> Achievement {
    clear_song_with_all(
        description,
        chain!(InGameStats::good_hits().eq(0), InGameStats::misses().eq(0)),
    )
}

pub fn clear_song_with_all_good(description: &'static str) -> Achievement {
    clear_song_with_all(
        description,
        chain!(InGameStats::great_hits().eq(0), InGameStats::misses().eq(0)),
    )
}

pub fn clear_song_with_all(description: &'static str, prelude: Chain<Condition>) -> Achievement {
    Achievement::builder("TBD - In-Game Achievement")
        .description(description)
        .core(chain!(
            delta!(SongResult::end_result_is(SongResult::Fail)),
            trigger!(SongResult::end_result_is(SongResult::GoldenCrown)),
            prelude,
            InGameStats::pointer_not_null(),
        ))
        .build()
}

pub fn achieve_combo(_description: &'static str, target: u32, mode: GameMode) -> Achievement {
    Achievement::builder("TBD - Combo")
        .core(chain!(
            delta!(InGameStats::highest_combo().lt(target)),
            InGameStats::highest_combo().eq(target),
            GameMode::is(mode),
            InGameStats::pointer_not_null(),
        ))
        .build()
}
