use rustcheevos::{prelude::*, types::achievement::Achievement};

use crate::{
    mem,
    types::{difficulty::Difficulty, modifier::SpecialModifier, result::Result, song::Song},
};

pub fn clear_song_with_result(
    title: &str,
    song: &Song,
    result: Result,
    difficulty: Difficulty,
) -> Achievement {
    let core = chain!(
        delta!(Result::end_result_is(Result::Fail)),
        Result::end_result_is(result),
        mem::current_song_id().eq(*song as u32),
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
