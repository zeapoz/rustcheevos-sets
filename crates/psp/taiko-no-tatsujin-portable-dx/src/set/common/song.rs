use rustcheevos::{prelude::*, types::achievement::Achievement};

use crate::{
    mem,
    types::{
        difficulty::Difficulty,
        modifier::SpecialModifier,
        song::{Song, result::SongResult},
    },
};

pub fn clear_song_with_result(
    title: &str,
    song: &Song,
    result: SongResult,
    difficulty: Difficulty,
) -> Achievement {
    let core = chain!(
        delta!(SongResult::end_result_is(SongResult::Fail)),
        SongResult::end_result_is(result),
        mem::current_song_id().eq(*song as u32),
        Difficulty::is(difficulty),
        SpecialModifier::is_not(SpecialModifier::Auto),
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
        .build()
}
