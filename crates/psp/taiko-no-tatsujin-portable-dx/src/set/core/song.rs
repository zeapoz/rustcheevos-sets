use rustcheevos::prelude::*;
use rustcheevos::types::achievement::Achievement;
use rustcheevos::types::requirement::Condition;

use crate::mem;
use crate::types::menu::{FreeplayState, MenuState};
use crate::types::song::in_game_stats::InGameStats;
use crate::{
    set::core::in_game_achievement::InGameAchievementBuilder,
    types::{achievement::InGameAchievement, difficulty::Difficulty, song::Song},
};

pub fn earn_gold_on(
    description: &'static str,
    in_game_achievement: InGameAchievement,
    song: Song,
    difficulty: Difficulty,
) -> Achievement {
    InGameAchievementBuilder::new(description, in_game_achievement)
        .extra_core(chain!(
            and_next!(InGameStats::misses().ne(0)),
            reset_if!(MenuState::is(FreeplayState::Playing as u32)),
            reset_if!(MenuState::is(FreeplayState::SongSelect as u32)),
        ))
        .alt_group(chain!(
            and_next!(mem::current_song_id().eq(song.id())),
            and_next!(Difficulty::is(difficulty)),
            and_next!(delta!(InGameStats::pointer_is_null())),
            InGameStats::pointer_not_null().with_hits(1),
        ))
        .alt_group(chain!(
            and_next!(mem::current_song_id().eq(song.id())),
            and_next!(Difficulty::is(difficulty)),
            and_next!(InGameStats::pointer_not_null()),
            and_next!(delta!(InGameStats::passed_notes()).ne(0)),
            InGameStats::passed_notes().eq(0).with_hits(1),
        ))
        .trigger()
        .build()
}

pub fn donderful_combo(
    description: &'static str,
    in_game_achievement: InGameAchievement,
) -> Achievement {
    trigger_challenge_achievement(
        description,
        in_game_achievement,
        chain!(
            or_next!(InGameStats::misses().ne(0)),
            and_next!(InGameStats::good_hits().ne(0)),
            reset_if!(MenuState::is(FreeplayState::Playing as u32)),
        ),
    )
}

pub fn false_donderful_combo(
    description: &'static str,
    in_game_achievement: InGameAchievement,
) -> Achievement {
    trigger_challenge_achievement(
        description,
        in_game_achievement,
        chain!(
            or_next!(InGameStats::misses().ne(0)),
            and_next!(InGameStats::great_hits().ne(0)),
            reset_if!(MenuState::is(FreeplayState::Playing as u32)),
        ),
    )
}

fn trigger_challenge_achievement(
    description: &'static str,
    in_game_achievement: InGameAchievement,
    restriction: Chain<Condition>,
) -> Achievement {
    InGameAchievementBuilder::new(description, in_game_achievement)
        .extra_core(chain!(
            and_next!(delta!(InGameStats::pointer_is_null())),
            InGameStats::pointer_not_null().with_hits(1),
            restriction,
            reset_if!(MenuState::is(FreeplayState::SongSelect as u32)),
        ))
        .trigger()
        .build()
}
