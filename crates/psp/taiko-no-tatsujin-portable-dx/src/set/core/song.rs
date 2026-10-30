use rustcheevos::{
    prelude::*,
    types::{achievement::Achievement, chain::Chain, requirement::Condition},
};

use crate::types::{
    achievement::InGameAchievement,
    menu::{FreeplayState, GeneralMenuState, MenuState},
    song::in_game_stats::InGameStats,
};

pub fn clear_song_with_all_great(description: &'static str) -> Achievement {
    clear_song_with_all(
        description,
        InGameAchievement::ClearASongWithAllGreatHitsAndNoMisses,
        chain!(InGameStats::good_hits().eq(0), InGameStats::misses().eq(0)),
    )
}

pub fn clear_song_with_all_good(description: &'static str) -> Achievement {
    clear_song_with_all(
        description,
        InGameAchievement::ClearASongWithAllGoodHitsAndNoMisses,
        chain!(InGameStats::great_hits().eq(0), InGameStats::misses().eq(0)),
    )
}

fn clear_song_with_all(
    description: &'static str,
    in_game_achievement: InGameAchievement,
    prelude: Chain<Condition>,
) -> Achievement {
    Achievement::builder("TBD - In-Game Achievement")
        .description(description)
        .core(chain!(
            delta!(in_game_achievement.is_locked()),
            in_game_achievement.is_unlocked(),
            MenuState::is(GeneralMenuState::Mail as u32),
        ))
        .alt_group(Condition::always_true())
        .alt_group(chain!(
            trigger!(MenuState::is(FreeplayState::Playing as u32)),
            prelude,
        ))
        .build()
}
