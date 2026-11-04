use rustcheevos::{prelude::*, types::achievement::Achievement};

use crate::{
    mem,
    types::{
        difficulty::Difficulty,
        modifier::SpecialModifier,
        song::{Song, in_game_stats::InGameStats, result::SongResult},
    },
};

pub fn clear_song_with_result(
    title: &str,
    song: &Song,
    result: SongResult,
    difficulty: Difficulty,
) -> Achievement {
    let core = chain!(
        trigger!(delta!(SongResult::end_result_is(SongResult::Fail))),
        trigger!(SongResult::end_result_is(result)),
        InGameStats::misses().eq(0),
        mem::current_song_id().eq(song.id()),
        Difficulty::is(difficulty),
        SpecialModifier::is_not(SpecialModifier::Auto),
        InGameStats::pointer_not_null(),
    );

    let insert = if result == SongResult::SilverCrown {
        "or higher "
    } else {
        ""
    };

    Achievement::builder(title)
        .description(format!(
            "Earn a {result} {insert}on \"{song}\" on {difficulty} difficulty"
        ))
        .core(core)
        .points(20)
        .build()
}
