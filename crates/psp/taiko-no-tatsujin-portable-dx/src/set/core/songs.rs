use rustcheevos::{prelude::*, types::achievement::Achievement};

use crate::types::{difficulty::Difficulty, profile::Profile, result::Result, song::Song};

pub fn clear_song_with_result(
    title: &str,
    song: Song,
    result: Result,
    difficulty: Difficulty,
) -> Achievement {
    let cond = match result {
        Result::Fail => unreachable!(),
        Result::SilverCrown => song.best_result(difficulty).ge(result as u32),
        Result::GoldenCrown => song.best_result(difficulty).eq(result as u32),
    };
    let core = chain!(
        delta!(song.best_result(difficulty).lt(result as u32)),
        cond,
        Profile::is_loaded(),
    );

    Achievement::builder(title)
        .description(format!(
            "Earn a {result} on \\\"{song}\\\" on {difficulty} difficulty"
        ))
        .points(0)
        .core(core)
        .build()
}
