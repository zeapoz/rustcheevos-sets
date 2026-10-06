use rustcheevos::{prelude::*, types::achievement::Achievement};

use crate::{
    mem,
    types::{difficulty::Difficulty, modifier::SpecialModifier, result::Result, song::Song},
};

pub fn clear_song_with_result(
    title: &str,
    song: Song,
    result: Result,
    difficulty: Difficulty,
) -> Achievement {
    let core = chain!(
        delta!(mem::end_result().eq(0x0)),
        mem::end_result().eq(result as u32),
        mem::current_song_id().eq(song as u32),
        Difficulty::is(difficulty),
        SpecialModifier::is_not(SpecialModifier::Auto),
    );

    let insert = if result == Result::SilverCrown {
        "or higher "
    } else {
        ""
    };

    Achievement::builder(title)
        .description(format!(
            "Earn a {result} {insert}on \"{song}\" on {difficulty} difficulty"
        ))
        .points(0)
        .core(core)
        .build()
}
