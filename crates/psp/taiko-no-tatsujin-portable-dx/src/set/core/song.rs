use rustcheevos::prelude::*;
use rustcheevos::types::achievement::Achievement;
use rustcheevos::types::requirement::Condition;

use crate::mem;
use crate::types::menu::{FreeplayState, GameMode, MenuState};
use crate::types::modifier::SpecialModifier;
use crate::types::song::in_game_stats::InGameStats;
use crate::types::song::result::SongResult;
use crate::{
    set::core::in_game_achievement::InGameAchievementBuilder,
    types::{achievement::InGameAchievement, difficulty::Difficulty, song::Song},
};

pub fn earn_gold_on(
    title: &str,
    points: u32,
    in_game_achievement: InGameAchievement,
    song: Song,
    difficulty: Difficulty,
    unlock_item: Option<&str>,
) -> Achievement {
    let description = if let Some(item) = unlock_item {
        format!(
            "Earn a {} on \"{song}\" on {difficulty} difficulty and obtain the {item}",
            SongResult::GoldenCrown
        )
    } else {
        format!(
            "Earn a {} on \"{song}\" on {difficulty} difficulty",
            SongResult::GoldenCrown
        )
    };

    InGameAchievementBuilder::new(title, &description, points, in_game_achievement)
        .extra_core(chain!(
            and_next!(InGameStats::misses().ne(0)),
            reset_if!(MenuState::is(FreeplayState::Playing as u32)),
            reset_if!(MenuState::is(FreeplayState::SongSelect as u32)),
            reset_if!(GameMode::is_not(GameMode::Freeplay)),
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

pub fn clear_song_with_special_modifier(
    title: &str,
    description: &str,
    points: u32,
    modifier: SpecialModifier,
) -> Achievement {
    Achievement::builder(title)
        .description(description)
        .core(chain!(
            trigger!(delta!(SongResult::end_result_is(SongResult::Fail))),
            trigger!(SongResult::end_result_is_not(SongResult::Fail)),
            SpecialModifier::is(modifier),
            InGameStats::pointer_not_null(),
            GameMode::is(GameMode::Freeplay),
        ))
        .points(points)
        .build()
}

pub fn clear_song_with_modifier_active(
    title: &str,
    description: &str,
    points: u32,
    modifier_active: Chain<Condition>,
) -> Achievement {
    Achievement::builder(title)
        .description(description)
        .core(chain!(
            trigger!(delta!(SongResult::end_result_is(SongResult::Fail))),
            trigger!(SongResult::end_result_is_not(SongResult::Fail)),
            modifier_active,
            SpecialModifier::is_not(SpecialModifier::Auto),
            InGameStats::pointer_not_null(),
            GameMode::is(GameMode::Freeplay),
        ))
        .points(points)
        .build()
}

pub fn donderful_combo(
    title: &str,
    description: &str,
    points: u32,
    in_game_achievement: InGameAchievement,
) -> Achievement {
    trigger_challenge_achievement(
        title,
        description,
        points,
        in_game_achievement,
        chain!(
            or_next!(InGameStats::misses().ne(0)),
            and_next!(InGameStats::good_hits().ne(0)),
            reset_if!(MenuState::is(FreeplayState::Playing as u32)),
        ),
    )
}

pub fn false_donderful_combo(
    title: &str,
    description: &str,
    points: u32,
    in_game_achievement: InGameAchievement,
) -> Achievement {
    trigger_challenge_achievement(
        title,
        description,
        points,
        in_game_achievement,
        chain!(
            or_next!(InGameStats::misses().ne(0)),
            and_next!(InGameStats::great_hits().ne(0)),
            reset_if!(MenuState::is(FreeplayState::Playing as u32)),
        ),
    )
}

fn trigger_challenge_achievement(
    title: &str,
    description: &str,
    points: u32,
    in_game_achievement: InGameAchievement,
    restriction: Chain<Condition>,
) -> Achievement {
    InGameAchievementBuilder::new(title, description, points, in_game_achievement)
        .extra_core(chain!(
            restriction,
            reset_if!(MenuState::is(FreeplayState::SongSelect as u32)),
            GameMode::is(GameMode::Freeplay),
        ))
        .alt_group(chain!(
            and_next!(delta!(InGameStats::pointer_is_null())),
            and_next!(InGameStats::pointer_not_null()),
        ))
        .alt_group(chain!(
            and_next!(InGameStats::pointer_not_null()),
            and_next!(delta!(InGameStats::passed_notes()).ne(0)),
            InGameStats::passed_notes().eq(0).with_hits(1),
        ))
        .trigger()
        .build()
}
